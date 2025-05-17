//! mfa.rs
//!
//! パスワード再発行フローにおけるMFA確認コード入力画面。

use gloo::net::http::Request;
use serde::Serialize;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

/// フォーム送信用データ構造体
#[derive(Serialize)]
struct PasswordResetMfaData {
    username: String,
    code: String,
}

/// PasswordResetMfaPage: パスワード再発行におけるMFA認証画面
#[function_component(PasswordResetMfaPage)]
pub fn password_reset_mfa_page() -> Html {
    let username = use_state(|| "".to_string());
    let code = use_state(|| "".to_string());
    let message = use_state(|| "".to_string());

    // 共通 input ハンドラ
    let handle_input = |state: UseStateHandle<String>| {
        Callback::from(move |e: InputEvent| {
            let value = e
                .target_unchecked_into::<web_sys::HtmlInputElement>()
                .value();
            state.set(value);
        })
    };

    let onsubmit = {
        let username = username.clone();
        let code = code.clone();
        let message = message.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();

            let payload = PasswordResetMfaData {
                username: username.to_string(),
                code: code.to_string(),
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
                        message.set("認証成功！パスワード変更画面へ進めます。".into());
                        // TODO: use_navigator で /reset にリダイレクト
                    }
                    Ok(_) => message.set("認証に失敗しました。コードを再確認してください。".into()),
                    Err(_) => message.set("ネットワークエラーが発生しました。".into()),
                }
            });
        })
    };

    html! {
        <form {onsubmit}>
            <h2>{ "MFA確認コード入力（パスワード再発行）" }</h2>

            <input
                type="text"
                placeholder="ユーザー名"
                value={(*username).clone()}
                oninput={handle_input(username.clone())}
            />

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
    use wasm_bindgen_test::*;
    use yew::Renderer;

    wasm_bindgen_test_configure!(run_in_browser);

    #[allow(dead_code)]
    #[wasm_bindgen_test]
    fn it_renders_password_reset_mfa_page() {
        let root = document().create_element("div").unwrap();
        document().body().unwrap().append_child(&root).unwrap();

        Renderer::<PasswordResetMfaPage>::with_root(root.into()).render();

        let body = document().body().unwrap().inner_html();
        assert!(
            body.contains("MFA確認コード入力"),
            "ページに 'MFA確認コード入力' が含まれていません"
        );
    }
}
