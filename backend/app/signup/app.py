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

        response = cognito.sign_up(
            ClientId=CLIENT_ID,
            SecretHash=get_secret_hash(email, CLIENT_ID, CLIENT_SECRET),
            Username=email,
            Password=password,
            UserAttributes=[
                {"Name": "email", "Value": email}
            ]
        )

        return build_response(200, {
            "message": "仮登録が完了しました（クライアントシークレット使用）",
            "user_sub": response["UserSub"]
        })

    except cognito.exceptions.UsernameExistsException:
        return build_response(409, {"error": "このメールアドレスは既に登録されています"})

    except Exception as e:
        return build_response(500, {"error": str(e)})