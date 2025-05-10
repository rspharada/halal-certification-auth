# ハラール認証 - 認証ページ（SPA）

## フォルダ構成

```console
app
├── Cargo.toml
├── pkg/                    # wasm-pack が出力する生成物（wasm, JS bindings）
├── static/                 # 静的ファイル（HTML, CSS, JS）
│   └── index.html
├── src/                    # Rustコード（lib.rs 必須）
│   ├── lib.rs              # wasm-bindgenエントリーポイント
│   ├── app.rs              # Appコンポーネント（ルーティング付き）
│   ├── routes.rs           # ルーティング定義
│   ├── pages/
│   │   ├── signup.rs       # サインアップページ
│   │   └── signin.rs       # サインインページ
│   ├── components/
│   │   └── signup_form.rs  # サインアップフォーム
│   ├── types.rs            # 型定義（構造体など）
│   └── utils.rs            # バリデーションなどの共通関数
├── .gitignore
├── README.md
```

## 静的ファイルの配信

```console
trunk serve --open --port XXXX
```

## テスト実行（ブラウザ上）

```console
wasm-pack test --headless --chrome
```
--headless を外すとブラウザが開きます。

|   **状況**   |   **対応方法**   |
| --- | --- |
|   ブラウザ開かずに実行したい   |   \--headless を指定   |
|   コンソールがエラーになる場合   |   cargo clean, wasm-pack build でリセット   |
|   CLIだけで流したい（CI含む）   |   wasm-pack test --headless --chrome   |