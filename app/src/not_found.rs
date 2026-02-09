use leptos::prelude::*;

#[component]
pub fn NotFound() -> impl IntoView {
    view! {
        <div class="flex flex-col items-center justify-center">
            <h1 class="text-6xl font-bold">"404"</h1>
            <p class="text-2xl">"Page not found"</p>
        </div>
    }
}
