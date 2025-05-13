//! signup.rs
//!
//! このファイルは Yew アプリケーションの「サインアップページ」を定義します。
//!
//! 主な役割：
//! - メールアドレス・パスワード・確認用パスワードの状態管理
//! - パスワード一致チェック
//! - サインアップAPI（POST /api/auth/signup）への非同期送信処理
//! - 成功時に `/auth/mfa/:email` へ遷移（yew_router使用）
//!
//! 関連ファイル：
//! - `routes.rs`: Route::SigninMfa { email } にマッピング

use crate::components::input_field::InputField;
use crate::components::layout::Layout;
use crate::components::password_rules::PasswordRules;
use crate::routes::Route;
use crate::types::AuthContext;
use crate::utils::PasswordRulesState;
use crate::utils::evaluate_password_rules;
use crate::utils::make_input_handler;
use crate::utils::make_input_handler_with_submit;
use gloo::net::http::Request;
use regex::Regex;
use serde::Serialize;
use std::rc::Rc;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Serialize)]
struct SignupForm {
    email: String,
    password: String,
}

fn validate_email(val: &str) -> Option<String> {
    if val.trim().is_empty() {
        Some("メールアドレスは必須です".into())
    } else if !Regex::new(r"^[\w\.-]+@[\w\.-]+\.\w+$")
        .unwrap()
        .is_match(val)
    {
        Some("有効なメールアドレスを入力してください".into())
    } else {
        None
    }
}

fn validate_password(val: &str) -> Option<String> {
    if val.len() < 8 {
        return Some("パスワードは8文字以上で入力してください".into());
    }
    if !Regex::new(r"[0-9]").unwrap().is_match(val) {
        return Some("パスワードには少なくとも1つの数字が必要です".into());
    }
    if !Regex::new(r#"[!@#$%^&*(),.?\":{}|<>]"#)
        .unwrap()
        .is_match(val)
    {
        return Some("パスワードには少なくとも1つの特殊文字が必要です".into());
    }
    if !Regex::new(r"[A-Z]").unwrap().is_match(val) {
        return Some("パスワードには少なくとも1つの大文字が必要です".into());
    }
    if !Regex::new(r"[a-z]").unwrap().is_match(val) {
        return Some("パスワードには少なくとも1つの小文字が必要です".into());
    }
    None
}

fn validate_confirm(password: &str, confirm: &str) -> Option<String> {
    (password != confirm).then(|| "パスワードが一致しません".into())
}

#[derive(Debug, PartialEq, Eq, Default)]
pub struct SignupValidationErrors {
    pub email: Option<String>,
    pub password: Option<String>,
    pub confirm: Option<String>,
}

impl SignupValidationErrors {
    pub fn is_empty(&self) -> bool {
        self.email.is_none() && self.password.is_none() && self.confirm.is_none()
    }
}

pub fn validate_signup(
    email: &str,
    password: &str,
    confirm_password: &str,
) -> Result<(), SignupValidationErrors> {
    let errors = SignupValidationErrors {
        email: validate_email(email),
        password: validate_password(password),
        confirm: validate_confirm(password, confirm_password),
    };

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

#[function_component(SignupPage)]
pub fn signup_page() -> Html {
    let auth_ctx = use_context::<AuthContext>();

    let email = use_state(|| "".to_string());
    let password = use_state(|| "".to_string());
    let confirm_password = use_state(|| "".to_string());
    let password_rules = use_state(PasswordRulesState::default);
    let email_error = use_state(|| None);
    let password_error = use_state(|| None);
    let confirm_error = use_state(|| None);
    let submitted = use_state(|| false);
    let message = use_state(|| "".to_string());
    let is_password_first_input = use_state(|| false);
    let is_password_first_input_for_handler = is_password_first_input.clone();
    let is_password_first_input_for_render = is_password_first_input.clone();

    let is_valid = !email.is_empty() && !password.is_empty() && !confirm_password.is_empty();
    let navigator = use_navigator().unwrap();
    let handle_email_input = make_input_handler_with_submit(
        email.clone(),
        email_error.clone(),
        submitted.clone(),
        validate_email,
    );

    let password_rules = password_rules.clone();
    let confirm_error_for_password = confirm_error.clone();
    let confirm_error_for_confirm = confirm_error.clone();

    let password_for_confirm = password.clone();
    let confirm_password_for_validation = confirm_password.clone();

    let submitted_for_password = submitted.clone();
    let rules = (*password_rules).clone();
    let handle_password_input =
        make_input_handler(password.clone(), password_error.clone(), move |password| {
            web_sys::console::log_1(&"ログ出力したい内容".into());
            is_password_first_input_for_handler.set(true);
            password_rules.set(evaluate_password_rules(&password));
            let is_submitted = *submitted_for_password;
            if !is_submitted {
                return None;
            }
            let confirm_val = confirm_password_for_validation.clone();
            confirm_error_for_password.set(validate_confirm(&password, &confirm_val));
            validate_password(&password)
        });

    let handle_confirm_input = make_input_handler_with_submit(
        confirm_password.clone(),
        confirm_error_for_confirm,
        submitted.clone(),
        move |confirm| validate_confirm(&password_for_confirm, confirm),
    );

    let onsubmit = {
        let email = email.clone();
        let password = password.clone();
        let confirm_password = confirm_password.clone();
        let message = message.clone();
        let navigator = navigator.clone();
        let auth_ctx = auth_ctx.clone();
        let email_error = email_error.clone();
        let password_error = password_error.clone();
        let confirm_error = confirm_error.clone();
        let submitted = submitted.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            submitted.set(true);

            match validate_signup(&email, &password, &confirm_password) {
                Ok(_) => {}
                Err(errs) => {
                    email_error.set(errs.email);
                    password_error.set(errs.password);
                    confirm_error.set(errs.confirm);
                    return;
                }
            }

            let form = SignupForm {
                email: email.to_string(),
                password: password.to_string(),
            };

            let message = message.clone();
            let nav = navigator.clone();
            let email_for_redirect = email.to_string();
            let auth_ctx = auth_ctx.clone();

            spawn_local(async move {
                let res = Request::post("/api/auth/signup")
                    .header("Content-Type", "application/json")
                    .body(serde_json::to_string(&form).unwrap())
                    .unwrap()
                    .send()
                    .await;

                match res {
                    Ok(resp) if resp.ok() => {
                        if let Some(auth_ctx) = auth_ctx {
                            auth_ctx.email.set(Rc::new(email_for_redirect.clone()));
                            auth_ctx.session.set(Rc::new("".to_string()));
                        }
                        nav.push(&Route::SignupConfirm);
                    }
                    Ok(_) => message.set("サインアップ失敗...".into()),
                    Err(_) => message.set("ネットワークエラー".into()),
                }
            });
        })
    };

    html! {
        <Layout>
            <form class="signup-form" {onsubmit}>
                <h2 class="form-title">{ "サインアップ" }</h2>

                if !message.is_empty() {
                    <p class="form-error">{ &*message }</p>
                }

                <InputField
                    input_type={"email".to_string()}
                    placeholder={"メールアドレス".to_string()}
                    value={(*email).clone()}
                    oninput={handle_email_input}
                    class={Some("form-group".to_string())}
                    error_message={(*email_error).clone()}
                />

                <InputField
                    input_type={"password".to_string()}
                    placeholder={"パスワード".to_string()}
                    value={(*password).clone()}
                    oninput={handle_password_input}
                    class={Some("form-group".to_string())}
                    error_message={(*password_error).clone()}
                />

                if *is_password_first_input_for_render {
                    <PasswordRules
                        length={rules.length}
                        number={rules.number}
                        lowercase={rules.lowercase}
                        uppercase={rules.uppercase}
                        symbol={rules.symbol}
                    />
                }

                <InputField
                    input_type={"password".to_string()}
                    placeholder={"確認用パスワード".to_string()}
                    value={(*confirm_password).clone()}
                    oninput={handle_confirm_input}
                    class={Some("form-group".to_string())}
                    error_message={(*confirm_error).clone()}
                />

                <button type="submit" class="form-submit" disabled={!is_valid}>{ "登録" }</button>
            </form>
        </Layout>
    }
}

