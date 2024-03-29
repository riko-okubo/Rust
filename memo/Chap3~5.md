## main.rs

このファイルでは、アプリケーションのエントリーポイントを定義している。
また、アプリケーションのルーティングを定義し、サーバーを起動。
アプリケーションのルーティングには、ハンドラー（リクエストを受け取り、レスポンスを返す関数）を定義。

このファイルでは、ルートハンドラーと、Todo リソースの CRUD ハンドラーを定義。
ハンドラーの実装には、リポジトリ（データベースやメモリなどのデータソースに対して、データの読み書きを行うオブジェクト）を利用。

このファイルでは、リポジトリの実装には、データベースを利用。
データベースには、PostgreSQL を利用。
また、リポジトリの実装には、メモリを利用するテスト用のリポジトリも定義。
このファイルでは、テスト用のリポジトリの実装には、HashMap を利用。

### async fn main()

→ 環境変数の読み込みと、ロギングの初期化

- env とは環境変数を扱うライブラリ
- "RUST_LOG"とは、ログレベルを設定する環境変数
- unwrap_or()は、env::var()がエラーを返した場合にデフォルト値を返す
- tracing とは、ログを出力するためのライブラリ
- tracing::debug!マクロは、RUST_LOG に指定したログレベル以上のログを出力する
- unwrap()はエラーが起きたら panic する

* layer とは、リクエストを処理する前に実行される処理のこと
  → ここではリクエストを処理する前に repository を Extension として渡している

- ’はライフタイム（lifetime）と呼ばれるもので、参照の有効期間を表す
- mod とは、モジュールを定義するもの
- super は test モジュールの親モジュール（この場合は main.rs）を指す
- crate はルートモジュールを指す

## repositories.rs

このファイルでは、Todo の CRUD を行うためのリポジトリを定義している。
リポジトリは、データの永続化を行うための機能を提供する。
→ 今回は、データベースを使って永続化を行うため、リポジトリはデータベースに接続する機能を提供する。
main.rs からリポジトリを引数に取ることで、テスト時にモックを渡せるようにする。

- std::は標準ライブラリ(HashMap など)
- sync とは、複数のスレッドが同時にアクセスできるようにするための機能
- Arc は複数のスレッドから参照されることを想定したスマートポインタ（参照カウント）
- RwLock は複数のスレッドから読み書きできる
- RwLockReadGuard は読み込み専用、RwLockWriteGuard は書き込み専用のロック（排他制御、つまり同時に書き込みができない）を提供する
- Context とは、エラーのコンテキストを追加するためのライブラリ
- trait とは、メソッドのシグネチャ（戻り値の型、引数の型）を定義することができる
- Clone,Send,Sync はトレイト境界(トレイトを実装している型のみを受け付ける)
- impl とは、構造体や enum に関連関数やメソッドを定義することができる
- Option 型とは、値があるかないかを表現する列挙型
- store とは一般的にデータを保存する場所のこと（ここでは HashMap）
- unwrap でエラーを強制的に発生させる（テスト時に panic を起こす）
- for とは、トレイトを実装する型を指定するための構文
- trait とは、メソッドのシグネチャ（関数の定義）のみを定義したもの
- \_ref は参照を返す
- insert とは、HashMap にキーと値を追加するメソッド
- Vec は可変長配列
- from_iter はイテレータからコレクションを作成する
- イテレータとは、コレクションの要素を順番に取り出すことができるオブジェクト
- コレクションとは、複数の値をまとめて管理するデータ構造のこと
- ok_or は Result 型の値が Err の場合に引数の値を返す
- Ok(())は Result 型の Ok を返す
- assert_eq!は値の比較、assert!は bool 値の比較

* bind は値をバインド(結びつける)するメソッド

## handlers.rs

このファイルでは、リクエストを処理する関数を定義している。

- payload とは、リクエストボディの JSON をデシリアライズ（JSON を構造体に変換）したもの
- Json(): Json<CreateTodo>を使うことで、リクエストボディを CreateTodo に変換して受け取ることができる
- Extension はリクエストハンドラーに状態(ここではリポジトリ)を渡すためのもの
- IntoResponse とは、レスポンスに変換できるものを表すトレイト（つまり、レスポンスを返す関数の戻り値の型として使える）
  - IntoResponse を返すことで、レスポンスを生成することができる
- StatusCode とは HTTP ステータスコード（200, 404, 500 など）を表す型
- Result 型（Ok 値と Err 値のどちらかを返す型）を返すことで、エラーを返すことができる　 → id に該当する Todo がない場合に 404 を返す
- .or(Err(StatusCode::NOT_FOUND))?は、update が Err を返した場合に StatusCode::NOT_FOUND を返す
- delete_todo で StatusCode を返すだけなのは、レスポンス（Json）を返す必要がないため
- .map とは、Result の中身が Ok の場合のみ、クロージャーの処理を実行するメソッド
  - delete メソッドが Ok を返した場合に StatusCode::NO_CONTENT を返す
  - クロージャーとは、関数を変数に代入したり、引数として渡したりできる関数のこと
- unwrap_or は、Ok の場合はそのまま返し、Err の場合は引数の値を返す

* (T)の（）はタプル（tuple）と呼ばれる。タプルは異なる型の値を一つにまとめることができる。ここでは、T 型の値を一つだけ持つタプルを定義している
* impl<T, B> は、トレイト境界と呼ばれ、T と B がトレイトを実装していることをコンパイラ（Rust のコンパイラは、コンパイル時に型の安全性をチェックする）に伝える
* where とは型パラメータの境界（型パラメータがどのような性質を持つかを制限するもの）を定義するもので、型パラメータに制約をつけることができる
* .map_err(|rejection| { })の||はパイプライン演算子で、左辺の結果（ここでは Json）が Err の場合に右辺の処理（ここでは map_err）を実行する
  - |…| expr クロージャ（関数閉包）: 外の呼び出しに対しても変数の値が初期化されず、継続される
* ?とは、Result 型の値を返す関数の末尾につけることで、Result が Err の場合はそのまま Err を返し、Ok の場合は Ok の中身を返すという処理を簡略化するもの
