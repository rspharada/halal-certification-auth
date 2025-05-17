//! index.rs
//!
//! サインインページの表示用エントリーポイント。
//! コンポーネントとレイアウトのみで構成される。

use crate::components::layout::Layout;
use crate::features::auth::components::signin_form::SigninForm;
use yew::prelude::*;

#[function_component(SigninPage)]
pub fn signin_page() -> Html {
    html! {
        <Layout>
            <SigninForm />
        </Layout>
    }
}
