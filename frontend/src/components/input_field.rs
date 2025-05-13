use yew::prelude::*;

/// InputField 用の Props
#[derive(Properties, PartialEq, Clone)]
pub struct InputFieldProps {
    pub input_type: String,
    pub placeholder: String,
    pub value: String,
    pub oninput: Callback<InputEvent>,
    #[prop_or_default]
    pub error_message: Option<String>,
    #[prop_or_default]
    pub class: Option<String>,
    #[prop_or_default]
    pub style: Option<String>,
}

#[function_component(InputField)]
pub fn input_field(props: &InputFieldProps) -> Html {
    let autocomplete = match props.input_type.as_str() {
        "email" => Some("email"),
        "password" => Some("new-password"),
        "username" => Some("username"),
        _ => None,
    };

    html! {
        <div
            class={props.class.clone().unwrap_or_default()}
            style={props.style.clone().unwrap_or_default()}
        >
            <input
                type={props.input_type.clone()}
                placeholder={props.placeholder.clone()}
                value={props.value.clone()}
                oninput={props.oninput.clone()}
                autocomplete={autocomplete}
                class="input-field"
            />
            {
                if let Some(error) = &props.error_message {
                    html! { <span class="input-error">{ error }</span> }
                } else {
                    html! {}
                }
            }
        </div>
    }
}
