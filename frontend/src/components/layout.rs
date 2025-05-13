use yew::prelude::*;

/// Layout コンポーネント：子要素をラップして、全体レイアウトを提供する
#[derive(Properties, PartialEq)]
pub struct LayoutProps {
    #[prop_or_default]
    pub children: Children,
}

#[function_component(Layout)]
pub fn layout(props: &LayoutProps) -> Html {
    html! {
        <div class="layout-root">
            <div class="layout-card">
                { for props.children.iter() }
            </div>
        </div>
    }
}
