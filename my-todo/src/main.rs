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
    let app = create_app();
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

// テスト対象を切り出す
fn create_app() -> Router {
    Router::new()
        .route("/", get(root))
        .route("/users", post(create_user))
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
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)] //テストで必要なSerializeとDeserializeを実装する
struct CreateUser {
    username: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)] //テストで必要なSerializeとDeserializeを実装する
struct User {
    id: u64,
    username: String,
}

//testモジュールはプロダクションコードからは削除される
#[cfg(test)]
mod test {
    use super::*;
    use axum::{
        body::Body,
        http::{header, Method, Request},
    };
    use tower::ServiceExt;

    // axum;;http;;Request::builder()でリクエストを作成する
    // tower::ServiceExt;;oneshotでリクエストを送信する
    // hyper::body::to_bytesでレスポンスのボディを取得する

    #[tokio::test]  //ルート関数のテスト
    async fn should_return_hello_world() {
        let req = Request::builder().uri("/").body(Body::empty()).unwrap();
        let res = create_app().oneshot(req).await.unwrap();

        let bytes = hyper::body::to_bytes(res.into_body()).await.unwrap();  //Bytes型からString型に変換する
        let body: String = String::from_utf8(bytes.to_vec()).unwrap();
        assert_eq!(body, "Hello, world!");
    }

    #[tokio::test]  //JSON bodyのテスト
    async fn should_return_user_data() {
        let req = Request::builder()
            .uri("/users")
            .method(Method::POST) //POST,contents-typeは、axum::httpのconstで定義されている
            .header(header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref()) //application/jsonは、mimeのパッケージで定義されている
            .body(Body::from(r#"{"username": "大久保 璃子"}"#))
            .unwrap();
        let res = create_app().oneshot(req).await.unwrap();

        let bytes = hyper::body::to_bytes(res.into_body()).await.unwrap(); 
        let body: String = String::from_utf8(bytes.to_vec()).unwrap();
        let user: User = serde_json::from_str(&body).expect("cannot conver User instance.");    //JSONをUser型に変換する
        assert_eq!(
            user,
            User {
                id: 1337,
                username: "大久保 璃子".to_string(),
            }
        );
    }
}