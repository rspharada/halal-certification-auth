//! signup.rs
//!
//! このファイルは Yew アプリケーションの「サインアップページ」を定義します。
//!
//! 主な役割：
//! - メールアドレス・パスワード・確認用パスワードの状態管理
//! - パスワード一致チェック
//! - サインアップAPI（POST /api/auth/signup）への非同期送信処理
//! - 成功時に `/auth/mfa/:email` へ遷移（yew_router使用）
//!
//! 関連ファイル：
//! - `routes.rs`: Route::SigninMfa { email } にマッピング

use gloo::net::http::Request;
use serde::Serialize;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::routes::Route;
use crate::types::AuthContext;
use std::rc::Rc;
use yew::prelude::use_context;

#[derive(Serialize)]
struct SignupForm {
    email: String,
    password: String,
}

#[function_component(SignupPage)]
pub fn signup_page() -> Html {
    let auth_ctx = use_context::<AuthContext>();

    let email = use_state(|| "".to_string());
    let password = use_state(|| "".to_string());
    let confirm_password = use_state(|| "".to_string());
    let message = use_state(|| "".to_string());

    let navigator = use_navigator().unwrap();

    let handle_input = |state: UseStateHandle<String>| {
        Callback::from(move |e: InputEvent| {
            let value = e
                .target_unchecked_into::<web_sys::HtmlInputElement>()
                .value();
            state.set(value);
        })
    };

    let onsubmit = {
        let email = email.clone();
        let password = password.clone();
        let confirm_password = confirm_password.clone();
        let message = message.clone();
        let navigator = navigator.clone(); // clone によって Fn を満たす

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();

            if *password != *confirm_password {
                message.set("パスワードが一致しません".into());
                return;
            }

            let form = SignupForm {
                email: email.to_string(),
                password: password.to_string(),
            };

            let message = message.clone();
            let nav = navigator.clone();
            let email_for_redirect = email.to_string();
            let auth_ctx = auth_ctx.clone();

            spawn_local(async move {
                let res = Request::post("/api/auth/signup")
                    .header("Content-Type", "application/json")
                    .body(serde_json::to_string(&form).unwrap())
                    .unwrap()
                    .send()
                    .await;

                match res {
                    Ok(resp) if resp.ok() => {
                        if let Some(auth_ctx) = auth_ctx {
                            auth_ctx.email.set(Rc::new(email_for_redirect.clone()));
                            auth_ctx.session.set(Rc::new("".to_string()));
                        }
                        nav.push(&Route::SignupConfirm);
                    }
                    Ok(_) => message.set("サインアップ失敗...".into()),
                    Err(_) => message.set("ネットワークエラー".into()),
                }
            });
        })
    };

    html! {
        <form {onsubmit}>
            <h2>{ "サインアップ" }</h2>

            <input
                type="email"
                placeholder="メールアドレス"
                value={(*email).clone()}
                oninput={handle_input(email.clone())}
            />

            <input
                type="password"
                placeholder="パスワード"
                value={(*password).clone()}
                oninput={handle_input(password.clone())}
            />

            <input
                type="password"
                placeholder="確認用パスワード"
                value={(*confirm_password).clone()}
                oninput={handle_input(confirm_password.clone())}
            />

            <button type="submit">{ "登録" }</button>

            <p>{ &*message }</p>
        </form>
    }
}
