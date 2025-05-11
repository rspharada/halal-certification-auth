//! app.rs
//!
//! このファイルは Yew アプリケーションのルートコンポーネント `<App />` を定義します。
//!
//! 主な役割：
//! - `BrowserRouter` によるクライアントサイドルーティングの設定
//! - `Switch<Route>` によるルーティングと画面切り替え処理
//!
//! このコンポーネントは `main.rs` または `lib.rs` から呼び出され、
//! アプリケーション全体のルーティングエントリーポイントとして機能します。
//!
//! 関連ファイル：
//! - `routes.rs`: `Route` 列挙型と `switch` 関数の定義
//! - `pages/`: 各ページコンポーネント（例：`signup.rs`, `signin.rs`）

use crate::routes::{Route, switch};
use crate::types::AuthContext;
use std::rc::Rc;
use yew::prelude::*;
use yew_router::prelude::*;

/// アプリケーション全体のルートコンポーネント（SPAのエントリーポイント）
#[function_component(App)]
pub fn app() -> Html {
    let email = use_state(|| Rc::new("".to_string()));
    let session = use_state(|| Rc::new("".to_string()));
    let auth_ctx = AuthContext { email, session };

    html! {
        <ContextProvider<AuthContext> context={auth_ctx}>
            <BrowserRouter>
                <Switch<Route> render={switch} />
            </BrowserRouter>
        </ContextProvider<AuthContext>>
    }
}
