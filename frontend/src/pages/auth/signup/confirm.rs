//! confirm.rs
//!
//! サインアップ時に送信されたメールの確認コードを入力・検証するページ。
//!
//! ユーザーは受け取った確認コードを入力してアカウントを有効化する。

use gloo::net::http::Request;
use serde::Serialize;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

/// 確認コード送信データの構造体
#[derive(Serialize)]
struct ConfirmData {
    username: String,
    code: String,
}

/// 確認ページ本体
#[function_component(ConfirmPage)]
pub fn confirm_page() -> Html {
    let username = use_state(|| "".to_string());
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
        let username = username.clone();
        let code = code.clone();
        let message = message.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();

            let data = ConfirmData {
                username: username.to_string(),
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
                        "確認に失敗しました。コードまたはユーザー名を確認してください。".into(),
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
    fn it_renders_confirm_page() {
        // DOMの一時ノード作成
        let div = document().create_element("div").unwrap();
        document().body().unwrap().append_child(&div).unwrap();

        // ConfirmPage を描画（実際の DOM に追加）
        Renderer::<ConfirmPage>::with_root(div.into()).render();
    }
}
