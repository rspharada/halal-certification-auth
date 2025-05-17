use crate::features::auth::commons::validation::validate_email;
use gloo::net::http::Request;
use serde::Serialize;
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Serialize)]
struct PasswordResetRequest {
    email: String,
}

#[derive(Clone)]
pub struct PasswordResetRequestState {
    pub email: UseStateHandle<String>,
    pub email_error: UseStateHandle<Option<String>>,
    pub message: UseStateHandle<String>,
    pub is_valid: bool,
}

#[derive(Clone)]
pub struct PasswordResetRequestHandlers {
    pub on_input: Callback<InputEvent>,
    pub on_submit: Callback<SubmitEvent>,
}

pub struct PasswordResetRequestHook {
    pub state: PasswordResetRequestState,
    pub handlers: PasswordResetRequestHandlers,
}

#[hook]
pub fn use_password_reset_request_state() -> PasswordResetRequestHook {
    let email = use_state(|| "".to_string());
    let email_error = use_state(|| None::<String>);
    let message = use_state(|| "".to_string());

    let is_valid = !email.is_empty() && email_error.is_none();

    let on_input = {
        let email = email.clone();
        let email_error = email_error.clone();

        Callback::from(move |e: InputEvent| {
            let value = e.target_unchecked_into::<HtmlInputElement>().value();
            let error = validate_email(&value);

            email.set(value);
            email_error.set(error);
        })
    };

    let on_submit = {
        let email = email.clone();
        let message = message.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();

            let data = PasswordResetRequest {
                email: email.to_string(),
            };

            let message = message.clone();

            spawn_local(async move {
                let res = Request::post("/api/auth/forgot-password")
                    .header("Content-Type", "application/json")
                    .body(serde_json::to_string(&data).unwrap())
                    .unwrap()
                    .send()
                    .await;

                match res {
                    Ok(resp) if resp.ok() => {
                        message.set("確認コードを送信しました。メールをご確認ください。".into());
                    }
                    Ok(_) => {
                        message
                            .set("送信に失敗しました。メールアドレスを確認してください。".into());
                    }
                    Err(_) => {
                        message.set("ネットワークエラーが発生しました。".into());
                    }
                }
            });
        })
    };

    PasswordResetRequestHook {
        state: PasswordResetRequestState {
            email,
            email_error,
            message,
            is_valid,
        },
        handlers: PasswordResetRequestHandlers {
            on_input,
            on_submit,
        },
    }
}
