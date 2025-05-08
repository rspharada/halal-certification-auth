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
use yew::prelude::*;
use yew_router::prelude::*;

/// アプリケーション全体のルートコンポーネント（SPAのエントリーポイント）
#[function_component(App)]
pub fn app() -> Html {
    html! {
        // <BrowserRouter> はクライアントサイドルーティングの土台
        <BrowserRouter>
            // <Switch<Route>> によりルート定義に応じてページ切り替え
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}
