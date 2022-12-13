use crate::components::tab::Tab;

use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use yew::prelude::*;

#[derive(Clone, Copy, Debug, EnumIter, PartialEq)]
enum RequestContentTab {
    Params,
    Authorization,
    Headers,
    Body,
}

#[function_component(RequestContent)]
pub fn request_content() -> Html {
    let tab = use_state(|| RequestContentTab::Params);

    html! {
        <>
            <div class="tabs is-small">
                <ul>
                    {
                        RequestContentTab::iter().map(|value| {
                            html! {
                                <Tab
                                    title={format!("{:?}", value)}
                                    is_active={*tab == value}
                                    onclick={
                                        let tab = tab.clone();
                                        Callback::from(move |_| { tab.set(value); })
                                    } />
                            }
                        }).collect::<Html>()
                    }
                </ul>
            </div>
            {
                match *tab {
                    RequestContentTab::Params => html! {
                        <div>
                            <div class="field is-horizontal">
                                <div class="field">
                                    <input
                                        class="input is-small"
                                        placeholder="Key" />
                                </div>
                                <div class="field ml-3">
                                    <input
                                        class="input is-small"
                                        placeholder="Value" />
                                </div>
                                <button class="delete is-small ml-3"></button>
                            </div>
                        </div>
                    },
                    RequestContentTab::Authorization => html! {
                        <div>
                            <h3>{ "Authorization" }</h3>
                        </div>
                    },
                    RequestContentTab::Headers => html! {
                        <div>
                            <h3>{ "Headers" }</h3>
                        </div>
                    },
                    RequestContentTab::Body => html! {
                        <div>
                            <h3>{ "Body" }</h3>
                        </div>
                    },
                }
            }
        </>
    }
}
