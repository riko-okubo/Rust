### async fn main()

- env とは環境変数を扱うライブラリ
- "RUST_LOG"とは、ログレベルを設定する環境変数
- unwrap_or()は、env::var()がエラーを返した場合にデフォルト値を返す
- tracing とは、ログを出力するためのライブラリ
- tracing::debug!マクロは、RUST_LOG に指定したログレベル以上のログを出力する
- unwrap()はエラーが起きたら panic する

- ’はライフタイム（lifetime）と呼ばれるもので、参照の有効期間を表す
- mod とは、モジュールを定義するもの
- super は test モジュールの親モジュール（この場合は main.rs）を指す
- crate はルートモジュールを指す

## repositories.rs

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

## handlers.rs

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
