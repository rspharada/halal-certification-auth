from shared.common import get_secret_hash, build_response, validate_email, validate_code
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
        email = body.get("email", "").strip()
        code = body.get("code", "").strip()
        session = body.get("session", "").strip()

        # 入力チェック
        if not email or not code or not session:
            return build_response(400, {"error": "Missing email, code or session"})

        # バリデーションチェック
        email_error = validate_email(email)
        code_error = validate_code(code)

        if email_error:
            return build_response(400, {"error": email_error})
        if code_error:
            return build_response(400, {"error": code_error})

        # 認証チャレンジへの応答
        response = cognito.respond_to_auth_challenge(
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
        redirect_url = f"{scheme}://www.{DOMAIN}{REDIRECT_PATH}"
        secure_flag = "Secure;" if ENV != "local" else ""

        return {
            "statusCode": 200,
            "multiValueHeaders": {
                "Set-Cookie": [
                    f"access_token={tokens['AccessToken']}; Path=/; Domain=.{DOMAIN}; HttpOnly; {secure_flag} SameSite=Lax; Max-Age=3600",
                    f"refresh_token={tokens['RefreshToken']}; Path=/; Domain=.{DOMAIN}; HttpOnly; {secure_flag} SameSite=Lax; Max-Age=3600",
                    f"id_token={tokens['IdToken']}; Path=/; Domain=.{DOMAIN}; HttpOnly; {secure_flag} SameSite=Lax; Max-Age=3600",
                ]
            },
            "headers": {
                "Content-Type": "application/json"
            },
            "body": json.dumps({ "message": "authenticated" })
        }

    except cognito.exceptions.NotAuthorizedException:
        return build_response(401, {"error": "確認コードが正しくありません"})

    except Exception as e:
        return build_response(500, {"error": str(e)})