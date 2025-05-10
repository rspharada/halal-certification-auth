# 使い方

## ビルド

```bash
sh ./build.sh
```

## テスト

```bash
# [形式]
sam local invoke {関数名} --event ./events/{イベント名}.json

# [SignupFunction]
sam local invoke SignupFunction --event ./events/event_signup.json --env-vars=env.local.json --profile verify

# [SignupConfirmFunction]
sam local invoke SignupConfirmFunction --event ./events/event_signup_confirm.json --env-vars=env.local.json --profile verify

# [SigninFunction]
sam local invoke SigninFunction --event ./events/event_signin.json --env-vars=env.local.json --profile verify

```

```bash
sam local start-api --port 8080 --env-vars=env.local.json
```

## デプロイ

```bash
# 初回
sam deploy --profile internal --guided --config-env stg --capabilities CAPABILITY_NAMED_IAM

# 2回目
sam deploy --profile internal --config-env stg --capabilities CAPABILITY_NAMED_IAM
```

## 削除
```bash
sam delete --stack-name mattermost-stg-auth-api --profile internal
```