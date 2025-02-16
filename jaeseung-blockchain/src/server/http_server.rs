use std::{convert::Infallible, net::SocketAddr};

use futures::{SinkExt, StreamExt};
use hyper::{Body, Response, Server};
use routerify::{Router, RouterService};
use routerify_websocket::{upgrade_ws, Message, WebSocket};

pub struct HttpServer {}

impl HttpServer {
    pub fn new() -> HttpServer {
        HttpServer {}
    }

    pub async fn listen(&self, port: u16) {
        let router: Router<Body, Infallible> = Router::builder()
            .get("/", |_req| async move {
                Ok(Response::new("I also serve http requests".into()))
            })
            .build()
            .unwrap();

        let service = RouterService::new(router).unwrap();

        let addr = SocketAddr::from(([127, 0, 0, 1], port));

        let server = Server::bind(&addr).serve(service);

        if let Err(err) = server.await {
            eprintln!("Http Server error: {}", err);
        }
    }
}
