//! utils.rs
//!
//! アプリケーション全体で再利用可能なユーティリティ関数を定義するファイルです。
//!
//! ✅ 現在の用途：
//! - 入力バリデーション関数（例：メール形式の検証、空文字チェック）
//! - 日付・文字列などの変換関数
//! - ログ出力やデバッグ用ヘルパー
//!
//! 🧭 将来的な用途：
//! - 複数ページ間で使われる汎用ロジックの共通化
//! - APIエラー処理の標準化（例：レスポンスからエラーメッセージを抽出）
//! - トークン／クッキー操作などのブラウザサイド関数
//!
//! ※ ページ固有・画面固有の処理はそれぞれのファイルに実装し、
//!    汎用性が高まった段階で本ファイルに抽出してください。

use regex::Regex;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew::{Callback, UseStateHandle};

/// バリデーション付き input ハンドラ（初回 Submit 状態を考慮）
#[allow(dead_code)]
pub fn make_input_handler_with_submit<F>(
    state: UseStateHandle<String>,
    error_state: UseStateHandle<Option<String>>,
    submitted: UseStateHandle<bool>,
    validator: F,
) -> Callback<InputEvent>
where
    F: Fn(&str) -> Option<String> + 'static,
{
    Callback::from(move |e: InputEvent| {
        let value = e.target_unchecked_into::<HtmlInputElement>().value();
        state.set(value.clone());

        let validation_result = if *submitted { validator(&value) } else { None };

        error_state.set(validation_result);
    })
}

/// バリデーション付き input ハンドラ（常時チェック）
#[allow(dead_code)]
pub fn make_input_handler<F>(
    state: UseStateHandle<String>,
    error_state: UseStateHandle<Option<String>>,
    validator: F,
) -> Callback<InputEvent>
where
    F: Fn(&str) -> Option<String> + 'static,
{
    Callback::from(move |e: InputEvent| {
        let value = e.target_unchecked_into::<HtmlInputElement>().value();
        state.set(value.clone());
        error_state.set(validator(&value));
    })
}

#[derive(Default, Clone)]
pub struct PasswordRules {
    pub length: bool,
    pub number: bool,
    pub lowercase: bool,
    pub uppercase: bool,
    pub symbol: bool,
}

#[allow(dead_code)]
impl PasswordRules {
    pub fn from_password(password: &str) -> Self {
        Self {
            length: password.len() >= 8,
            number: Regex::new(r"[0-9]").unwrap().is_match(password),
            lowercase: Regex::new(r"[a-z]").unwrap().is_match(password),
            uppercase: Regex::new(r"[A-Z]").unwrap().is_match(password),
            symbol: Regex::new(r#"[!@#$%^&*(),.?\":{}|<>]"#)
                .unwrap()
                .is_match(password),
        }
    }

    pub fn is_all_valid(&self) -> bool {
        self.length && self.number && self.lowercase && self.uppercase && self.symbol
    }
}

#[derive(Clone, PartialEq, Default)]
pub struct PasswordRulesState {
    pub length: bool,
    pub number: bool,
    pub lowercase: bool,
    pub uppercase: bool,
    pub symbol: bool,
}

pub fn evaluate_password_rules(password: &str) -> PasswordRulesState {
    PasswordRulesState {
        length: password.len() >= 8,
        number: Regex::new(r"[0-9]").unwrap().is_match(password),
        lowercase: Regex::new(r"[a-z]").unwrap().is_match(password),
        uppercase: Regex::new(r"[A-Z]").unwrap().is_match(password),
        symbol: Regex::new(r#"[!@#$%^&*(),.?\":{}|<>]"#)
            .unwrap()
            .is_match(password),
    }
}
