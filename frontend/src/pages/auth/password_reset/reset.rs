//! reset.rs
//!
//! パスワード再設定画面。
//! MFA認証後のユーザーが新しいパスワードを入力し、確定させる。

use gloo::net::http::Request;
use serde::Serialize;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

/// パスワードリセット用の送信データ
#[derive(Serialize)]
struct PasswordResetData {
    username: String,
    new_password: String,
}

/// パスワード変更ページ本体
#[function_component(PasswordResetPage)]
pub fn password_reset_page() -> Html {
    let username = use_state(|| "".to_string());
    let new_password = use_state(|| "".to_string());
    let confirm_password = use_state(|| "".to_string());
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

    // フォーム送信処理
    let onsubmit = {
        let username = username.clone();
        let new_password = new_password.clone();
        let confirm_password = confirm_password.clone();
        let message = message.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();

            if *new_password != *confirm_password {
                message.set("パスワードが一致しません。".into());
                return;
            }

            let payload = PasswordResetData {
                username: username.to_string(),
                new_password: new_password.to_string(),
            };

            let message = message.clone();
            spawn_local(async move {
                let res = Request::post("/api/auth/forgot-password/confirm")
                    .header("Content-Type", "application/json")
                    .body(serde_json::to_string(&payload).unwrap())
                    .unwrap()
                    .send()
                    .await;

                match res {
                    Ok(resp) if resp.ok() => {
                        message
                            .set("パスワードの変更が完了しました。ログインしてください。".into());
                        // TODO: リダイレクト to /signin
                    }
                    Ok(_) => message.set("パスワード変更に失敗しました。".into()),
                    Err(_) => message.set("ネットワークエラーが発生しました。".into()),
                }
            });
        })
    };

    html! {
        <form {onsubmit}>
            <h2>{ "パスワード再設定" }</h2>

            <input
                type="text"
                placeholder="ユーザー名"
                value={(*username).clone()}
                oninput={handle_input(username.clone())}
            />

            <input
                type="password"
                placeholder="新しいパスワード"
                value={(*new_password).clone()}
                oninput={handle_input(new_password.clone())}
            />

            <input
                type="password"
                placeholder="確認用パスワード"
                value={(*confirm_password).clone()}
                oninput={handle_input(confirm_password.clone())}
            />

            <button type="submit">{ "パスワードを変更" }</button>

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
    fn it_renders_password_reset_page() {
        let root = document().create_element("div").unwrap();
        document().body().unwrap().append_child(&root).unwrap();

        Renderer::<PasswordResetPage>::with_root(root.into()).render();

        let body = document().body().unwrap().inner_html();
        assert!(
            body.contains("パスワード再設定"),
            "ページに 'パスワード再設定' が含まれていません"
        );
    }
}
