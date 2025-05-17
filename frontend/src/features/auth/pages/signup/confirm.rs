use yew::function_component;
use yew::prelude::*;

use crate::components::layout::Layout;
use crate::features::auth::components::signup_confirm_form::SignupConfirmForm;
use crate::features::auth::hooks::use_confirm_state::use_confirm_state;

#[function_component(SignupConfirmPage)]
pub fn signup_confirm_page() -> Html {
    let confirm = use_confirm_state();
    html! {
        <Layout>
            <SignupConfirmForm
                code={confirm.state.code.clone()}
                message={confirm.state.message.clone()}
                on_input={confirm.handlers.on_input.clone()}
                on_submit={confirm.handlers.on_submit.clone()}
                on_resend={confirm.handlers.on_resend.clone()}
                is_valid={confirm.is_valid}
            />
        </Layout>
    }
}
