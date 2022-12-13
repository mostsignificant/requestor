use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct RequestBarProps {
    pub onchange_url: Callback<String>,
    pub onclick_request: Callback<MouseEvent>,
}

#[function_component(RequestBar)]
pub fn request_bar(props: &RequestBarProps) -> Html {
    let url = use_state(String::default);

    let onchange = {
        let onchange_url = props.onchange_url.clone();
        let url = url.clone();
        Callback::from({
            move |e: Event| {
                let input: HtmlInputElement = e.target_unchecked_into();
                url.set(input.value());
                onchange_url.emit(input.value());
            }
        })
    };

    let onkeypress = {
        let onchange_url = props.onchange_url.clone();
        let onclick_request = props.onclick_request.clone();
        Callback::from(move |event: KeyboardEvent| {
            if event.key() == "Enter" {
                onchange_url.emit((*url).clone());
                onclick_request.emit(MouseEvent::new("").unwrap());
            }
        })
    };

    let onclick = {
        let onclick_request = props.onclick_request.clone();
        Callback::from(move |e: MouseEvent| onclick_request.emit(e))
    };

    html! {
        <div class="field is-horizontal">
            <div class="field-body">
                <div class="field is-expanded">
                    <div class="field has-addons">
                        <p class="control">
                            <div class="select">
                                <select>
                                    <option>{ "GET" }</option>
                                    <option>{ "POST" }</option>
                                    <option>{ "PUT" }</option>
                                    <option>{ "PATCH" }</option>
                                    <option>{ "DELETE" }</option>
                                </select>
                            </div>
                        </p>
                        <p class="control is-expanded">
                            <input
                                class="input is-fullwidth"
                                type="text"
                                {onchange}
                                {onkeypress} />
                        </p>
                    </div>
                </div>
            </div>
            <button
                class="button is-primary ml-3"
                id="send"
                {onclick} >
                { "Send" }
            </button>
        </div>
    }
}
