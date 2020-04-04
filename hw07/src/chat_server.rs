use std::{
    collections::HashMap,
    net::SocketAddr,
    sync::{Arc, Mutex},
};

use futures_util::{future, pin_mut, stream::TryStreamExt, StreamExt};
use rustc_serialize::json;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender};
use tokio_tungstenite::accept_async;
use tungstenite::Message;

type Tx = UnboundedSender<Message>;
type PeerMap = Arc<Mutex<HashMap<SocketAddr, Tx>>>;

use crate::utils::Result;

const WS_ADDR: &'static str = "0.0.0.0:1981";

#[derive(Debug, Clone, RustcDecodable, RustcEncodable)]
/// Represents a single, atomic action taken by a chat member.
///
/// DO NOT MODIFY: the JavaScript relies on this!
enum ChatAction {
    Connect { addr: String },
    Disconnect { addr: String },
    Msg { user: String, text: String },
}

/// Create the relay MPSC (multi-producer/single-consumer) channel, spawn the
/// relay thread, then listen for WebSocket clients and spawn their threads.
pub async fn start() -> Result<()> {
    let mut listener = TcpListener::bind(&WS_ADDR).await.expect("Can't listen");
    println!("Listening on {}", WS_ADDR);

    let peer_map = PeerMap::new(Mutex::new(HashMap::new()));

    while let Ok((stream, _)) = listener.accept().await {
        let peer = stream
            .peer_addr()
            .expect("connected streams should have a peer address");
        println!("Peer address: {}", peer);

        let new_peer_map = peer_map.clone();

        tokio::spawn(async move {
            relay_task(new_peer_map, peer, stream)
                .await
                .expect("Error executing relay_task in Chat Server")
        });
    }

    Ok(())
}

/// The relay task handles all `ChatAction`s received on its MPSC channel
/// by sending them out to all of the currently connected clients.
async fn relay_task(peer_map: PeerMap, peer: SocketAddr, stream: TcpStream) -> Result<()> {
    let ws_stream = accept_async(stream).await.expect("Failed to accept");

    let (in_tx, in_rx) = unbounded_channel();
    let (out_tx, out_rx) = unbounded_channel();

    println!("New WebSocket connection: {}", peer);

    peer_map.lock().unwrap().insert(peer, out_tx);

    tokio::spawn(client_task(peer, in_rx, peer_map));

    let (outgoing, incoming) = ws_stream.split();

    {
        let connection_msg = json::encode(&ChatAction::Connect {
            addr: peer.to_string(),
        })?;
        in_tx.send(connection_msg)?;
    }

    let broadcast_incoming = incoming.try_for_each(|msg| async {
        if let Message::Text(txt) = msg {
            in_tx.send(txt).or(Ok(()))
        } else {
            Ok(())
        }
    });

    let receive_from_others = out_rx.map(Ok).forward(outgoing);

    pin_mut!(broadcast_incoming, receive_from_others);
    future::select(broadcast_incoming, receive_from_others).await;

    {
        let connection_msg = json::encode(&ChatAction::Disconnect {
            addr: peer.to_string(),
        })?;
        in_tx.send(connection_msg)?;
    }

    Ok(())
}

/// Each client task waits for input (or disconnects) from its respective clients
/// and relays the appropriate messages via the relay MPSC channel.
///
/// The messages received-from and sent-to the client should be JSON objects with the same
/// form as rustc_serialize's serialization of the `ChatAction` type.
///
/// * If the client connects, a `ChatAction::Connect` will be relayed with their IP address.
///
/// * If the client disconnects, a `ChatAction::Disconnect` will be relayed with their IP address.
///
/// * If the client sends any other message (i.e. `ChatAction::Msg`), it will be relayed verbatim.
///   (But you should still deserialize and reserialize the `ChatAction` to make sure it is valid!)
async fn client_task(
    current_peer: SocketAddr,
    mut rx: UnboundedReceiver<String>,
    peer_map: PeerMap,
) -> Result<()> {
    while let Some(msg) = rx.recv().await {
        let chat_msg: ChatAction = json::decode(&msg)?;

        let mut disconnect = false;

        match chat_msg {
            ChatAction::Disconnect { addr: _ } => {
                disconnect = true;
            }
            _ => {}
        }

        if disconnect {
            println!("Disconnect {}", current_peer);

            peer_map.lock().unwrap().remove(&current_peer);
        }
        let peers = peer_map.lock().unwrap();

        for (peer, ws_sink) in peers.iter() {
            println!("Sending msg {} to: {}", msg, peer);
            ws_sink.send(Message::Text(msg.clone()))?;
        }
    }

    Ok(())
}
