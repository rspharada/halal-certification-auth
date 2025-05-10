import boto3
import json
import os
from shared.common import get_secret_hash, build_response

cognito = boto3.client("cognito-idp")

CLIENT_ID = os.environ["COGNITO_APP_CLIENT_ID"]
CLIENT_SECRET = os.environ["COGNITO_APP_CLIENT_SECRET"]

def lambda_handler(event, context):
    try:
        body = json.loads(event.get("body", "{}"))
        email = body.get("email")
        code = body.get("code")
        new_password = body.get("new_password")

        if not email or not code or not new_password:
            return build_response(400, {"error": "Missing email, code, or new_password"})

        # パスワードリセット確定
        cognito.confirm_forgot_password(
            ClientId=CLIENT_ID,
            Username=email,
            ConfirmationCode=code,
            Password=new_password,
            SecretHash=get_secret_hash(email, CLIENT_ID, CLIENT_SECRET)
        )

        return build_response(200, {"message": "パスワードを変更しました"})

    except cognito.exceptions.CodeMismatchException:
        return build_response(400, {"error": "認証コードが間違っています"})

    except cognito.exceptions.ExpiredCodeException:
        return build_response(400, {"error": "認証コードの有効期限が切れています"})

    except Exception as e:
        return build_response(500, {"error": str(e)})