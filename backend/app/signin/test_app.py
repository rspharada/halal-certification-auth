import unittest
from unittest.mock import patch, MagicMock
import app  # テスト対象のモジュール（ファイル名が app.py の場合）

class TestLambdaHandler(unittest.TestCase):

    @patch("app.cognito.initiate_auth")
    def test_mfa_challenge_response(self, mock_initiate_auth):
        # モックでMFAチャレンジを返すようにする
        mock_initiate_auth.return_value = {
            "ChallengeName": "SOFTWARE_TOKEN_MFA",
            "Session": "dummy-session-token"
        }

        event = {
            "body": '{"email": "test@example.com", "password": "Passw0r92d"}'
        }

        result = app.lambda_handler(event, None)

        self.assertEqual(result["statusCode"], 200)
        self.assertIn("MFA required", result["body"])

    @patch("app.cognito.initiate_auth")
    def test_unexpected_response(self, mock_initiate_auth):
        # ChallengeName がない → エラー扱いになる想定
        mock_initiate_auth.return_value = {}

        event = {
            "body": '{"email": "test@example.com", "password": "TestPass123"}'
        }

        result = app.lambda_handler(event, None)

        self.assertEqual(result["statusCode"], 500)
        self.assertIn("Unexpected response from Cognito", result["body"])

    def test_missing_credentials(self):
        event = {
            "body": '{"email": "", "password": ""}'
        }

        result = app.lambda_handler(event, None)

        self.assertEqual(result["statusCode"], 400)
        self.assertIn("Missing email or password", result["body"])

if __name__ == "__main__":
    unittest.main()