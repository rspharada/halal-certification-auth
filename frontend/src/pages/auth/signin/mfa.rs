//! mfa.rs
//!
//! サインイン後のメール認証コード入力画面。
//! ユーザーは受信したMFAコードを入力して、二段階認証を完了する。

use crate::types::AuthContext;
use gloo::net::http::Request;
use serde::Serialize;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

/// MFAコード送信用データ構造体
#[derive(Serialize)]
struct MfaVerificationData {
    email: String,
    code: String,
    session: String,
}

/// サインイン後のMFAページ本体
#[function_component(SigninMfaPage)]
pub fn signin_mfa_page() -> Html {
    let auth_ctx = use_context::<AuthContext>().expect("AuthContext not found");
    let email = use_state(|| (*auth_ctx.email).clone());
    let session = use_state(|| (*auth_ctx.session).clone());
    let code = use_state(|| "".to_string());
    let message = use_state(|| "".to_string());

    // 入力処理の共通化
    let handle_input = |state: UseStateHandle<String>| {
        Callback::from(move |e: InputEvent| {
            let value = e
                .target_unchecked_into::<web_sys::HtmlInputElement>()
                .value();
            state.set(value);
        })
    };

    // フォーム送信処理
    let onsubmit = {
        let email = email.clone();
        let code = code.clone();
        let session = session.clone();
        let message = message.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();

            let payload = MfaVerificationData {
                email: email.to_string(),
                code: code.to_string(),
                session: session.to_string(),
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
                        message.set("MFA認証に成功しました！".into());
                        // TODO: 認証後のリダイレクトをここで実装する（use_navigatorなど）
                    }
                    Ok(_) => message.set("コードが正しくありません。".into()),
                    Err(_) => message.set("ネットワークエラーが発生しました。".into()),
                }
            });
        })
    };

    html! {
        <form {onsubmit}>
            <h2>{ "MFAコード入力" }</h2>

            <input
                type="text"
                placeholder="確認コード（6桁）"
                value={(*code).clone()}
                oninput={handle_input(code.clone())}
            />

            <button type="submit">{ "確認" }</button>

            <p>{ &*message }</p>
        </form>
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gloo::utils::document;
    use std::rc::Rc;
    use wasm_bindgen_test::*;
    use yew::Renderer;

    wasm_bindgen_test_configure!(run_in_browser);

    #[allow(dead_code)]
    #[wasm_bindgen_test]
    fn it_renders_signin_mfa_page() {
        use yew::functional::*;

        #[function_component(TestWrapper)]
        fn test_wrapper() -> Html {
            let email = use_state(|| Rc::new("test@example.com".to_string()));
            let session = use_state(|| Rc::new("dummy-session-id".to_string()));

            let auth_ctx = AuthContext { email, session };

            html! {
                <ContextProvider<AuthContext> context={auth_ctx}>
                    <SigninMfaPage />
                </ContextProvider<AuthContext>>
            }
        }

        let div = document().create_element("div").unwrap();
        document().body().unwrap().append_child(&div).unwrap();

        Renderer::<TestWrapper>::with_root(div.into()).render();

        let body = document().body().unwrap().inner_html();
        assert!(
            body.contains("MFAコード入力"),
            "ページに 'MFAコード入力' が含まれていません"
        );
    }
}
