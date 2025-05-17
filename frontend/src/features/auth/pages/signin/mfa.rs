//! index.rs
//!
//! サインインページの表示用エントリーポイント。
//! コンポーネントとレイアウトのみで構成される。

use crate::components::layout::Layout;
use crate::features::auth::components::signin_mfa_form::SigninMfaForm;
use yew::prelude::*;

#[function_component(SigninMfaPage)]
pub fn signin_page() -> Html {
    html! {
        <Layout>
            <SigninMfaForm />
        </Layout>
    }
}
