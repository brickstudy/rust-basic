use axum::{
    Router,
    routing::{get},
};
use std::net::SocketAddr;

async fn hello_world() -> &'static str {
    "Hello, world!!"
}

pub async fn run() {
    let app = Router::new()
        .route("/", get(hello_world));

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("Server run: http://{}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
