//! confirm.rs
//!
//! サインアップ時に送信されたメールの確認コードを入力・検証するページ。
//!
//! ユーザーは受け取った確認コードを入力してアカウントを有効化する。

use crate::types::AuthContext;
use gloo::net::http::Request;
use serde::Serialize;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

/// 確認コード送信データの構造体
#[derive(Serialize)]
struct ConfirmData {
    email: String,
    code: String,
}

/// 確認ページ本体
#[function_component(SignupConfirmPage)]
pub fn confirm_page() -> Html {
    let auth_ctx = use_context::<AuthContext>().expect("AuthContext not found");
    let email = auth_ctx.email.clone();
    let code = use_state(|| "".to_string());
    let message = use_state(|| "".to_string());

    // 入力共通処理
    let handle_input = |state: UseStateHandle<String>| {
        Callback::from(move |e: InputEvent| {
            let value = e
                .target_unchecked_into::<web_sys::HtmlInputElement>()
                .value();
            state.set(value);
        })
    };

    // フォーム送信時の非同期処理
    let onsubmit = {
        let email = email.clone();
        let code = code.clone();
        let message = message.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();

            let data = ConfirmData {
                email: email.to_string(),
                code: code.to_string(),
            };

            let message = message.clone();
            spawn_local(async move {
                let res = Request::post("/api/auth/signup/confirm")
                    .header("Content-Type", "application/json")
                    .body(serde_json::to_string(&data).unwrap())
                    .unwrap()
                    .send()
                    .await;

                match res {
                    Ok(resp) if resp.ok() => message.set("確認に成功しました！".into()),
                    Ok(_) => message.set(
                        "確認に失敗しました。コードまたはメールアドレスを確認してください。".into(),
                    ),
                    Err(_) => message.set("通信エラーが発生しました。".into()),
                }
            });
        })
    };

    html! {
        <form {onsubmit}>
            <h2>{ "アカウント確認コード入力" }</h2>

            <input
                type="text"
                placeholder="確認コード"
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
    fn it_renders_confirm_page() {
        let div = document().create_element("div").unwrap();
        document().body().unwrap().append_child(&div).unwrap();

        // yew::Renderer expects a component that implements `Component`, not a ContextProvider.
        // So we use an intermediate wrapper component instead.
        #[function_component(TestWrapper)]
        fn test_wrapper() -> Html {
            let email = use_state_eq(|| Rc::new("test@example.com".to_string()));
            let session = use_state_eq(|| Rc::new("".to_string()));
            let ctx = AuthContext { email, session };
            html! {
                <ContextProvider<AuthContext> context={ctx}>
                    <SignupConfirmPage />
                </ContextProvider<AuthContext>>
            }
        }

        Renderer::<TestWrapper>::with_root(div.into()).render();

        let body = document().body().unwrap().inner_html();
        assert!(
            body.contains("確認コード"),
            "ページに '確認コード' が含まれていません"
        );
    }
}
