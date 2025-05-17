//! password_reset_request_form.rs
//!
//! パスワード再発行申請フォームのUIコンポーネント。

use crate::components::error_message::ErrorMessage;
use crate::components::input_field::InputField;
use crate::features::auth::hooks::use_password_reset_request_state::use_password_reset_request_state;
use crate::routes::Route;
use yew::prelude::*;
use yew_router::components::Link;

/// パスワード再発行申請フォーム本体
#[function_component(PasswordResetRequestForm)]
pub fn password_reset_request_form() -> Html {
    let reset = use_password_reset_request_state();

    html! {
        <form onsubmit={reset.handlers.on_submit.clone()} class="signup-form">
            <h2 class="form-title">{ "パスワードを忘れた方" }</h2>

            if !reset.state.message.is_empty() {
                <ErrorMessage
                    message={(*reset.state.message).clone()}
                    class={Some("error-message".to_string())}
                />
            }

            <InputField
                input_type="text"
                placeholder="メールアドレス"
                value={(*reset.state.email).clone()}
                oninput={reset.handlers.on_input.clone()}
                class={Some("form-group".to_string())}
                error_message={(*reset.state.email_error).clone()}
            />

            <button
                type="submit"
                class="form-submit"
                disabled={!reset.state.is_valid}
            >
                { "確認コードを送信" }
            </button>

            <div class="form-link-right">
                <Link<Route> to={Route::Signin} classes="back-to">
                    { "サインインに戻る" }
                </Link<Route>>
            </div>
        </form>
    }
}
