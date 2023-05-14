use leptos::*;

#[component]
pub(crate) fn Projects(cx: Scope) -> impl IntoView {
    view! {cx,
        "Fair warning, most my side projects are just random hobby garbage"<br/>
        "For the latest hot garbage, do checkout my "<a href="https://github.com/akarras">"github"</a>
        <h3><a href="https://ultros.app">"Ultros"</a></h3>
        "Ultros is my attempt at making a ffxiv marketboard tool. it mostly works, and is written in leptos"
        <h3>"Sort of Rust Craft Optimizer"</h3>
        "A half rewritten version of a popular ffxiv crafting optimizer that's been forked a bunch"

        <h3>"todo- pretty much all of this page :D"</h3>
    }
}
