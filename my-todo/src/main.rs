use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use std::env;

#[tokio::main]
async fn main() {
    //loggingの初期化
    let log_level = env::var("RUST_LOG").unwrap_or("info".to_string());
    env::set_var("RUST_LOG", log_level);
    tracing_subscriber::fmt::init();
    // ルートを定義する
    let app = Router::new()
        .route("/", get(root))
        .route("/users", post(create_user));
    // ルートをバインドしてサーバーを起動する
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    
    tracing::debug!("listening on {}", addr);

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

async fn create_user(
    //ここでDeserializeを実装した構造体を受け取る
    Json(payload): Json<CreateUser>,
) -> impl IntoResponse {
    let user = User {
        id: 1337,
        username: payload.username,
    };

    (StatusCode::CREATED, Json(user))
}

//current_user関数と関連する構造体
#[derive(Deserialize)]
struct CreateUser {
    username: String,
}

#[derive(Serialize)]
struct User {
    id: u64,
    username: String,
}