#[cfg(test)]
mod signup_tests {
    use super::*;
    use gloo::utils::document;
    use wasm_bindgen_test::*;
    use yew::Renderer;

    wasm_bindgen_test_configure!(run_in_browser);

    #[allow(dead_code)]
    #[wasm_bindgen_test]
    fn it_renders_signup_page() {
        let div = document().create_element("div").unwrap();
        document().body().unwrap().append_child(&div).unwrap();

        Renderer::<SignupPage>::with_root(div.into()).render();

        let body_html = document().body().unwrap().inner_html();
        assert!(
            body_html.contains("サインアップ"),
            "ページに 'サインアップ' が含まれていません。HTML内容:\n{}",
            body_html
        );
    }

    #[test]
    fn test_validate_signup_valid_input() {
        assert!(validate_signup("test@example.com", "Password1!", "Password1!").is_ok());
    }

    #[test]
    fn test_validate_signup_invalid_cases() {
        let cases = vec![
            ("", "Password1!", "Password1!", "メールアドレスは必須です"),
            (
                "invalid",
                "Password1!",
                "Password1!",
                "有効なメールアドレスを入力してください",
            ),
            (
                "test@example.com",
                "P1!",
                "P1!",
                "パスワードは8文字以上で入力してください",
            ),
            (
                "test@example.com",
                "Password!",
                "Password!",
                "パスワードには少なくとも1つの数字が必要です",
            ),
            (
                "test@example.com",
                "Password1",
                "Password1",
                "パスワードには少なくとも1つの特殊文字が必要です",
            ),
            (
                "test@example.com",
                "password1!",
                "password1!",
                "パスワードには少なくとも1つの大文字が必要です",
            ),
            (
                "test@example.com",
                "PASSWORD1!",
                "PASSWORD1!",
                "パスワードには少なくとも1つの小文字が必要です",
            ),
            (
                "test@example.com",
                "Password1!",
                "Password2!",
                "パスワードが一致しません",
            ),
        ];

        for (email, pw, confirm, _) in cases {
            let result = validate_signup(email, pw, confirm);
            assert!(
                result.is_err(),
                "期待されるエラーが発生しませんでした。入力: {}, {}, {}",
                email,
                pw,
                confirm
            );
        }
    }
}
