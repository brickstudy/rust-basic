use std::{convert::Infallible, net::SocketAddr};

use futures::{SinkExt, StreamExt};
use hyper::{Body, Server};
use routerify::{Router, RouterService};
use routerify_websocket::{upgrade_ws, Message, WebSocket};

pub struct P2PServer {
    sockets: Vec<WebSocket>,
}

impl P2PServer {
    pub fn new() -> P2PServer {
        P2PServer { sockets: vec![] }
    }

    pub async fn listen(&self, port: u16) {
        let router: Router<Body, Infallible> = Router::builder()
            .any_method("/", upgrade_ws(Self::ws_handler))
            .build()
            .unwrap();

        let service = RouterService::new(router).unwrap();

        let addr = SocketAddr::from(([127, 0, 0, 1], port));

        let server = Server::bind(&addr).serve(service);

        if let Err(err) = server.await {
            eprintln!("Socket Server error: {}", err);
        }
    }

    async fn ws_handler(ws: WebSocket) {
        println!("New websocket connection: {}", ws.remote_addr());

        // The `WebSocket` implements the `Sink` and `Stream` traits
        // to read and write messages.
        let (mut tx, mut rx) = ws.split();

        // Read messages.
        while let Some(msg) = rx.next().await {
            let msg = msg.unwrap();

            // Check message type and take appropriate actions.
            if msg.is_text() {
                println!("{}", msg.into_text().unwrap());
            } else if msg.is_binary() {
                println!("{:?}", msg.into_bytes());
            }

            // Send a text message.
            let send_msg = Message::text("Hello world");
            tx.send(send_msg).await.unwrap();
        }
    }
}
