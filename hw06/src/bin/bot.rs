extern crate bbs;
extern crate hyper;
extern crate hyper_tls;
extern crate regex;
extern crate rustc_serialize;
extern crate tokio;

use hyper::body::HttpBody as _;
use hyper::{Body, Client, Uri};
use hyper_tls::HttpsConnector;

use std::io::Read;
use std::net::{TcpListener, TcpStream};

use bbs::utils::Result;
use bbs::UserClient;
use bbs::{BOT_ADDR, HTTP_ADDR};

const BOT_USER: &'static str = "Mr Bot";
const RANDOM_NUMBER_URL: &'static str =
    "https://www.random.org/integers/?num=1&min=1&max=3&col=1&base=10&format=plain&rnd=new";
const BOT_QUESTION_RE: &'static str = r".+[Bb]ot.+choose (.+), (.+), or (.+)";

async fn get_content(url: String) -> Result<String> {
    let uri = url.parse::<Uri>()?;

    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, Body>(https);

    let req = hyper::Request::builder()
        .method(hyper::Method::GET)
        .uri(uri)
        .header("content-type", "application/json")
        .body(hyper::Body::from(""))?;

    let mut res = client.request(req).await?;

    while let Some(Ok(body)) = res.body_mut().data().await {
        return Ok(std::str::from_utf8(&body).unwrap().to_string());
    }

    Err(From::from("Unexpected".to_string()))
}

async fn get_random_number() -> Result<i32> {
    let response = get_content(RANDOM_NUMBER_URL.to_string()).await?;
    response.trim().parse::<i32>().map_err(|e| e.into())
}

async fn handle_message(mut stream: TcpStream) -> Result<()> {
    let question_regex = regex::Regex::new(BOT_QUESTION_RE)?;

    let mut buf = vec![];
    stream.read_to_end(&mut buf)?;
    let request = String::from_utf8(buf)?;

    println!("Received text: {}", request);

    if let Some(capture) = question_regex.captures(&request) {
        if let Some(response) = capture.get(get_random_number().await? as usize) {
            let user_client = UserClient::new(HTTP_ADDR.to_string());
            user_client
                .send_msg(BOT_USER.to_string(), response.as_str().to_string())
                .await?;
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let listener = TcpListener::bind(BOT_ADDR).unwrap();

    loop {
        for stream in listener.incoming() {
            match stream {
                Ok(stream) => match handle_message(stream).await {
                    Ok(_) => {}
                    Err(e) => println!("Error in bot: {}", e),
                },
                Err(e) => println!("Error in bot: {}", e),
            }
        }
    }
}
