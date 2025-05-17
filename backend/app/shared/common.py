import json
import hmac
import hashlib
import base64
import re

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

def validate_email(email: str) -> str | None:
    """
    メールアドレスのバリデーションを行う。

    入力値が空でないこと、および有効なメールアドレス形式であることを検証する。
    許容形式は「username@domain.tld」の基本的な構造。

    Args:
        email (str): 検証対象のメールアドレス。

    Returns:
        str | None: エラーメッセージ（エラーがある場合）または None（問題ない場合）。

    Examples:
        >>> validate_email("") 
        'メールアドレスは必須です'
        >>> validate_email("invalid-email")
        '有効なメールアドレスを入力してください'
        >>> validate_email("user@example.com")
        None
    """
    email = email.strip()
    if not email:
        return "メールアドレスは必須です"
    if not re.match(r"^[\w\.-]+@[\w\.-]+\.\w+$", email):
        return "有効なメールアドレスを入力してください"
    return None


def validate_password(password: str) -> str | None:
    """
    パスワードのバリデーションを行う。

    以下の条件をすべて満たすか検証する：
    - 最低8文字
    - 少なくとも1つの数字
    - 少なくとも1つの特殊文字
    - 少なくとも1つの大文字
    - 少なくとも1つの小文字

    Args:
        password (str): 検証対象のパスワード。

    Returns:
        str | None: エラーメッセージ（エラーがある場合）または None（問題ない場合）。

    Examples:
        >>> validate_password("abc")
        'パスワードは8文字以上で入力してください'
        >>> validate_password("abcdefgh")
        'パスワードには少なくとも1つの数字が必要です'
        >>> validate_password("Abcd1234!")
        None
    """
    if len(password) < 8:
        return "パスワードは8文字以上で入力してください"
    if not re.search(r"[0-9]", password):
        return "パスワードには少なくとも1つの数字が必要です"
    if not re.search(r"[!@#$%^&*(),.?\":{}|<>]", password):
        return "パスワードには少なくとも1つの特殊文字が必要です"
    if not re.search(r"[A-Z]", password):
        return "パスワードには少なくとも1つの大文字が必要です"
    if not re.search(r"[a-z]", password):
        return "パスワードには少なくとも1つの小文字が必要です"
    return None

def validate_code(code):
    """
    確認コードが6桁の数字であるかを検証します。
    """
    if not re.fullmatch(r"\d{6}", code):
        return "確認コードは6桁の数字で入力してください"
    return None