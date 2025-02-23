mod block;
mod chain;
mod constants;
mod server;

use std::{
    convert::Infallible,
    net::{SocketAddr, TcpListener, TcpStream},
    result,
};

use block::Block;
use chain::Chain;
use constants::{HTTP_PORT, SOCKET_PORT};
use futures::{SinkExt, StreamExt};
use hyper::{Body, Response, Server};
use routerify::{Router, RouterService};
use routerify_websocket::{upgrade_ws, Message, WebSocket};
use server::{HttpServer, P2PServer};

// fn main() -> std::io::Result<()> {
//     let mut chain = Chain::new().unwrap();

//     let listener = TcpListener::bind("127.0.0.1:80")?;

//     for stream in listener.incoming() {
//         handle_client(stream?);
//     }

//     Ok(())

//     // for x in 1..10 {
//     //     chain.add_block(&vec![format!("data-{x}").to_string()]);
//     // }

//     // print!("{:#?}", chain);
// }

async fn ws_handler(ws: WebSocket) {
    println!("New websocket connection: {}", ws.remote_addr());
    connect_socket(ws);
}

async fn connect_socket(ws: WebSocket) {
    // 참조를 sockets 에 저

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

// tokio : async 런타임 라이브러리
#[tokio::main]
async fn main() {
    tokio::spawn(async move {
        match hyper::upgrade::on(req).await {
            Ok(upgraded) => {
                ws_handler(WebSocket::from_raw_socket(upgraded, remote_addr, config).await).await;
            }
            Err(err) => log::error!("{}", crate::WebsocketError::Upgrade(err.into())),
        }
    });

    let http_server = HttpServer::new();
    let p2p_server = P2PServer::new();

    let ((), ()) = futures::join!(
        http_server.listen(HTTP_PORT),
        p2p_server.listen(SOCKET_PORT)
    );

    println!("{:#?}", P2PServer::sockets);
}
