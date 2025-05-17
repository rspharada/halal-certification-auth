use crate::config::index::get_redirect_url;
use crate::routes::Route;
use crate::types::AuthContext;
use gloo::net::http::Request;
use serde::Serialize;
use std::rc::Rc;
use wasm_bindgen_futures::spawn_local;
use web_sys::{HtmlInputElement, window};
use yew::prelude::*;
use yew_router::prelude::use_navigator;

#[derive(Serialize)]
struct MfaVerificationData {
    email: String,
    code: String,
    session: String,
}

#[allow(dead_code)]
#[derive(Clone)]
pub struct SigninMfaFormState {
    pub email: String,
    pub session: String,
    pub code: UseStateHandle<String>,
    pub message: UseStateHandle<String>,
    pub is_valid: bool,
}

#[derive(Clone)]
pub struct SigninMfaHandlers {
    pub on_input: Callback<InputEvent>,
    pub on_submit: Callback<SubmitEvent>,
}

pub struct SigninMfaState {
    pub state: SigninMfaFormState,
    pub handlers: SigninMfaHandlers,
}

#[hook]
pub fn use_signin_mfa_state() -> SigninMfaState {
    let auth_ctx = use_context::<AuthContext>().expect("AuthContext not found");

    let email = (*auth_ctx.email).clone();
    let session = (*auth_ctx.session).clone();
    let code = use_state(|| "".to_string());
    let message = use_state(|| "".to_string());
    let navigator = Rc::new(use_navigator().expect("navigator not found"));

    let is_valid = {
        let code_str = &*code;
        code_str.len() == 6 && code_str.chars().all(|c| c.is_ascii_digit())
    };

    {
        let navigator = navigator.clone();
        let email = email.clone();
        let session = session.clone();

        use_effect_with((email.clone(), session.clone()), move |(email, session)| {
            if email.is_empty() || session.is_empty() {
                navigator.push(&Route::Signin);
            }
            || ()
        });
    }

    let on_input = {
        let code = code.clone();
        Callback::from(move |e: InputEvent| {
            let value = e.target_unchecked_into::<HtmlInputElement>().value();
            code.set(value);
        })
    };

    let on_submit = {
        let code = code.clone();
        let message = message.clone();
        let email = email.clone();
        let session = session.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();

            let payload = MfaVerificationData {
                email: email.as_ref().to_string(),
                code: (*code).clone(),
                session: session.as_ref().to_string(),
            };

            let message = message.clone();
            spawn_local(async move {
                let res = Request::post("/api/auth/mfa")
                    .header("Content-Type", "application/json")
                    .body(serde_json::to_string(&payload).unwrap())
                    .unwrap()
                    .send()
                    .await;

                match res {
                    Ok(resp) if resp.ok() => {
                        if let Some(win) = window() {
                            let _ = win.location().set_href(&get_redirect_url());
                        }
                    }
                    Ok(_) => message.set("コードが正しくありません。".into()),
                    Err(_) => message.set("ネットワークエラーが発生しました。".into()),
                }
            });
        })
    };

    SigninMfaState {
        state: SigninMfaFormState {
            email: email.to_string(),
            session: session.to_string(),
            code,
            message,
            is_valid,
        },
        handlers: SigninMfaHandlers {
            on_input,
            on_submit,
        },
    }
}
