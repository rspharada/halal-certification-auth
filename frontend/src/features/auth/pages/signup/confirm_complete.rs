use crate::components::layout::Layout;
use crate::features::auth::components::signup_confirm_complete::SignupConfirmComplete;
use yew::prelude::*;

/// サインアップ完了メッセージ用ページ
#[function_component(SignupConfirmCompletePage)]
pub fn signup_confirm_complete_page() -> Html {
    html! {
        <Layout>
            <SignupConfirmComplete />
        </Layout>
    }
}
