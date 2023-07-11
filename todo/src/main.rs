use axum::{routing::get, Router};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    // 単一のルートでアプリケーションを構築する
    let app = Router::new().route("/", get(root));
    // ルートをバインドしてサーバーを起動する
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    // `app` は `MakeService` トレイトを実装しているので、
    // `axum::Server` に渡すことができる
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// ルートハンドラーは `async fn` でなければならない
async fn root() -> &'static str {
    "Hello, world!"
}