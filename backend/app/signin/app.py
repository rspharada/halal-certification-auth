from shared.common import get_secret_hash, build_response
import json
import os
import boto3

cognito = boto3.client("cognito-idp")

CLIENT_ID = os.environ["COGNITO_APP_CLIENT_ID"]
CLIENT_SECRET = os.environ["COGNITO_APP_CLIENT_SECRET"]

def lambda_handler(event, context):
    try:
        body = json.loads(event.get("body", "{}"))
        email = body.get("email")
        password = body.get("password")

        if not email or not password:
            return build_response(400, {"error": "Missing email or password"})

        response = cognito.initiate_auth(
            AuthFlow="USER_PASSWORD_AUTH",
            AuthParameters={
                "USERNAME": email,
                "PASSWORD": password,
                "SECRET_HASH": get_secret_hash(email, CLIENT_ID, CLIENT_SECRET)
            },
            ClientId=CLIENT_ID
        )

        # MFAまたはカスタムチャレンジが必要な場合
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
        return build_response(403, {"error": "ユーザーが確認されていません"})

    except Exception as e:
        return build_response(500, {"error": str(e)})