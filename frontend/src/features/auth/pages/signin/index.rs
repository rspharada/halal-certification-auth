//! signin.rs
//!
//! このファイルは Yew アプリケーションの「サインインページ」を定義します。
//!
//! 主な役割：
//! - メールアドレスとパスワードの状態管理
//! - 入力フォームと状態の双方向バインディング
//! - サインインAPI（POST /api/auth/signin）への非同期リクエスト送信
//! - 成功・失敗時のメッセージ表示
//!
//! 関連ファイル：
//! - `routes.rs`: このページに対応するURL `/signin` をマッピング
//! - `types.rs`: フォームデータ構造体定義を分離する場合に使用可能

use crate::routes::Route;
use crate::types::AuthContext;
use gloo::net::http::Request;
use serde::Serialize;
use std::rc::Rc;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_router::prelude::use_navigator;

/// サインインフォームの送信データ構造
#[derive(Serialize)]
struct SigninForm {
    email: String,
    password: String,
}

/// サインインページ本体（関数型コンポーネント）
#[function_component(SigninPage)]
pub fn signin_page() -> Html {
    // 状態管理
    let email = use_state(|| "".to_string());
    let password = use_state(|| "".to_string());
    let message = use_state(|| "".to_string());

    let nav = use_navigator().unwrap();
    let auth_ctx = use_context::<AuthContext>();

    // 入力フィールド用の共通イベントハンドラ
    let handle_input = |state: UseStateHandle<String>| {
        Callback::from(move |e: InputEvent| {
            let value = e
                .target_unchecked_into::<web_sys::HtmlInputElement>()
                .value();
            state.set(value);
        })
    };

    // フォーム送信時の処理（非同期POST）
    let nav_clone = nav.clone();
    let auth_ctx_clone = auth_ctx.clone();

    let onsubmit = {
        let email = email.clone();
        let password = password.clone();
        let message = message.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();

            let form = SigninForm {
                email: email.to_string(),
                password: password.to_string(),
            };

            let message = message.clone();
            let email_for_redirect = email.to_string();

            let auth_ctx_inner = auth_ctx_clone.clone();
            let nav_inner = nav_clone.clone();

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

                            if let Some(auth_ctx) = auth_ctx_inner {
                                auth_ctx.email.set(Rc::new(email_for_redirect.clone()));
                                auth_ctx.session.set(Rc::new(session.clone()));
                            }
                            nav_inner.push(&Route::SigninMfa);
                        } else {
                            message.set(
                                "サインイン成功しましたが、レスポンスの解析に失敗しました。".into(),
                            );
                        }
                    }
                    Ok(resp) if resp.status() == 403 => {
                        if let Some(auth_ctx) = auth_ctx_inner {
                            auth_ctx.email.set(Rc::new(email_for_redirect.clone()));
                            auth_ctx.session.set(Rc::new("".to_string()));
                        }
                        nav_inner.push(&Route::SignupConfirm);
                    }
                    Ok(_) => message.set("認証失敗。メールまたはパスワードが違います".into()),
                    Err(_) => message.set("ネットワークエラー".into()),
                }
            });
        })
    };

    html! {
        <form {onsubmit}>
            <h2>{ "サインイン" }</h2>

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

            <button type="submit">{ "ログイン" }</button>

            <p>{ &*message }</p>
        </form>
    }
}

#[cfg(test)]
mod tests {
    use super::*; // これがない場合は追加
    use gloo::utils::document;
    use wasm_bindgen_test::*;
    use yew::Renderer;

    wasm_bindgen_test_configure!(run_in_browser);

    #[allow(dead_code)]
    #[wasm_bindgen_test]
    fn it_renders_signin_page() {
        let root = document().create_element("div").unwrap();
        document().body().unwrap().append_child(&root).unwrap();

        Renderer::<SigninPage>::with_root(root.into()).render();

        let body_html = document().body().unwrap().inner_html();
        assert!(
            body_html.contains("サインイン"),
            "ページに 'サインイン' が含まれていません。\nHTML内容:\n{}",
            body_html
        );
    }
}
