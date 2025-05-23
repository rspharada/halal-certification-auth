//! types.rs
//!
//! 本ファイルは、アプリケーション内で使用されるデータ構造（構造体や列挙型など）を定義します。
//! 特に、ページごとのフォーム送信データや API リクエスト／レスポンスに使用される型を集約します。
//!
//! ✅ 現在の用途：
//! - 各ページのフォーム送信用構造体の定義
//! - JSON 送信に対応するための Serialize/Deserialize 実装
//!
//! 🧭 将来的な用途：
//! - フロント全体で再利用される値オブジェクトの定義
//! - API レスポンス型（例：`ApiError`, `UserInfo`）の共通化
//! - ユーザー状態、セッション、MFAステージなどの enum 化
//!
//! ※ ページごとに限定される型は当面は各ページ内に定義し、共通化が必要になった時点で本ファイルへ移動してください。
use std::rc::Rc;
use yew::prelude::*;

/// 認証状態（主にメールアドレスとセッション情報）を共有するコンテキスト
#[derive(Clone, PartialEq)]
pub struct AuthContext {
    pub message: UseStateHandle<Rc<String>>,
    pub email: UseStateHandle<Rc<String>>,
    pub session: UseStateHandle<Rc<String>>,
}
