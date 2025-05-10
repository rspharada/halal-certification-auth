import json
import pytest
import os
import mocker
import app.signin.app as app

@pytest.fixture()
def apigw_event_valid():
    return {
        "body": json.dumps({
            "email": "test@example.com",
            "password": "TestPassword123!"
        }),
        "httpMethod": "POST",
        "headers": {
            "Content-Type": "application/json"
        },
        "isBase64Encoded": False
    }

def test_lambda_handler_success(apigw_event_valid, mocker):
    # モックの戻り値を定義
    mock_response = {
        "ChallengeName": "SMS_MFA",
        "Session": "dummy-session-token"
    }

    # cognito-idp の initiate_auth をモック
    mock_client = mocker.patch("signin.app.cognito")
    mock_client.initiate_auth.return_value = mock_response

    # 環境変数もテスト内で設定（またはpytestの前処理で代用可能）
    os.environ["COGNITO_APP_CLIENT_ID"] = "dummy-client-id"
    os.environ["COGNITO_APP_CLIENT_SECRET"] = "dummy-secret"

    # 実行
    ret = app.lambda_handler(apigw_event_valid, None)
    data = json.loads(ret["body"])

    # 検証
    assert ret["statusCode"] == 200
    assert data["message"] == "MFA required"
    assert data["challenge_name"] == "SMS_MFA"
    assert data["session"] == "dummy-session-token"