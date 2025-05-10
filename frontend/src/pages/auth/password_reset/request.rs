//! request.rs
//!
//! パスワード再発行申請ページ。
//! ユーザーがメールアドレス（またはユーザー名）を入力し、
//! パスワードリセット用の確認コードを受け取る。

use gloo::net::http::Request;
use serde::Serialize;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

/// 再発行申請データ構造体
#[derive(Serialize)]
struct PasswordResetRequest {
    username: String,
}

/// パスワード再発行申請ページ
#[function_component(PasswordResetRequestPage)]
pub fn password_reset_request_page() -> Html {
    let username = use_state(|| "".to_string());
    let message = use_state(|| "".to_string());

    // 入力共通処理
    let handle_input = {
        let username = username.clone();
        Callback::from(move |e: InputEvent| {
            let value = e
                .target_unchecked_into::<web_sys::HtmlInputElement>()
                .value();
            username.set(value);
        })
    };

    // フォーム送信処理
    let onsubmit = {
        let username = username.clone();
        let message = message.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();

            let data = PasswordResetRequest {
                username: username.to_string(),
            };

            let message = message.clone();
            spawn_local(async move {
                let res = Request::post("/auth/api/request-reset")
                    .header("Content-Type", "application/json")
                    .body(serde_json::to_string(&data).unwrap())
                    .unwrap()
                    .send()
                    .await;

                match res {
                    Ok(resp) if resp.ok() => {
                        message.set("確認コードを送信しました。メールをご確認ください。".into())
                    }
                    Ok(_) => {
                        message.set("送信に失敗しました。ユーザー名を確認してください。".into())
                    }
                    Err(_) => message.set("ネットワークエラーが発生しました。".into()),
                }
            });
        })
    };

    html! {
        <form {onsubmit}>
            <h2>{ "パスワード再発行申請" }</h2>

            <input
                type="text"
                placeholder="ユーザー名またはメールアドレス"
                value={(*username).clone()}
                oninput={handle_input}
            />

            <button type="submit">{ "確認コードを送信" }</button>

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
    fn it_renders_password_reset_request_page() {
        let root = document().create_element("div").unwrap();
        document().body().unwrap().append_child(&root).unwrap();

        Renderer::<PasswordResetRequestPage>::with_root(root.into()).render();

        let body = document().body().unwrap().inner_html();
        assert!(
            body.contains("パスワード再発行申請"),
            "ページに 'パスワード再発行申請' が含まれていません"
        );
    }
}
