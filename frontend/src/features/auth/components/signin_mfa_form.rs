use crate::components::error_message::ErrorMessage;
use crate::components::input_field::InputField;
use crate::features::auth::hooks::use_signin_mfa_state::use_signin_mfa_state;
use crate::routes::Route;
use yew::prelude::*;
use yew_router::components::Link;

#[function_component(SigninMfaForm)]
pub fn signin_mfa_form() -> Html {
    let signin_mfa = use_signin_mfa_state();

    html! {
        <form onsubmit={signin_mfa.handlers.on_submit.clone()} class="signup-form">
            <h2 class="form-title">{ "確認コードを入力" }</h2>

            if !signin_mfa.state.message.is_empty() {
                <ErrorMessage
                    message={(*signin_mfa.state.message).clone()}
                    class={Some("error-message".to_string())}
                />
            }

            <InputField
                input_type="text"
                placeholder="確認コード（6桁）"
                value={(*signin_mfa.state.code).clone()}
                oninput={signin_mfa.handlers.on_input.clone()}
                class={Some("form-group".to_string())}
            />

            <button
                type="submit"
                class="form-submit"
                disabled={!signin_mfa.state.is_valid}
            >
                { "確認" }
            </button>

            <div class="form-link-right">
                <Link<Route> to={Route::Signin} classes="back-to">
                    { "サインインに戻る" }
                </Link<Route>>
            </div>
        </form>
    }
}
