//! signin_form.rs
//!
//! サインイン画面のフォーム部品

use crate::components::error_message::ErrorMessage;
use crate::components::input_field::InputField;
use crate::features::auth::hooks::use_signin_state::use_signin_state;
use crate::routes::Route;
use yew::prelude::*;
use yew_router::components::Link;

#[function_component(SigninForm)]
pub fn signin_form() -> Html {
    let signin = use_signin_state();

    html! {
        <form onsubmit={signin.onsubmit.clone()} class="signup-form">
            <h2 class="form-title">{ "サインイン" }</h2>

            if !signin.state.message.is_empty() {
                <ErrorMessage message={signin.state.message.clone()} class={Some("error-message".to_string())} />
            }

            <InputField
                input_type="email"
                placeholder="メールアドレス"
                value={signin.state.email.clone()}
                oninput={signin.handle_email_input.clone()}
                class={Some("form-group".to_string())}
            />

            <InputField
                input_type="password"
                placeholder="パスワード"
                value={signin.state.password.clone()}
                oninput={signin.handle_password_input.clone()}
                class={Some("form-group".to_string())}
            />

            <button type="submit" class="form-submit" disabled={!signin.is_valid}>
                { "ログイン" }
            </button>

            <div class="form-links">
                <Link<Route> to={Route::Signup} classes="form-link">
                    { "アカウントをお持ちでない方はこちら" }
                </Link<Route>>
                <Link<Route> to={Route::PasswordResetRequest} classes="form-link">
                    { "パスワードを忘れた方はこちら" }
                </Link<Route>>
            </div>
        </form>
    }
}
