//! routes.rs
//!
//! このファイルは Yew Router を用いたルーティング定義を行うモジュールです。
//!
//! 主な役割：
//! - ルートパス（`/signup` など）とページコンポーネントのマッピング
//! - `Route` 列挙型：URLと各ページの対応付け
//! - `switch` 関数：現在のルートに基づいて対応するページコンポーネントを描画
//!
//! このルーティング設定は、`app.rs` 内で `<Switch<Route>>` に渡されて使用されます。
//!
//! 関連ファイル：
//! - `pages/signup.rs`: Signupページの実装
//! - `app.rs`: `Switch<Route>` でこの `Route` と `switch()` を使用

use crate::pages::signin::SigninPage;
use crate::pages::signup::SignupPage;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Routable, Clone, PartialEq, Debug)]
pub enum Route {
    // #[at("/")]
    // Home,
    #[at("/signup")]
    Signup,
    #[at("/signin")]
    Signin,
    #[at("/404")]
    NotFound,
}

/// `Route` に基づいて対応する HTML を返す関数
pub fn switch(routes: Route) -> Html {
    match routes {
        Route::Signup => html! { <SignupPage /> },
        Route::Signin => html! { <SigninPage /> },
        Route::NotFound => html! { <h1>{ "ページが見つかりません" }</h1> },
    }
}
