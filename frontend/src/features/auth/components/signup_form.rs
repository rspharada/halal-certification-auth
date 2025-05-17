use crate::components::error_message::ErrorMessage;
use crate::components::input_field::InputField;
use crate::components::password_rules::PasswordRules;
use crate::features::auth::hooks::use_signup_state::use_signup_state;
use yew::prelude::*;

/// サインアップフォームコンポーネント
#[function_component(SignupForm)]
pub fn signup_form() -> Html {
    let signup = use_signup_state();
    let state = &*signup.state;

    html! {
        <form class="signup-form" onsubmit={signup.onsubmit.clone()}>
            <h2 class="form-title">{ "サインアップ" }</h2>

            if !state.message.is_empty() {
                <ErrorMessage message={state.message.clone()} class={Some("error-message".to_string())}/>
            }

            <InputField
                input_type={"email".to_string()}
                placeholder={"メールアドレス".to_string()}
                value={state.email.clone()}
                oninput={signup.handle_email_input.clone()}
                class={Some("form-group".to_string())}
                error_message={state.email_error.clone()}
            />

            <InputField
                input_type={"password".to_string()}
                placeholder={"パスワード".to_string()}
                value={state.password.clone()}
                oninput={signup.handle_password_input.clone()}
                class={Some("form-group".to_string())}
                error_message={state.password_error.clone()}
            />

            if state.is_password_first_input {
                <PasswordRules
                    length={state.password_rules.length}
                    number={state.password_rules.number}
                    lowercase={state.password_rules.lowercase}
                    uppercase={state.password_rules.uppercase}
                    symbol={state.password_rules.symbol}
                />
            }

            <InputField
                input_type={"password".to_string()}
                placeholder={"確認用パスワード".to_string()}
                value={state.confirm_password.clone()}
                oninput={signup.handle_confirm_input.clone()}
                class={Some("form-group".to_string())}
                error_message={state.confirm_error.clone()}
            />

            <button type="submit" class="form-submit" disabled={!signup.is_valid}>
                { "登録" }
            </button>
        </form>
    }
}
