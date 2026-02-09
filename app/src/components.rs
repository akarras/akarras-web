mod match_formatter;
mod select;

pub use match_formatter::MatchFormatter;
pub use select::Select;

use leptos::prelude::*;

#[component]
pub(crate) fn Card(children: Children) -> impl IntoView {
    view! {
        <div class="border-l-4 border-amber-400 dark:border-teal-500 shadow-md hover:shadow-lg p-5 m-2 rounded-xl bg-white dark:bg-slate-800
            hover:-translate-y-0.5 transition-all duration-300">
            {children()}
        </div>
    }
}