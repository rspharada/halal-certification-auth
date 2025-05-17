use crate::routes::Route;
use crate::types::AuthContext;
use gloo::net::http::Request;
use serde::Serialize;
use std::rc::Rc;
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Serialize)]
struct SigninFormData {
    email: String,
    password: String,
}

#[derive(Clone, PartialEq, Default)]
pub struct SigninFormState {
    pub email: String,
    pub password: String,
    pub message: String,
}

pub struct SigninState {
    pub state: UseStateHandle<SigninFormState>,
    pub handle_email_input: Callback<InputEvent>,
    pub handle_password_input: Callback<InputEvent>,
    pub onsubmit: Callback<SubmitEvent>,
    pub is_valid: bool,
}

#[hook]
pub fn use_signin_state() -> SigninState {
    let state = use_state(SigninFormState::default);
    let navigator = Rc::new(use_navigator().expect("navigator not found"));
    let auth_ctx = use_context::<AuthContext>();

    let handle_email_input = {
        let state = state.clone();
        Callback::from(move |e: InputEvent| {
            let value = e.target_unchecked_into::<HtmlInputElement>().value();
            state.set(SigninFormState {
                email: value,
                ..(*state).clone()
            });
        })
    };

    let handle_password_input = {
        let state = state.clone();
        Callback::from(move |e: InputEvent| {
            let value = e.target_unchecked_into::<HtmlInputElement>().value();
            state.set(SigninFormState {
                password: value,
                ..(*state).clone()
            });
        })
    };

    let onsubmit = {
        let state = state.clone();
        let navigator = Rc::clone(&navigator);
        let auth_ctx = auth_ctx.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();

            let form = SigninFormData {
                email: state.email.clone(),
                password: state.password.clone(),
            };

            let message = state.clone();
            let email_for_redirect = state.email.clone();
            let navigator = Rc::clone(&navigator);
            let auth_ctx = auth_ctx.clone();

            spawn_local(async move {
                let res = Request::post("/api/auth/signin")
                    .header("Content-Type", "application/json")
                    .body(serde_json::to_string(&form).unwrap())
                    .unwrap()
                    .send()
                    .await;

                match res {
                    Ok(resp) if resp.ok() => {
                        if let Ok(json) = resp.json::<serde_json::Value>().await {
                            let session = json
                                .get("session")
                                .and_then(|v| v.as_str())
                                .unwrap_or("")
                                .to_string();

                            if let Some(ctx) = auth_ctx {
                                ctx.email.set(Rc::new(email_for_redirect.clone()));
                                ctx.session.set(Rc::new(session));
                            }
                            navigator.push(&Route::SigninMfa);
                        } else {
                            message.set(SigninFormState {
                                message:
                                    "サインイン成功しましたが、レスポンスの解析に失敗しました。"
                                        .into(),
                                ..(*message).clone()
                            });
                        }
                    }
                    Ok(resp) if resp.status() == 403 => {
                        if let Some(ctx) = auth_ctx {
                            ctx.email.set(Rc::new(email_for_redirect.clone()));
                            ctx.session.set(Rc::new("".to_string()));
                        }
                        navigator.push(&Route::SignupConfirm);
                    }
                    Ok(_) => message.set(SigninFormState {
                        message: "メールまたはパスワードが違います".into(),
                        ..(*message).clone()
                    }),
                    Err(_) => message.set(SigninFormState {
                        message: "ネットワークエラー".into(),
                        ..(*message).clone()
                    }),
                }
            });
        })
    };

    let is_valid = !state.email.is_empty() && !state.password.is_empty();

    SigninState {
        state,
        handle_email_input,
        handle_password_input,
        onsubmit,
        is_valid,
    }
}
