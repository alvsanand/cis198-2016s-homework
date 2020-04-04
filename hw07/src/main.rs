pub extern crate futures;
pub extern crate futures_util;
pub extern crate hyper;
pub extern crate rustc_serialize;
pub extern crate tokio;
pub extern crate tokio_tungstenite;
pub extern crate tungstenite;

mod chat_server;
mod utils;
mod webpage_server;

use crate::utils::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let chat_server_task = tokio::spawn(async move { chat_server::start().await });
    let webpage_server_task = tokio::spawn(async move { webpage_server::start().await });

    futures::future::join_all(vec![chat_server_task, webpage_server_task]).await;

    Ok(())
}
