use std::convert::Infallible;
use std::fs::OpenOptions;
use std::io::{self, BufReader, Read};
use std::path::Path;

use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Method, Request, Response, Server, StatusCode};

use crate::utils::Result;

const HTTP_ADDR: &'static str = "0.0.0.0:1980";
const HTML_DATA: &'static str = "html/index.html";

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

async fn req_handler(req: Request<Body>) -> Result<Response<Body>> {
    match (req.method(), req.uri().path()) {
        (&Method::GET, _) => {
            let mut buf = String::new();

            if Path::new(HTML_DATA).exists() {
                try_or_server_err!(read_file(HTML_DATA, &mut buf));
            }

            // And return buf as the response.
            Ok(Response::builder()
                .status(StatusCode::OK)
                .body(buf.into())
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

pub async fn start() -> Result<()> {
    let addr = HTTP_ADDR.parse::<std::net::SocketAddr>().unwrap();

    let make_svc = make_service_fn(|_conn| {
        // This is the `Service` that will handle the connection.
        // `service_fn` is a helper to convert a function that
        // returns a Response into a `Service`.
        async { Ok::<_, Infallible>(service_fn(req_handler)) }
    });

    let server = Server::bind(&addr).serve(make_svc);

    println!("Listening on {}", HTTP_ADDR);

    server.await?;

    Ok(())
}
