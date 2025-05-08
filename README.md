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

## was-pack インストール

```console
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
```

## miniserveインストール

```console
brew install miniserve
```

## ビルド方法

```console
cd app
# ビルド（wasm を生成）
wasm-pack build --target web --out-name wasm --out-dir ./static
```

## 静的ファイルの配信

```console
miniserve ./static --index index.html --port 3000
```

|   **オプション**   |   **説明**   |
| --- | --- |
|   \--port <番号>   |   ポート番号を指定（デフォルトは 8080）   |
|   \--index <file>   |   SPA用のエントリHTML（例: index.html）   |
|   \--enable-cors   |   CORSヘッダを付与（API連携の開発に便利）   |
|   \--open   |   起動後にブラウザを自動で開く   |
|   \--auth user:pw   |   Basic認証をつける（必要なら）   |