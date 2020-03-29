extern crate bbs;
extern crate chrono;
extern crate hyper;
extern crate rustc_serialize;
extern crate tokio;

use bbs::utils::Result;
use bbs::Message;
use bbs::{BOT_ADDR, HTML_FOOTER, HTML_HEADER, JSON_DATA, SERVER_ADDR};
use bytes::buf::BufExt;
use chrono::{DateTime, Utc};
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Method, Request, Response, Server, StatusCode};
use rustc_serialize::json;
use std::convert::Infallible;
use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use std::io::{self, BufReader, Read, Write};
use std::net::TcpStream;
use std::path::Path;

macro_rules! HTML_MESSAGE_FORMAT {
    () => {
        r###"
<article class="POST">
  <header>
    <h2>{text}</h2>
  </header>
  <p>
    Posted on <time datetime="{date}">{date}</time> by <b>{user}</b>.
  </p>
</article>"###
    };
}

// Returns val from Ok(val) or sets the response to return an InternalServerError.
macro_rules! try_or_server_err {
    ($expr:expr) => {
        match $expr {
            Ok(val) => val,
            Err(err) => {
                println!("Server error: {}", err);

                return Ok(Response::builder()
                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                    .body(format!("INTERNAL_SERVER_ERROR: {}", err).into())
                    .unwrap());
            }
        }
    };
}

fn read_file<P>(filename: P, buf: &mut String) -> io::Result<usize>
where
    P: AsRef<Path>,
{
    let file = OpenOptions::new().read(true).open(filename)?;
    BufReader::new(file).read_to_string(buf)
}

fn read_file_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = OpenOptions::new().read(true).open(filename)?;
    Ok(BufReader::new(file).lines())
}

fn write_file<P>(filename: P, text: &String) -> io::Result<()>
where
    P: AsRef<Path>,
{
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(filename)?;
    file.write_all(text.as_bytes())
}

async fn req_handler(req: Request<Body>) -> Result<Response<Body>> {
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/json") => {
            let mut buf = String::new();

            if Path::new(JSON_DATA).exists() {
                try_or_server_err!(read_file(JSON_DATA, &mut buf));
            }

            // And return buf as the response.
            Ok(Response::builder()
                .status(StatusCode::OK)
                .body(buf.into())
                .unwrap())
        }
        (&Method::GET, _) => {
            let mut buf = String::new();

            try_or_server_err!(read_file(HTML_HEADER, &mut buf));

            if Path::new(JSON_DATA).exists() {
                let messages = try_or_server_err!(read_file_lines(JSON_DATA));
                for m in messages {
                    let message: Message = try_or_server_err!(json::decode(&m.unwrap()));
                    let message_html = format!(
                        HTML_MESSAGE_FORMAT!(),
                        text = message.text,
                        user = message.user,
                        date = message.date.unwrap_or_default()
                    );
                    buf.extend(message_html.chars());
                }
            }

            try_or_server_err!(read_file(HTML_FOOTER, &mut buf));

            // And return buf as the response.
            Ok(Response::builder()
                .status(StatusCode::OK)
                .body(buf.into())
                .unwrap())
        }
        (&Method::POST, _) => {
            let whole_body = try_or_server_err!(hyper::body::aggregate(req).await);

            let mut buf = String::new();

            try_or_server_err!(whole_body.reader().read_to_string(&mut buf));

            println!("Received POST: {}", buf);

            let mut message: Message = try_or_server_err!(json::decode(&buf));

            let now: DateTime<Utc> = Utc::now();
            message.date = Some(now.format("%a %b %e %T").to_string());

            let new_json = format!("{}\n", try_or_server_err!(json::encode(&message)));

            try_or_server_err!(write_file(JSON_DATA, &new_json));

            if let Ok(mut stream) = TcpStream::connect(BOT_ADDR){
                stream.write_all(&message.text.as_bytes())?;
            }

            Ok(Response::builder()
                .status(StatusCode::OK)
                .body(Body::empty())
                .unwrap())
        }
        _ => {
            // Return 404 not found response.
            Ok(Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body("NOT_FOUND".into())
                .unwrap())
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    println!("Listening on {}.", SERVER_ADDR);
    let addr = SERVER_ADDR.parse::<std::net::SocketAddr>().unwrap();

    let make_svc = make_service_fn(|_conn| {
        // This is the `Service` that will handle the connection.
        // `service_fn` is a helper to convert a function that
        // returns a Response into a `Service`.
        async { Ok::<_, Infallible>(service_fn(req_handler)) }
    });

    let server = Server::bind(&addr).serve(make_svc);

    println!("Listening on http://{}", addr);

    server.await?;

    Ok(())
}
