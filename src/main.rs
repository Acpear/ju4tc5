use axum::{
    routing::{get, put, Router},
    serve::serve,
};
use tokio::net::TcpListener;
mod api;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/gen_key", get(api::gen_key))
        .route("/put_key", put(api::put_key));
    let listener = TcpListener::bind("127.0.0.1:8000")
        .await
        .expect("8000 端口绑定不上喵");
    serve(listener, app).await.unwrap();
}
