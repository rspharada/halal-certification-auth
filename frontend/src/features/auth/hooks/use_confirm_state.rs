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
struct ConfirmData {
    email: String,
    code: String,
}

#[derive(Clone, PartialEq, Debug, Default)]
pub struct ConfirmFormState {
    pub code: String,
    pub message: String,
}

pub struct ConfirmHandlers {
    pub on_input: Callback<InputEvent>,
    pub on_submit: Callback<SubmitEvent>,
    pub on_resend: Callback<MouseEvent>,
}

pub struct ConfirmForm {
    pub state: UseStateHandle<ConfirmFormState>,
    pub handlers: ConfirmHandlers,
    pub is_valid: bool,
}

#[hook]
pub fn use_confirm_state() -> ConfirmForm {
    let auth_ctx = use_context::<AuthContext>().expect("AuthContext not found");
    let navigator = Rc::new(use_navigator().expect("navigator not found"));

    let state = use_state(ConfirmFormState::default);
    let email = auth_ctx.email.clone();

    // 未ログイン時リダイレクト
    {
        let navigator = navigator.clone();
        let email = email.clone();
        use_effect_with(email, move |email| {
            if email.is_empty() {
                navigator.push(&Route::Signin);
            }
            || ()
        });
    }

    // 入力ハンドラ
    let on_input = {
        let state = state.clone();
        Callback::from(move |e: InputEvent| {
            let value = e.target_unchecked_into::<HtmlInputElement>().value();
            state.set(ConfirmFormState {
                code: value,
                message: state.message.clone(),
            });
        })
    };

    // 送信ハンドラ
    let on_submit = {
        let state = state.clone();
        let navigator = Rc::clone(&navigator);
        let email = email.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            let state = state.clone();
            let code = state.code.clone();
            let email = email.clone();
            let navigator = Rc::clone(&navigator);

            let data = ConfirmData {
                email: email.to_string(),
                code: code.to_string(),
            };

            spawn_local(async move {
                let res = Request::post("/api/auth/signup/confirm")
                    .header("Content-Type", "application/json")
                    .body(serde_json::to_string(&data).unwrap())
                    .unwrap()
                    .send()
                    .await;

                match res {
                    Ok(resp) if resp.ok() => {
                        navigator.push(&Route::SignupConfirmComplete);
                    }
                    Ok(_) => state.set(ConfirmFormState {
                        code,
                        message:
                            "確認に失敗しました。コードまたはメールアドレスを確認してください。"
                                .into(),
                    }),
                    Err(_) => state.set(ConfirmFormState {
                        code,
                        message: "通信エラーが発生しました。".into(),
                    }),
                }
            });
        })
    };

    // 再送信ハンドラ
    let on_resend = {
        let state = state.clone();
        let email = email.clone();

        Callback::from(move |_| {
            let email = email.clone();
            let state = state.clone();

            spawn_local(async move {
                let res = Request::post("/api/auth/resend-code")
                    .header("Content-Type", "application/json")
                    .body(format!("{{\"email\":\"{}\"}}", email.as_ref()))
                    .unwrap()
                    .send()
                    .await;

                match res {
                    Ok(resp) if resp.ok() => state.set(ConfirmFormState {
                        code: state.code.clone(),
                        message: "確認コードを再送信しました。".into(),
                    }),
                    Ok(_) => state.set(ConfirmFormState {
                        code: state.code.clone(),
                        message: "再送信に失敗しました。".into(),
                    }),
                    Err(_) => state.set(ConfirmFormState {
                        code: state.code.clone(),
                        message: "通信エラーが発生しました。".into(),
                    }),
                }
            });
        })
    };

    let is_valid = !state.code.is_empty();

    ConfirmForm {
        state,
        handlers: ConfirmHandlers {
            on_input,
            on_submit,
            on_resend,
        },
        is_valid,
    }
}
