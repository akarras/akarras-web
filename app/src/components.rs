use comrak::{markdown_to_html, ComrakOptions};
use leptos::*;

#[component]
pub(crate) fn Card(cx: Scope, children: Children) -> impl IntoView {
    view!{cx, 
        <div class="border-solid border-neutral-100 dark:border-neutral-950 border-2 shadow-md p-4 m-2 rounded-md bg-neutral-300 dark:bg-neutral-800
            dark:hover:border-red-300 hover:border-red-700 dark:hover:bg-neutral-700 ease-in-out duration-300">
            {children(cx)}
        </div>
    }
}

#[component]
pub(crate) fn Markdown(cx: Scope, text: String) -> impl IntoView {
    view!{cx, 
    <div class="prose dark:prose-invert" inner_html=markdown_to_html(&text, &ComrakOptions::default())>
    </div>
    }
}
