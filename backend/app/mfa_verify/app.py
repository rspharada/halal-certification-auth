from shared.common import get_secret_hash, build_response
import os
import json
import boto3

cognito = boto3.client("cognito-idp")

ENV = os.getenv("ENV", "local")
DOMAIN = os.environ["DOMAIN"]
REDIRECT_PATH = os.environ["REDIRECT_PATH"]
USER_POOL_ID = os.environ["COGNITO_USER_POOL_ID"]
CLIENT_ID = os.environ["COGNITO_APP_CLIENT_ID"]
CLIENT_SECRET = os.environ["COGNITO_APP_CLIENT_SECRET"]
scheme = "http" if ENV == "local" else "https"

def lambda_handler(event, context):
    try:
        body = json.loads(event.get("body", "{}"))
        email = body.get("email")
        code = body.get("code")
        session = body.get("session")

        if not email or not code or not session:
            return build_response(400, {"error": "Missing email, code or session"})

        response = cognito.admin_respond_to_auth_challenge(
            UserPoolId=USER_POOL_ID,
            ClientId=CLIENT_ID,
            ChallengeName="EMAIL_OTP",
            ChallengeResponses={
                "USERNAME": email,
                "EMAIL_OTP_CODE": code,
                "SECRET_HASH": get_secret_hash(email, CLIENT_ID, CLIENT_SECRET)
            },
            Session=session
        )

        tokens = response["AuthenticationResult"]
        redirect_url = f"{scheme}://{DOMAIN}{REDIRECT_PATH}"

        return {
            "statusCode": 302,
            "headers": {
                "Location": redirect_url,
                "Set-Cookie": (
                    f"access_token={tokens['AccessToken']}; Path=/; HttpOnly; Secure; SameSite=Lax, "
                    f"refresh_token={tokens['RefreshToken']}; Path=/; HttpOnly; Secure; SameSite=Lax, "
                    f"id_token={tokens['IdToken']}; Path=/; HttpOnly; Secure; SameSite=Lax"
                )
            },
            "body": ""
        }

    except cognito.exceptions.NotAuthorizedException:
        return build_response(401, {"error": "認証コードが正しくありません"})

    except Exception as e:
        return build_response(500, {"error": str(e)})