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

// tokio : async 런타임 라이브러리
#[tokio::main]
async fn main() {
    let http_server = HttpServer::new();
    let p2p_server = P2PServer::new();

    let ((), ()) = futures::join!(
        http_server.listen(HTTP_PORT),
        p2p_server.listen(SOCKET_PORT)
    );
}
