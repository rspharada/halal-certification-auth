//! request.rs
//!
//! パスワード再発行申請ページ（UIは PasswordResetRequestForm に分離）

use crate::components::layout::Layout;
use crate::features::auth::components::password_reset_request_form::PasswordResetRequestForm;
use yew::prelude::*;

/// パスワード再発行申請ページ
#[function_component(PasswordResetRequestPage)]
pub fn password_reset_request_page() -> Html {
    html! {
        <Layout>
            <PasswordResetRequestForm />
        </Layout>
    }
}
