//! signup.rs
//!
//! このファイルは Yew アプリケーションの「サインアップページ」を定義します。
//!
//! 主な役割：
//! - 状態（ユーザー名・メール・パスワード）の管理
//! - フォーム入力の双方向バインディング
//! - サインアップAPI（POST /auth/api/signup）への非同期送信処理
//! - 成功・失敗時のメッセージ表示
//!
//! 関連ファイル：
//! - `types.rs`: フォームデータの構造体定義を分離する場合に使用
//! - `routes.rs`: このページに対応するURL `/signup` をマッピング

use gloo::net::http::Request;
use serde::Serialize;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

/// サインアップフォームの送信データ構造
#[derive(Serialize)]
struct SignupForm {
    username: String,
    email: String,
    password: String,
}

/// サインアップページ本体（関数型コンポーネント）
#[function_component(SignupPage)]
pub fn signup_page() -> Html {
    // 入力フィールドごとの状態管理（Yewのuse_stateを使用）
    let username = use_state(|| "".to_string());
    let email = use_state(|| "".to_string());
    let password = use_state(|| "".to_string());
    let message = use_state(|| "".to_string()); // 結果メッセージ用

    // 入力イベントハンドラを共通化するクロージャ（再利用性のため）
    let handle_input = |state: UseStateHandle<String>| {
        Callback::from(move |e: InputEvent| {
            let value = e
                .target_unchecked_into::<web_sys::HtmlInputElement>()
                .value();
            state.set(value); // 入力値を state に反映
        })
    };

    // フォーム送信時の非同期処理を定義（POSTリクエスト）
    let onsubmit = {
        // 状態をクロージャ内にmoveで取り込む
        let username = username.clone();
        let email = email.clone();
        let password = password.clone();
        let message = message.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default(); // ページリロードを防止

            // 送信用データの作成
            let form = SignupForm {
                username: username.to_string(),
                email: email.to_string(),
                password: password.to_string(),
            };

            let message = message.clone();
            spawn_local(async move {
                // 非同期HTTP POSTリクエスト
                let res = Request::post("/auth/api/signup")
                    .header("Content-Type", "application/json")
                    .body(serde_json::to_string(&form).unwrap()) // JSONに変換
                    .unwrap() // RequestBuilderのResultをunwrap
                    .send()
                    .await;

                // 応答の内容に応じてメッセージを表示
                match res {
                    Ok(resp) if resp.ok() => message.set("サインアップ成功！".into()),
                    Ok(_) => message.set("サインアップ失敗...".into()),
                    Err(_) => message.set("ネットワークエラー".into()),
                }
            });
        })
    };

    // 実際のHTMLフォームUIの構築
    html! {
        <form {onsubmit}>
            <h2>{ "サインアップ" }</h2>

            <input
                type="text"
                placeholder="ユーザー名"
                value={(*username).clone()}
                oninput={handle_input(username.clone())}
            />

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

            <button type="submit">{ "登録" }</button>

            // 結果メッセージの表示
            <p>{ &*message }</p>
        </form>
    }
}
