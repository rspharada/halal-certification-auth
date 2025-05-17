import boto3
import json
import os
from shared.common import (
    get_secret_hash,
    build_response,
    validate_email,
    validate_code,
    validate_password,
)

cognito = boto3.client("cognito-idp")

CLIENT_ID = os.environ["COGNITO_APP_CLIENT_ID"]
CLIENT_SECRET = os.environ["COGNITO_APP_CLIENT_SECRET"]

def lambda_handler(event, context):
    try:
        body = json.loads(event.get("body", "{}"))
        email = body.get("email", "").strip()
        code = body.get("code", "").strip()
        new_password = body.get("new_password", "").strip()

        if not email or not code or not new_password:
            return build_response(400, {"error": "Missing email, code, or new_password"})

        # 入力バリデーション
        email_error = validate_email(email)
        code_error = validate_code(code)
        password_error = validate_password(new_password)

        if email_error:
            return build_response(400, {"error": email_error})
        if code_error:
            return build_response(400, {"error": code_error})
        if password_error:
            return build_response(400, {"error": password_error})

        # パスワードリセットの確定処理
        cognito.confirm_forgot_password(
            ClientId=CLIENT_ID,
            Username=email,
            ConfirmationCode=code,
            Password=new_password,
            SecretHash=get_secret_hash(email, CLIENT_ID, CLIENT_SECRET)
        )

        return build_response(200, {"message": "パスワードを変更しました"})

    except cognito.exceptions.CodeMismatchException:
        return build_response(400, {"error": "確認コードが間違っています"})

    except cognito.exceptions.ExpiredCodeException:
        return build_response(400, {"error": "確認コードの有効期限が切れています"})

    except Exception as e:
        return build_response(500, {"error": str(e)})