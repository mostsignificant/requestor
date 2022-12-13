use yew::prelude::*;

fn truncate(string: &String, max_chars: usize) -> String {
    match string.char_indices().nth(max_chars) {
        None => string.clone(),
        Some((idx, _)) => format!("{}...", &string[..idx]).to_string(),
    }
}

#[function_component(SideNav)]
pub fn side_nav() -> Html {
    html! {
        <aside class="menu">
            <p class="menu-label">
                {"Bookmarks"}
            </p>
            <ul class="menu-list">
                <li><a>{"Dashboard"}</a></li>
                <li><a>{"Customers"}</a></li>
            </ul>
            <p class="menu-label">
                {"History"}
            </p>
            <ul class="menu-list">
                <li><a>{"Payments"}</a></li>
                <li><a>{"Transfers"}</a></li>
                <li><a>{"Balance"}</a></li>
            </ul>
            <p class="menu-label">
                {"Examples"}
            </p>
            <ul class="menu-list">
                <li><a>{truncate(&String::from("https://jsonplaceholder.typicode.com/posts"), 45)}</a></li>
                <li><a>{truncate(&String::from("https://jsonplaceholder.typicode.com/commentsaaaaaaaaaaaaaa"), 45)}</a></li>
                <li><a>{truncate(&String::from("https://jsonplaceholder.typicode.com/albums"), 45)}</a></li>
                <li><a>{truncate(&String::from("https://jsonplaceholder.typicode.com/photos"), 45)}</a></li>
                <li><a>{truncate(&String::from("https://jsonplaceholder.typicode.com/todos"), 45)}</a></li>
                <li><a>{truncate(&String::from("https://jsonplaceholder.typicode.com/users"), 45)}</a></li>
            </ul>
        </aside>
    }
}
