import boto3
import json
import os
from shared.common import get_secret_hash

cognito = boto3.client("cognito-idp")

CLIENT_ID = os.environ["COGNITO_APP_CLIENT_ID"]
CLIENT_SECRET = os.environ["COGNITO_APP_CLIENT_SECRET"]

def lambda_handler(event, context):
    try:
        body = json.loads(event.get("body", "{}"))
        email = body.get("email")

        if not email:
            return {
                "statusCode": 400,
                "body": json.dumps({"error": "Missing email"})
            }

        # 認証コードの再送
        response = cognito.resend_confirmation_code(
            ClientId=CLIENT_ID,
            Username=email,
            SecretHash=get_secret_hash(email, CLIENT_ID, CLIENT_SECRET)
        )

        return {
            "statusCode": 200,
            "body": json.dumps({
                "message": "確認コードを再送しました",
                "delivery": response.get("CodeDeliveryDetails")
            })
        }

    except cognito.exceptions.UserNotFoundException:
        return {
            "statusCode": 404,
            "body": json.dumps({"error": "ユーザーが見つかりません"})
        }

    except cognito.exceptions.InvalidParameterException:
        return {
            "statusCode": 400,
            "body": json.dumps({"error": "ユーザーは既に確認済みです"})
        }

    except Exception as e:
        return {
            "statusCode": 500,
            "body": json.dumps({"error": str(e)})
        }