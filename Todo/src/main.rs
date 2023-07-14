use anyhow::Context;
use axum::{
    extract::Extension,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use std::{
    collections::HashMap,
    env,
    sync::{Arc, RwLock},
};
use thiserror::Error;

#[tokio::main]
async fn main() {
    let log_level = env::var("RUST_LOG").unwrap_or("info".to_string());
    env::set_var("RUST_LOG", log_level);
    tracing_subscriber::fmt::init();
    let repository = TodoRepositoryForMemory::new();
    let app = create_app(repository);
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    
    tracing::debug!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// テスト対象を切り出す
fn create_app<T: TodoRepository>(repository: T) -> Router { //repositoryを引数に取ることで、テスト時にモックを渡せるようにする
    Router::new()
        .route("/", get(root))
        .route("/users", post(create_user))
        .route("/todos", post(create_todo::<T>))
        .layer(Extension(Arc::new(repository)))
}

// Todoreposirotyトレイトに依存
pub async fn create_todo<T: TodoRepository>(
    Json(payload): Json<CreateTodo>,
    Extension(repository): Extension<Arc<T>>,
) -> impl IntoResponse {
    let todo = repository.create(payload);

    (StatusCode::CREATED, Json(todo))
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
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
struct CreateUser {
    username: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
struct User {
    id: u64,
    username: String,
}

// point 1**（リポジトリで発生し得るエラーを定義）
#[derive(Debug, Error)]
enum RepositoryError {
    #[error("NotFound, id is {0}")]
    NotFound(i32),
}

// point 2**（利用者側の必要としているものを定義）
pub trait TodoRepository: Clone + std::marker::Send + std::marker::Sync + 'static {
    fn create(&self, payload: CreateTodo) -> Todo;
    fn find(&self, id: i32) -> Option<Todo>;
    fn all(&self) -> Vec<Todo>;
    fn update(&self, id: i32, payload: UpdateTodo) -> anyhow::Result<Todo>;
    fn delete(&self, id: i32) -> anyhow::Result<()>;
}

// point 3**（Todo自体やTodoの更新に必要な構造体を定義）
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Todo {
    id: i32,
    text: String,
    completed: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct CreateTodo {
    text: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct UpdateTodo {
    text: Option<String>,
    completed: Option<bool>,
}

impl Todo {
    pub fn new(id: i32, text: String) -> Self {
        Self {
            id,
            text,
            completed: false,
        }
    }
}

//以下、TodoRepositoryForMemoryの実装

type TodoDatas = HashMap<i32, Todo>;

#[derive(Debug, Clone)]
pub struct TodoRepositoryForMemory {
    store: Arc<RwLock<TodoDatas>>,  //Arc<Rwlock<>>はスレッドセーフなデータ構造
}

impl TodoRepositoryForMemory {
    pub fn new() -> Self {
        TodoRepositoryForMemory {
            store: Arc::default(),
        }
    }
}

impl TodoRepository for TodoRepositoryForMemory {
    fn create(&self, payload: CreateTodo) -> Todo {
        todo!();
    }

    fn find(&self, id: i32) -> Option<Todo> {
        todo!();
    }

    fn all(&self) -> Vec<Todo> {
        todo!();
    }

    fn update(&self, id: i32, payload: UpdateTodo) -> anyhow::Result<Todo> {
        todo!()
    }

    fn delete(&self, id: i32) -> anyhow::Result<()> {
        todo!();
    }
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
        let repository = TodoRepositoryForMemory::new();
        let req = Request::builder().uri("/").body(Body::empty()).unwrap();
        let res = create_app(repository).oneshot(req).await.unwrap();

        let bytes = hyper::body::to_bytes(res.into_body()).await.unwrap();  //Bytes型からString型に変換する
        let body: String = String::from_utf8(bytes.to_vec()).unwrap();
        assert_eq!(body, "Hello, world!");
    }

    #[tokio::test]  //JSON bodyのテスト
    async fn should_return_user_data() {
        let repository = TodoRepositoryForMemory::new();
        let req = Request::builder()
            .uri("/users")
            .method(Method::POST) //POST,contents-typeは、axum::httpのconstで定義されている
            .header(header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref()) //application/jsonは、mimeのパッケージで定義されている
            .body(Body::from(r#"{"username": "大久保 璃子"}"#))
            .unwrap();
        let res = create_app(repository).oneshot(req).await.unwrap();

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