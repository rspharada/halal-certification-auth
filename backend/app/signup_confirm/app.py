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
        code = body.get("code")

        if not email or not code:
            return {
                "statusCode": 400,
                "body": json.dumps({"error": "Missing email or code"})
            }

        cognito.confirm_sign_up(
            ClientId=CLIENT_ID,
            Username=email,
            ConfirmationCode=code,
            SecretHash=get_secret_hash(email, CLIENT_ID, CLIENT_SECRET)
        )

        return {
            "statusCode": 200,
            "body": json.dumps({"message": "本登録が完了しました"})
        }

    except cognito.exceptions.CodeMismatchException:
        return {
            "statusCode": 400,
            "body": json.dumps({"error": "確認コードが間違っています"})
        }

    except cognito.exceptions.ExpiredCodeException:
        return {
            "statusCode": 400,
            "body": json.dumps({"error": "確認コードの有効期限が切れています"})
        }

    except cognito.exceptions.UserNotFoundException:
        return {
            "statusCode": 404,
            "body": json.dumps({"error": "ユーザーが見つかりません"})
        }

    except Exception as e:
        return {
            "statusCode": 500,
            "body": json.dumps({"error": str(e)})
        }