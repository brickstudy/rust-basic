use std::{convert::Infallible, net::SocketAddr, sync::OnceLock};

use futures::{SinkExt, StreamExt};
use hyper::{upgrade, Body, Request, Response, Server};
use routerify::{Router, RouterService};
use routerify_websocket::{upgrade_ws, upgrade_ws_with_config, Message, WebSocket};

pub struct P2PServer {}

impl P2PServer {
    pub const sockets: Vec<&WebSocket> = vec![];

    // P2PServer 는 하나의 인스턴스만 존재한다.
    pub fn new() -> &'static P2PServer {
        static INSTANCE: OnceLock<P2PServer> = OnceLock::new();
        INSTANCE.get_or_init(|| P2PServer {})
    }

    pub async fn listen(&'static self, port: u16) {
        let router: Router<Body, Infallible> = Router::builder()
            .any_method("/", upgrade_ws(|ws| self.connect_socket(ws)))
            // upgrade_ws 의 handler 는 static 한 무언가를 받는 것 같다.
            .post("/addToPeer", move |req| async move {
                let response = self.connect_to_peer(req);
                Ok(response)
            })
            .build()
            .unwrap();

        let service = RouterService::new(router).unwrap();

        let addr = SocketAddr::from(([127, 0, 0, 1], port));

        let server = Server::bind(&addr).serve(service);

        if let Err(err) = server.await {
            eprintln!("Socket Server error: {}", err);
        }
    }

    fn connect_to_peer(&'static self, req: Request<Body>) {
        return Response::new("I also serve http requests".into());
    }

    async fn connect_socket(&'static self, ws: WebSocket) {
        println!("New websocket connection: {}", ws.remote_addr());

        // 참조를 sockets 에 저장.
        Self::sockets.push(&ws);

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
            let send_msg = Message::text("msg from server");
            tx.send(send_msg).await.unwrap();
        }
    }
}
