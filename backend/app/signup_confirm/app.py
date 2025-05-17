from shared.common import get_secret_hash, build_response, validate_code, validate_email
import json
import os
import re
import boto3

cognito = boto3.client("cognito-idp")

CLIENT_ID = os.environ["COGNITO_APP_CLIENT_ID"]
CLIENT_SECRET = os.environ["COGNITO_APP_CLIENT_SECRET"]

def lambda_handler(event, context):
    try:
        body = json.loads(event.get("body", "{}"))
        email = body.get("email", "").strip()
        code = body.get("code", "").strip()

        if not email or not code:
            return build_response(400, {"error": "Missing email or code"})

        # バリデーション追加
        email_error = validate_email(email)
        if email_error:
            return build_response(400, {"error": email_error})

        code_error = validate_code(code)
        if code_error:
            return build_response(400, {"error": code_error})

        # 確認コードによる本登録
        cognito.confirm_sign_up(
            ClientId=CLIENT_ID,
            Username=email,
            ConfirmationCode=code,
            SecretHash=get_secret_hash(email, CLIENT_ID, CLIENT_SECRET)
        )

        return build_response(200, {"message": "本登録が完了しました"})

    except cognito.exceptions.CodeMismatchException:
        return build_response(400, {"error": "確認コードが間違っています"})

    except cognito.exceptions.ExpiredCodeException:
        return build_response(400, {"error": "確認コードの有効期限が切れています"})

    except cognito.exceptions.UserNotFoundException:
        return build_response(404, {"error": "ユーザーが見つかりません"})

    except Exception as e:
        return build_response(500, {"error": str(e)})