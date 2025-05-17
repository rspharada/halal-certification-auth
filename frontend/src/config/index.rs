//! 認証後のリダイレクト先URLを返す
//! `.env` ファイルに APP_REDIRECT_URL を定義しておく必要があります。

pub fn get_redirect_url() -> String {
    option_env!("APP_REDIRECT_URL")
        .expect("APP_REDIRECT_URL が定義されていません。環境変数または .env に設定してください。")
        .to_string()
}
