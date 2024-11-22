use axum::{routing::get, Router};
use std::fs::create_dir_all;
use tokio::net::TcpListener;

mod parse;
mod serve;

use parse::*;
use serve::*;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let output_dir = "_site";
    create_dir_all(output_dir).expect("Failed to create _site directory");

    process_index("./markdown");
    let router = Router::new()
        .route("/", get(serve_index))
        .route("/index.html", get(serve_index))
        .route("/*file", get(serve_static));

    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    println!("Listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, router).await.unwrap();
}
