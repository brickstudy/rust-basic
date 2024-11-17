mod auth;
mod app;

#[tokio::main]
async fn main() {
    // cli 실행
    // auth::run();

    // api 실행
    app::run().await;
}
