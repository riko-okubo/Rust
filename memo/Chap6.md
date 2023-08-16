## ラベル機能を追加

repository,handler は todo でも label でもあるため、分割する

### src/repositories.rs

データベースの操作を行うための関数を定義している.

### src/handlers.rs

リクエストを処理する関数を定義する.

↓ ここでは、FromRequest トレイト（つまり、リクエストから値を抽出するトレイト）を実装している．
impl<T, B> FromRequest<B> for ValidatedJson<T>

### src/repositories/todo.rs

- このファイルは、Todo の CRUD を行う Repository を定義する。
- Repository は、データの永続化を行う層のことで、データベースにアクセスするための機能を提供する．
- 今回は、sqlx を使ってデータベースとの接続を行い、永続化する。
- sqlx は、sqlx::query_as を使って、データベースから取得したデータを、自動的に構造体に変換してくれる。

```
//TodoRepositoryForDb構造体（つまり、TodoのCRUDを行うRepository）を定義しているが、async_traitなので、非同期処理を行うことができる。
impl TodoRepository for TodoRepositoryForDb
```

```
//この＋によって、トレイト境界（つまり、トレイトを実装する型の制限）を複数指定できる。
pub trait TodoRepository: Clone + std::marker::Send + std::marker::Sync + 'static
```

- crud_scenario()の特徴は、データベースを使っていること.
- test_utils では、テスト用のユーティリティ（便利な関数）を定義している.
  → このテストはメモリを使う

```
//ここでは、TodoRepositoryForMemory構造体（つまり、TodoのCRUDを行うRepository）を定義しているが、cfg(test)なので、テスト時のみ有効.
impl TodoRepositoryForMemory
```

### src/repositories/label.rs

このファイルは、ラベルの CRUD を行うための Repository を定義している．
Todo と同じだが、メソッドは更新や個人的な読み取りはいらないため、「create」,「all」,「delete」を定義する.

### src/handlers/todo.rs

Todo の API の実装を行う.
それぞれの関数では、Path と Extension を引数に取っている.
Path と Extension は、それぞれ、URL のパスの部分と、リクエストの中身を取得するための構造体.

#### create_todo()

1. リクエストの中身を取得する（ValidatedJson, Extension）
2. Todo を作成する（TodoRepository の create()）
3. 作成した Todo を返す（Json）
4. Todo を作成できなかったら、NOT_FOUND を返す（Err(StatusCode::NOT_FOUND)）
5. Todo を作成するために、TodoRepository の create()を呼び出す（repository.create(payload)）
6. Todo を作成したら、作成した Todo を返す（Ok(todo)）

#### find_todo()

1. リクエストの中身を取得する（Path と Extension）
2. Todo を取得する（TodoRepository の find()）
3. Todo を取得できなかったら、NOT_FOUND を返す（Err(StatusCode::NOT_FOUND)）
4. Todo を取得するために、TodoRepository の find()を呼び出す（repository.find(id)）
5. Todo を取得したら、取得した Todo を返す（Ok(todo)）

#### update_todo()

1. リクエストの中身を取得する（Path と ValidatedJson と Extension）
2. Todo を更新する（TodoRepository の update()）
3. Todo を更新できなかったら、NOT_FOUND を返す（Err(StatusCode::NOT_FOUND)）
4. Todo を更新するために、TodoRepository の update()を呼び出す（repository.update(id, payload)）
5. Todo を更新したら、更新した Todo を返す（Ok(todo)）

#### delete_todo()

1. リクエストの中身を取得する（Path と Extension）
2. Todo を削除する（TodoRepository の delete()）
3. Todo を削除できなかったら、NOT_FOUND を返す（Err(StatusCode::NOT_FOUND)）
4. Todo を削除するために、TodoRepository の delete()を呼び出す（repository.delete(id)）
5. Todo を削除したら、NO\*CONTENT を返す（Ok(StatusCode::NO_CONTENT)）

   |\*|は、エラーの場合の処理を記述している.

### src/handlers/todo.rs

src/handlers/todo.rs とほぼ同じ
