use yew::prelude::*;

/// 汎用エラーメッセージ表示用のコンポーネント
#[derive(Properties, PartialEq)]
pub struct ErrorMessageProps {
    pub message: String,
    #[prop_or_default]
    pub class: Option<String>,
}

#[function_component(ErrorMessage)]
pub fn error_message(props: &ErrorMessageProps) -> Html {
    if props.message.is_empty() {
        return html! {};
    }

    html! {
        <div class={props.class.clone().unwrap_or_default()}>
            { &props.message }
        </div>
    }
}
