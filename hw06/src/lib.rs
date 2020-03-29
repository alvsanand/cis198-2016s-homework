extern crate hyper;
extern crate rustc_serialize;

use hyper::body::HttpBody as _;
use rustc_serialize::json;
use utils::*;

pub mod utils;

pub const SERVER_ADDR: &'static str = "127.0.0.1:1980";
pub const BOT_ADDR: &'static str = "127.0.0.1:1981";
pub const HTTP_ADDR: &'static str = "http://127.0.0.1:1980";
pub const JSON_ADDR: &'static str = "http://127.0.0.1:1980/json";

pub const JSON_DATA: &'static str = "data/index.json";
pub const HTML_HEADER: &'static str = "html/header.html";
pub const HTML_FOOTER: &'static str = "html/footer.html";

#[derive(Debug, RustcDecodable, RustcEncodable)]
pub struct Message {
    pub user: String,
    pub text: String,
    pub date: Option<String>,
}

impl Message {
    pub fn new(user: String, text: String) -> Message {
        Message {
            text: text,
            user: user,
            date: None,
        }
    }
}

pub struct UserClient {
    server_addr: String,
    client: hyper::Client<hyper::client::connect::HttpConnector, hyper::Body>,
}

impl UserClient {
    pub fn new(server_addr: String) -> UserClient {
        UserClient {
            server_addr: server_addr,
            client: Default::default(),
        }
    }

    pub async fn send_msg(&self, user: String, text: String) -> Result<hyper::StatusCode> {
        let uri = (self.server_addr).parse::<hyper::Uri>()?;

        let message = Message::new(user.clone(), text);

        let json_message = json::encode(&message).unwrap();

        let req = hyper::Request::builder()
            .method(hyper::Method::POST)
            .uri(uri)
            .header("content-type", "application/json")
            .body(hyper::Body::from(json_message))?;

        let res = self.client.request(req).await?;

        Ok(res.status())
    }

    pub async fn get_content(&self) -> Result<(hyper::StatusCode, String)> {
        let uri = self.server_addr.parse::<hyper::Uri>()?;

        let req = hyper::Request::builder()
            .method(hyper::Method::GET)
            .uri(uri)
            .header("content-type", "application/json")
            .body(hyper::Body::from(""))?;

        let mut res = self.client.request(req).await?;

        while let Some(Ok(body)) = res.body_mut().data().await {
            return Ok((
                res.status(),
                std::str::from_utf8(&body).unwrap().to_string(),
            ));
        }

        Err(From::from("Unexpected".to_string()))
    }
}
