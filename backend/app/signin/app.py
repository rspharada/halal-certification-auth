from shared.common import get_secret_hash
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
            return {
                "statusCode": 400,
                "body": json.dumps({"error": "Missing email or password"})
            }

        response = cognito.admin_initiate_auth(
            UserPoolId=os.environ["COGNITO_USER_POOL_ID"],
            ClientId=CLIENT_ID,
            AuthFlow="ADMIN_NO_SRP_AUTH",
            AuthParameters={
                "USERNAME": email,
                "PASSWORD": password,
                "SECRET_HASH": get_secret_hash(email, CLIENT_ID, CLIENT_SECRET)
            }
        )

        # MFAまたはカスタムチャレンジが必要な場合
        if "ChallengeName" in response:
            return {
                "statusCode": 200,
                "body": json.dumps({
                    "message": "MFA required",
                    "challenge_name": response["ChallengeName"],
                    "session": response.get("Session")
                })
            }

        # MFAが必須なのにトークンが返ってきたのは想定外（エラー）
        if "AuthenticationResult" in response:
            return {
                "statusCode": 500,
                "body": json.dumps({
                    "error": "Unexpected token response: MFA is required, but tokens were returned"
                })
            }

        # 他に想定外のレスポンスが返ってきた場合
        return {
            "statusCode": 500,
            "body": json.dumps({"error": "Unexpected response from Cognito"})
        }

    except cognito.exceptions.NotAuthorizedException:
        return {
            "statusCode": 401,
            "body": json.dumps({"error": "メールアドレスまたはパスワードが正しくありません"})
        }

    except cognito.exceptions.UserNotConfirmedException:
        return {
            "statusCode": 403,
            "body": json.dumps({"error": "ユーザーが確認されていません"})
        }

    except Exception as e:
        return {
            "statusCode": 500,
            "body": json.dumps({"error": str(e)})
        }