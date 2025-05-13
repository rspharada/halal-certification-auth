use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct PasswordRulesProps {
    pub length: bool,
    pub number: bool,
    pub lowercase: bool,
    pub uppercase: bool,
    pub symbol: bool,
}

#[function_component(PasswordRules)]
pub fn password_rules(props: &PasswordRulesProps) -> Html {
    html! {
        <ul class="password-rules">
            <li class={if props.length { "valid" } else { "invalid" }}>
                { "8文字以上であること" }
            </li>
            <li class={if props.number { "valid" } else { "invalid" }}>
                { "数字を含むこと" }
            </li>
            <li class={if props.lowercase { "valid" } else { "invalid" }}>
                { "小文字を含むこと" }
            </li>
            <li class={if props.uppercase { "valid" } else { "invalid" }}>
                { "大文字を含むこと" }
            </li>
            <li class={if props.symbol { "valid" } else { "invalid" }}>
                { "記号（!@#$%^&*など）を含むこと" }
            </li>
        </ul>
    }
}
