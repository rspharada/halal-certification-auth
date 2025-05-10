import json
import hmac
import hashlib
import base64

def get_secret_hash(username: str, client_id: str, client_secret: str) -> str:
    """
    AWS Cognito 用の SECRET_HASH を生成するユーティリティ関数。

    Cognito の App Client に Client Secret が有効化されている場合、
    認証 API 呼び出し時（SignUp, SignIn, ConfirmSignUp など）に
    SECRET_HASH を付与する必要がある。

    生成ロジック:
      HMAC_SHA256(Username + ClientId, ClientSecret) を Base64 エンコード

    Parameters:
    ----------
    username : str
        Cognito に登録されているユーザー名（通常はメールアドレス）
    client_id : str
        Cognito の App Client ID
    client_secret : str
        Cognito の App Client Secret

    Returns:
    -------
    str
        Base64 エンコードされた HMAC-SHA256 ダイジェスト（SECRET_HASH）
    """
    message = username + client_id
    dig = hmac.new(
        key=client_secret.encode("utf-8"),
        msg=message.encode("utf-8"),
        digestmod=hashlib.sha256
    ).digest()
    return base64.b64encode(dig).decode()

def build_response(status_code, message):
    """
    汎用的なAPI Gateway向けHTTPレスポンス整形関数

    Parameters:
    - status_code (int): HTTPステータスコード（例: 200, 400, 500）
    - message (dict): JSONとして返すメッセージ内容（辞書）

    Returns:
    dict: API Gateway Lambda Proxy統合形式のレスポンス
    """
    return {
        "statusCode": status_code,
        "body": json.dumps(message)
    }