# 📦 Halmark Auth API

AWS SAM を使用して構築された、Cognito ベースの認証バックエンドです。

## ディレクトリ構成

```bash
backend
├── .aws-sam/                         # SAM CLI によるビルド成果物（自動生成・無視推奨）
├── .gitignore                        # Git 管理から除外するファイル設定
├── app/                              # 各 Lambda 関数の実装ディレクトリ
│   ├── shared/                       # 共通ユーティリティ（例: get_secret_hash）
│   ├── forgot_password/              # パスワード再設定申請用 Lambda
│   ├── forgot_password_confirm/      # パスワード再設定確定用 Lambda
│   ├── mfa_verify/                   # MFA 認証コード検証用 Lambda
│   ├── resend_code/                  # 認証コード再送信用 Lambda
│   ├── signin/                       # サインイン処理用 Lambda
│   ├── signup/                       # サインアップ処理用 Lambda
│   └── signup_confirm/               # サインアップ確認用 Lambda
├── build.sh                          # Docker イメージビルドスクリプト
├── env.local.json                    # SAM 実行時の環境変数定義（ローカル用）
├── events/                           # sam local invoke 用のテストイベントファイル
├── README.md                         # プロジェクトの使用方法を記載したドキュメント
├── samconfig.toml                    # sam deploy の設定ファイル
├── template.yaml                     # AWS SAM のテンプレートファイル（全関数定義）
└── tests/                            # ユニットテストや統合テスト用ディレクトリ
```

## 🔧 ビルド

```bash
sh ./build.sh
```

## 🧪 ローカルテスト

### 単体テスト（関数単位）
```bash
# フォーマット:
sam local invoke {関数名} --event ./events/{イベントファイル}.json --env-vars env.local.json --profile verify
```

### 関数別コマンド例
```bash
# サインアップ
sam local invoke SignupFunction --event ./events/event_signup.json --env-vars=env.local.json

# 確認コード再送
sam local invoke ResendCodeFunction --event ./events/event_resend_code.json --env-vars=env.local.json

# サインアップ確認
sam local invoke SignupConfirmFunction --event ./events/event_signup_confirm.json --env-vars=env.local.json

# サインイン
sam local invoke SigninFunction --event ./events/event_signin.json --env-vars=env.local.json

# MFA認証（コード検証）
sam local invoke MfaVerifyFunction --event ./events/event_mfa_verify.json --env-vars=env.local.json

# パスワード再設定リクエスト
sam local invoke ForgotPasswordFunction --event ./events/event_forgot_password.json --env-vars=env.local.json

# パスワード変更
sam local invoke ForgotPasswordConfirmFunction --event ./events/event_forgot_password_confirm.json --env-vars=env.local.json
```

### API Gateway モード
```bash
sam local start-api --port 8080 --env-vars=env.local.json --docker-network=halal_default
```

## デプロイ

### 初回デプロイ（対話モード）
```bash
sam deploy --profile internal --guided --config-env stg --capabilities CAPABILITY_NAMED_IAM
```

### 2回目以降（自動）
```bash
sam deploy --profile internal --config-env stg --capabilities CAPABILITY_NAMED_IAM
```

## 🗑️ スタック削除
```bash
sam delete --stack-name mattermost-stg-auth-api --profile internal
```