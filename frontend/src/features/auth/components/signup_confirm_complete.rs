use crate::routes::Route;
use yew::prelude::*;
use yew_router::components::Link;

/// サインアップ完了メッセージ用コンポーネント
#[function_component(SignupConfirmComplete)]
pub fn signup_confirm_complete() -> Html {
    html! {
        <div class="complete-container">
            <h2 class="form-title">
                { "アカウントの有効化が" }<br />
                { "完了しました" }
            </h2>

            <p>
                { "ご登録ありがとうございます。" }<br />
                { "これでアカウントの準備が整いました。" }
            </p>

            <p>
                { "さっそくログインしてサービスを" }<br />
                { "ご利用ください。" }
            </p>

            <Link<Route> to={Route::Signin} classes="back-to">
                { "サインイン画面に進む" }
            </Link<Route>>
        </div>
    }
}
