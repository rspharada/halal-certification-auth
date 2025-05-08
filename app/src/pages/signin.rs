//! signin.rs
//!
//! このファイルは Yew アプリケーションの「サインインページ」を定義します。
//!
//! 主な役割：
//! - メールアドレスとパスワードの状態管理
//! - 入力フォームと状態の双方向バインディング
//! - サインインAPI（POST /auth/api/signin）への非同期リクエスト送信
//! - 成功・失敗時のメッセージ表示
//!
//! 関連ファイル：
//! - `routes.rs`: このページに対応するURL `/signin` をマッピング
//! - `types.rs`: フォームデータ構造体定義を分離する場合に使用可能

use gloo::net::http::Request;
use serde::Serialize;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

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
            spawn_local(async move {
                let res = Request::post("/auth/api/signin")
                    .header("Content-Type", "application/json")
                    .body(serde_json::to_string(&form).unwrap())
                    .unwrap()
                    .send()
                    .await;

                match res {
                    Ok(resp) if resp.ok() => message.set("サインイン成功！".into()),
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
