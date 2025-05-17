//! routes.rs
//!
//! Yew Router を使ったアプリのルーティング定義。
//! 各画面に対して、URLパスとのマッピングと画面描画処理を提供します。

use yew::prelude::*;
use yew_router::prelude::*;

// 各ページを use でインポート

use crate::features::auth::pages::signup::confirm::SignupConfirmPage;
use crate::features::auth::pages::signup::confirm_complete::SignupConfirmCompletePage;
use crate::features::auth::pages::signup::index::SignupPage;

use crate::features::auth::pages::signin::index::SigninPage;
use crate::features::auth::pages::signin::mfa::SigninMfaPage;

use crate::features::auth::pages::password_reset::mfa::PasswordResetMfaPage;
use crate::features::auth::pages::password_reset::request::PasswordResetRequestPage;
use crate::features::auth::pages::password_reset::reset::PasswordResetPage;

/// アプリ内のルーティング定義
#[derive(Routable, Clone, PartialEq, Debug)]
pub enum Route {
    #[at("/auth/signup")]
    Signup,

    #[at("/auth/signup/confirm")]
    SignupConfirm,

    #[at("/auth/signup/confirm/complete")]
    SignupConfirmComplete,

    #[at("/auth/signin")]
    Signin,

    #[at("/auth/signin/mfa/")]
    SigninMfa,

    #[at("/auth/forgot")]
    PasswordResetRequest,

    #[at("/auth/forgot/mfa")]
    PasswordResetMfa,

    #[at("/auth/reset")]
    PasswordReset,

    #[not_found]
    #[at("/404")]
    NotFound,
}

/// `Route` に応じて描画するコンポーネントを返す関数
pub fn switch(route: Route) -> Html {
    match route {
        Route::Signup => html! { <SignupPage /> },
        Route::SignupConfirm => html! { <SignupConfirmPage/> },
        Route::SignupConfirmComplete => html! { <SignupConfirmCompletePage /> },
        Route::Signin => html! { <SigninPage /> },
        Route::SigninMfa => {
            html! { <SigninMfaPage/> }
        }
        Route::PasswordResetRequest => html! { <PasswordResetRequestPage /> },
        Route::PasswordResetMfa => html! { <PasswordResetMfaPage /> },
        Route::PasswordReset => html! { <PasswordResetPage /> },
        Route::NotFound => html! {
            <div style="text-align: center; padding: 3rem;">
                <h1 style="font-size: 3rem; color: #000000;">{ "404 - ページが見つかりません" }</h1>
                <p style="font-size: 1.25rem; margin-top: 1rem;">
                    { "申し訳ありません。お探しのページは存在しないか、移動された可能性があります。" }
                </p>
                <a href="/auth/signin" style="color: #3182ce; text-decoration: underline; display: inline-block; margin-top: 2rem;">
                    { "サインインページに戻る" }
                </a>
            </div>
        },
    }
}
