// src/features/auth/components/signup_confirm_form.rs

use crate::components::error_message::ErrorMessage;
use crate::components::input_field::InputField;
use crate::routes::Route;
use yew::prelude::*;
use yew_router::components::Link;

#[derive(Properties, PartialEq)]
pub struct SignupConfirmFormProps {
    pub code: String,
    pub message: String,
    pub on_input: Callback<InputEvent>,
    pub on_submit: Callback<SubmitEvent>,
    pub on_resend: Callback<MouseEvent>,
    pub is_valid: bool,
}

#[function_component(SignupConfirmForm)]
pub fn signup_confirm_form(props: &SignupConfirmFormProps) -> Html {
    html! {
        <form onsubmit={props.on_submit.clone()} class="confirm-form">
            <h2 class="form-title">{ "アカウントを有効化する" }</h2>

            if !props.message.is_empty() {
                <ErrorMessage message={props.message.clone()} class={Some("error-message".to_string())} />
            }

            <InputField
                input_type="text"
                placeholder="確認コード"
                value={props.code.clone()}
                oninput={props.on_input.clone()}
                class={Some("form-group".to_string())}
            />

            <div class="form-actions">
                <button type="submit" class="form-submit" disabled={!props.is_valid}>{ "確認" }</button>
                <button type="button" class="form-resend" onclick={props.on_resend.clone()}>{ "確認コード再送信" }</button>
            </div>

            <div class="form-link-right">
                <Link<Route> to={Route::Signin} classes="back-to-signin">
                    { "サインイン画面に戻る" }
                </Link<Route>>
            </div>
        </form>
    }
}
