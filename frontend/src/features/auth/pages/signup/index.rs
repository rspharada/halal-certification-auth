//! サインアップページ（SignupPage）
//!
//! レイアウトとサインアップフォームの表示のみを担当。
//! ロジックは `use_signup_state`、UI構成は `SignupForm` コンポーネントへ委譲。

use crate::components::layout::Layout;
use crate::features::auth::components::signup_form::SignupForm;
use yew::prelude::*;

#[function_component(SignupPage)]
pub fn signup_page() -> Html {
    html! {
        <Layout>
            <SignupForm />
        </Layout>
    }
}
