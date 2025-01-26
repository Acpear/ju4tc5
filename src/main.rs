use axum::{serve::serve, routing::{get, Router}};
use tokio::net::TcpListener;
mod api;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/gen_key", get(api::gen_key));
    let listener = TcpListener::bind("127.0.0.1:8000").await.expect("Failed to bind port");
    serve(listener, app).await.unwrap();
}
