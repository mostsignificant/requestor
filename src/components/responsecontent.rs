use crate::components::tab::Tab;

use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use yew::prelude::*;

#[derive(Clone, Copy, Debug, EnumIter, PartialEq)]
enum ResponseContentTab {
    Body,
    Headers,
}

#[derive(Properties, PartialEq)]
pub struct ResponseContentProps {
    pub body: String,
    pub headers: Vec<(String, String)>,
    pub size: u64,
    pub status: u16,
    pub status_text: String,
    pub time: u128,
}

#[function_component(ResponseContent)]
pub fn response_content(props: &ResponseContentProps) -> Html {
    let tab = use_state(|| ResponseContentTab::Body);

    html! {
        <>
            <div class="level m-3">
                <div class="level-left">
                    <div class="level-item">
                        <div class="tags has-addons is-small">
                            <span class="tag is-small">{ "Status" }</span>
                            <span class="tag is-primary is-small">{ props.status }</span>
                        </div>
                    </div>
                    <div class="level-item">
                        <div class="tags has-addons is-small ml-3">
                            <span class="tag is-small">{ "Time" }</span>
                            <span class="tag is-primary is-small">{ props.time } { " ms" }</span>
                        </div>
                    </div>
                    <div class="level-item">
                        <div class="tags has-addons is-small ml-3">
                            <span class="tag is-small">{ "Size" }</span>
                            <span class="tag is-primary is-small">{ props.size } { " B" }</span>
                        </div>
                    </div>
                </div>
            </div>
            <div class="tabs is-small">
                <ul>
                    {
                        ResponseContentTab::iter().map(|value| {
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
                    ResponseContentTab::Body => html! {
                        <div>
                            <pre id="json" class="is-size-7">
                                { props.body.clone() }
                            </pre>
                        </div>
                    },
                    ResponseContentTab::Headers => html! {
                        <div>
                            <table class="table is-size-7">
                                <thead>
                                    <tr>
                                        <th>{"Key"}</th>
                                        <th>{"Value"}</th>
                                    </tr>
                                </thead>
                                <tbody>
                                {
                                    props.headers.clone().into_iter().map(|header| {
                                        html! {
                                            <tr>
                                                <td>{ header.0 }</td>
                                                <td>{ header.1 }</td>
                                            </tr>
                                        }
                                    }).collect::<Html>()
                                }
                                </tbody>
                            </table>
                        </div>
                    },
                }
            }
        </>
    }
}
