mod match_formatter;
mod select;

pub use match_formatter::MatchFormatter;
pub use select::Select;

use comrak::{markdown_to_html, ComrakOptions};
use leptos::*;

#[component]
pub(crate) fn Card(children: Children) -> impl IntoView {
    view! {
        <div class="border-solid border-neutral-100 dark:border-neutral-950 border-2 shadow-md p-4 m-2 rounded-md bg-neutral-300 dark:bg-neutral-950
            dark:hover:border-red-300 hover:border-red-700 dark:hover:bg-gray-900 ease-in-out duration-300">
            {children()}
        </div>
    }
}

#[component]
pub(crate) fn Markdown(text: String) -> impl IntoView {
    view! {
    <div class="prose dark:prose-invert" inner_html=markdown_to_html(&text, &ComrakOptions::default())>
    </div>
    }
}
