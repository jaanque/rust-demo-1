use axum::{routing::get, Router};
use std::net::SocketAddr;

mod handlers;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(handlers::root));

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("🚀 Servidor Rust escuchando en http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}