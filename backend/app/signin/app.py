from shared.common import get_secret_hash, build_response, validate_password, validate_email
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
        password = body.get("password", "").strip()

        if not email or not password:
            return build_response(400, {"error": "Missing email or password"})

        # 入力バリデーション
        email_error = validate_email(email)
        if email_error:
            return build_response(400, {"error": email_error})

        password_error = validate_password(password)
        if password_error:
            return build_response(400, {"error": password_error})

        response = cognito.initiate_auth(
            AuthFlow="USER_PASSWORD_AUTH",
            AuthParameters={
                "USERNAME": email,
                "PASSWORD": password,
                "SECRET_HASH": get_secret_hash(email, CLIENT_ID, CLIENT_SECRET)
            },
            ClientId=CLIENT_ID
        )

        if "ChallengeName" not in response:
            return build_response(500, {"error": "Unexpected response from Cognito"})

        return build_response(200, {
            "message": "MFA required",
            "challenge_name": response["ChallengeName"],
            "session": response.get("Session")
        })

    except cognito.exceptions.NotAuthorizedException:
        return build_response(401, {"error": "メールアドレスまたはパスワードが正しくありません"})

    except cognito.exceptions.UserNotConfirmedException:
        return build_response(403, {"error": "アカウントの確認が完了していません。ご登録時に送信された確認コードを入力して、アカウントを有効化してください。"})

    except Exception as e:
        return build_response(500, {"error": str(e)})