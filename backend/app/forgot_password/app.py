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

        if not email:
            return build_response(400, {"error": "Missing email"})

        # パスワード再設定コードの送信
        response = cognito.forgot_password(
            ClientId=CLIENT_ID,
            Username=email,
            SecretHash=get_secret_hash(email, CLIENT_ID, CLIENT_SECRET)
        )

        return build_response(200, {
            "message": "パスワード再設定用コードを送信しました",
            "delivery": response.get("CodeDeliveryDetails")
        })

    except cognito.exceptions.UserNotFoundException:
        return build_response(404, {"error": "ユーザーが見つかりません"})

    except Exception as e:
        return build_response(500, {"error": str(e)})