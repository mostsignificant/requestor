use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct TabProps {
    pub title: String,
    pub is_active: bool,
    pub onclick: Callback<MouseEvent>,
}

#[function_component(Tab)]
pub fn tab(props: &TabProps) -> Html {
    if props.is_active {
        html! {
            <li class="is-active">
                <a onclick={props.onclick.clone()}>
                    { props.title.clone() }
                </a>
            </li>
        }
    } else {
        html! {
            <li>
                <a onclick={props.onclick.clone()}>
                    { props.title.clone() }
                </a>
            </li>
        }
    }
}
