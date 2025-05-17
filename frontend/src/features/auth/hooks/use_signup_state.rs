// バリデーション関数とルーティング、コンテキスト、ユーティリティをインポート
use crate::features::auth::commons::validation::{
    validate_confirm, validate_email, validate_password, validate_signup,
};
use crate::routes::Route;
use crate::types::AuthContext;
use crate::utils::{PasswordRulesState, evaluate_password_rules};
use gloo::net::http::Request;
use serde::Serialize;
use std::rc::Rc;
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::prelude::*;

// API送信時に使う構造体（シリアライズされてJSONとして送信される）
#[derive(Serialize)]
struct SignupForm {
    email: String,
    password: String,
}

// ユーザーの入力や状態（エラーメッセージ、UIの補助など）を保持する構造体
#[derive(Clone, PartialEq)]
pub struct SignupFormState {
    pub email: String,
    pub password: String,
    pub confirm_password: String,
    pub email_error: Option<String>,
    pub password_error: Option<String>,
    pub confirm_error: Option<String>,
    pub password_rules: PasswordRulesState,
    pub is_password_first_input: bool,
    pub submitted: bool,
    pub message: String,
}

// 初期状態の実装
impl Default for SignupFormState {
    fn default() -> Self {
        Self {
            email: "".into(),
            password: "".into(),
            confirm_password: "".into(),
            email_error: None,
            password_error: None,
            confirm_error: None,
            password_rules: PasswordRulesState::default(),
            is_password_first_input: false,
            submitted: false,
            message: "".into(),
        }
    }
}

// フックから返す状態とハンドラ類をまとめた構造体
pub struct SignupState {
    pub state: UseStateHandle<SignupFormState>,
    pub handle_email_input: Callback<InputEvent>,
    pub handle_password_input: Callback<InputEvent>,
    pub handle_confirm_input: Callback<InputEvent>,
    pub onsubmit: Callback<SubmitEvent>,
    pub is_valid: bool,
}

// カスタムフック：サインアップフォームの状態と処理を定義
#[hook]
pub fn use_signup_state() -> SignupState {
    // サインアップ後のグローバルな認証コンテキストを取得
    let auth_ctx = use_context::<AuthContext>();

    // ページ遷移用のナビゲータを取得
    let navigator = Rc::new(use_navigator().expect("navigator not found"));

    // ユーザー入力やエラー、UI補助状態を格納するステートフック
    let state = use_state(SignupFormState::default);

    // メールアドレス入力時の処理
    let handle_email_input = {
        let state = state.clone();
        Callback::from(move |e: InputEvent| {
            let input = e.target_unchecked_into::<HtmlInputElement>().value();
            // バリデーションと入力更新
            state.set(SignupFormState {
                email: input.clone(),
                email_error: validate_email(&input),
                ..(*state).clone()
            });
        })
    };

    // パスワード入力時の処理
    let handle_password_input = {
        let state = state.clone();
        Callback::from(move |e: InputEvent| {
            let input = e.target_unchecked_into::<HtmlInputElement>().value();
            let mut new_state = (*state).clone();
            new_state.password = input.clone();
            new_state.password_rules = evaluate_password_rules(&input); // ルールに合っているかのチェック
            new_state.is_password_first_input = true;
            new_state.confirm_error = validate_confirm(&input, &new_state.confirm_password);
            new_state.password_error = validate_password(&input);
            state.set(new_state);
        })
    };

    // 確認用パスワード入力時の処理
    let handle_confirm_input = {
        let state = state.clone();
        Callback::from(move |e: InputEvent| {
            let input = e.target_unchecked_into::<HtmlInputElement>().value();
            let mut new_state = (*state).clone();
            new_state.confirm_password = input.clone();
            new_state.confirm_error = validate_confirm(&new_state.password, &input);
            state.set(new_state);
        })
    };

    // フォーム送信時の処理
    let onsubmit = {
        let state = state.clone();
        let navigator = Rc::clone(&navigator);
        let auth_ctx = auth_ctx.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default(); // デフォルトのフォーム送信をキャンセル
            let mut current = (*state).clone();
            current.submitted = true;

            // バリデーションチェック
            match validate_signup(&current.email, &current.password, &current.confirm_password) {
                Ok(_) => {
                    // バリデーションOKならAPIに送信
                    let form = SignupForm {
                        email: current.email.clone(),
                        password: current.password.clone(),
                    };

                    let email_for_redirect = current.email.clone();
                    let auth_ctx = auth_ctx.clone();
                    let navigator = Rc::clone(&navigator);
                    let state = state.clone();

                    // 非同期にAPI送信
                    spawn_local(async move {
                        let res = Request::post("/api/auth/signup")
                            .header("Content-Type", "application/json")
                            .body(serde_json::to_string(&form).unwrap())
                            .unwrap()
                            .send()
                            .await;

                        match res {
                            // 成功時：認証情報を更新して画面遷移
                            Ok(resp) if resp.ok() => {
                                if let Some(auth_ctx) = auth_ctx {
                                    auth_ctx.email.set(Rc::new(email_for_redirect.clone()));
                                    auth_ctx.session.set(Rc::new("".to_string()));
                                }
                                navigator.push(&Route::SignupConfirm);
                            }
                            // 失敗時：メッセージを更新
                            Ok(_) => {
                                let mut s = (*state).clone();
                                s.message = "サインアップ失敗...".into();
                                state.set(s);
                            }
                            Err(_) => {
                                let mut s = (*state).clone();
                                s.message = "ネットワークエラー".into();
                                state.set(s);
                            }
                        }
                    });
                }
                // バリデーションNGならエラーメッセージ更新
                Err(errs) => {
                    current.email_error = errs.email;
                    current.password_error = errs.password;
                    current.confirm_error = errs.confirm;
                    state.set(current);
                }
            }
        })
    };

    // 入力がすべて埋まっているかチェック（ボタン活性条件）
    let is_valid = state.email_error.is_none()
        && state.password_error.is_none()
        && state.confirm_error.is_none()
        && !state.email.is_empty()
        && !state.password.is_empty()
        && !state.confirm_password.is_empty();

    // log
    web_sys::console::log_1(&format!("is_valid: {}", is_valid).into());
    web_sys::console::log_1(&format!("email_error: {}", state.email_error.is_none()).into());
    web_sys::console::log_1(&format!("password_error: {}", state.password_error.is_none()).into());
    web_sys::console::log_1(&format!("confirm_error: {}", state.confirm_error.is_none()).into());

    // フックから必要な状態・関数を返す
    SignupState {
        state,
        handle_email_input,
        handle_password_input,
        handle_confirm_input,
        onsubmit,
        is_valid,
    }
}
