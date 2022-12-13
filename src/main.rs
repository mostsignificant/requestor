mod components;

use components::requestbar::RequestBar;
use components::requestcontent::RequestContent;
use components::responsecontent::ResponseContent;
use components::sidenav::SideNav;
use reqwasm::http::Request;
use yew::prelude::*;

#[function_component(App)]
fn app_component() -> Html {
    // state

    let url = use_state(String::default);
    let error = use_state(|| None);
    let response_body = use_state(String::default);
    let response_headers = use_state(Vec::<(String, String)>::default);
    let response_size = use_state(u64::default);
    let response_status = use_state(u16::default);
    let response_status_text = use_state(String::default);
    let response_time = use_state(u128::default);

    // handlers

    let onclick_request = {
        let url = url.clone();
        let error = error.clone();
        let response_body = response_body.clone();
        let response_headers = response_headers.clone();
        let response_size = response_size.clone();
        let response_status = response_status.clone();
        let response_status_text = response_status_text.clone();
        let response_time = response_time.clone();

        Callback::from(move |_| {
            let url = url.clone();
            let error = error.clone();
            let response_body = response_body.clone();
            let response_headers = response_headers.clone();
            let response_size = response_size.clone();
            let response_status = response_status.clone();
            let response_status_text = response_status_text.clone();
            let response_time = response_time.clone();

            wasm_bindgen_futures::spawn_local(async move {
                let now = instant::Instant::now();
                let fetched = Request::get(&url).send().await;
                let duration = instant::Instant::now() - now;

                match fetched {
                    Ok(fetched) => {
                        let headers: Vec<(String, String)> = fetched.headers().entries().collect();
                        response_headers.set(headers);
                        response_status.set(fetched.status());
                        response_status_text.set(fetched.status_text());
                        response_time.set(duration.as_millis());

                        let binary = fetched.binary().await;
                        match binary {
                            Ok(binary) => {
                                response_body.set(String::from_utf8(binary.clone()).unwrap());
                                response_size.set(binary.len().try_into().unwrap());
                            }
                            Err(e) => error.set(Some(e.to_string())),
                        }
                    }
                    Err(e) => error.set(Some(e.to_string())),
                }
            });
        })
    };

    let onchange_url = {
        let url = url.clone();
        Callback::from(move |new_url: String| {
            url.set(new_url);
        })
    };

    // return html render

    html! {
        <div class="tile is-ancestor p-3 is-12 is-align-items-stretch">
            <div class="tile is-parent box is-2 is-align-items-stretch">
                <div class="tile is-child is-align-items-stretch">
                    <SideNav />
                </div>
            </div>
            <div class="tile is-parent is-vertical is-10">
                <div class="tile is-parent is-vertical">
                    <div class="tile is-child">
                        <RequestBar
                            onchange_url={onchange_url.clone()}
                            onclick_request={onclick_request.clone()} />
                        <RequestContent />
                    </div>
                </div>
                <div class="tile is-parent">
                    <div class="tile is-child">
                        <ResponseContent
                            body={(*response_body).clone()}
                            headers={(*response_headers).clone()}
                            size={*response_size}
                            status={*response_status}
                            status_text={(*response_status_text).clone()}
                            time={*response_time} />
                    </div>
                </div>
            </div>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
