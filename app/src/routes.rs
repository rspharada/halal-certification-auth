//! routes.rs
//!
//! Yew Router を使ったアプリのルーティング定義。
//! 各画面に対して、URLパスとのマッピングと画面描画処理を提供します。

use yew::prelude::*;
use yew_router::prelude::*;

// 各ページを use でインポート
use crate::pages::auth::signup::confirm::ConfirmPage;
use crate::pages::auth::signup::index::SignupPage;

use crate::pages::auth::signin::index::SigninPage;
use crate::pages::auth::signin::mfa::SigninMfaPage;

use crate::pages::auth::password_reset::mfa::PasswordResetMfaPage;
use crate::pages::auth::password_reset::request::PasswordResetRequestPage;
use crate::pages::auth::password_reset::reset::PasswordResetPage;

/// アプリ内のルーティング定義
#[derive(Routable, Clone, PartialEq, Debug)]
pub enum Route {
    #[at("/signup")]
    Signup,

    #[at("/confirm")]
    ConfirmAccount,

    #[at("/signin")]
    Signin,

    #[at("/mfa")]
    SigninMfa,

    #[at("/forgot")]
    PasswordResetRequest,

    #[at("/forgot/mfa")]
    PasswordResetMfa,

    #[at("/reset")]
    PasswordReset,

    #[not_found]
    #[at("/404")]
    NotFound,
}

/// `Route` に応じて描画するコンポーネントを返す関数
pub fn switch(route: Route) -> Html {
    match route {
        Route::Signup => html! { <SignupPage /> },
        Route::ConfirmAccount => html! { <ConfirmPage /> },
        Route::Signin => html! { <SigninPage /> },
        Route::SigninMfa => html! { <SigninMfaPage /> },
        Route::PasswordResetRequest => html! { <PasswordResetRequestPage /> },
        Route::PasswordResetMfa => html! { <PasswordResetMfaPage /> },
        Route::PasswordReset => html! { <PasswordResetPage /> },
        Route::NotFound => html! { <h1>{ "ページが見つかりません" }</h1> },
    }
}
