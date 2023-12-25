mod blog;
mod components;
mod flickr;
mod home_page;
mod pictures;
mod projects;

use crate::{blog::*, home_page::*, pictures::*, projects::*};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

pub mod error_template;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/akarras.css"/>

        <Title text="aaron karras' personal home page"/>
        <Body class="bg-neutral-100 dark:bg-neutral-900 text-base dark:text-white"/>
        // content for this welcome page
        <Router>
            <div class="container mx-auto px-4">
                <nav class="p-4 flex flex-row align-items-middle justify-items-stretch gap-4">
                    <A class="aria-current:font-bold" href="/" exact=true>"home"</A>
                    <A class="aria-current:font-bold" href="blog">"blog"</A>
                    <A class="aria-current:font-bold" href="projects">"projects"</A>
                    <A class="aria-current:font-bold" href="photos">"photos"</A>
                    <div class="grow"></div>
                    <a href="https://www.linkedin.com/in/adkarras">"linkedin"</a>
                    <a href="https://github.com/akarras">"github"</a>
                    <a href="mailto:aaron@akarras.com">"email"</a>
                </nav>
                <main>
                    <Routes>
                        <Route path="" view=HomePage />
                        <Route path="blog" view=Blog >
                            <Route path=":slug" view=BlogItem />
                            <Route path="" view=BlogList />
                        </Route>
                        <Route path="projects" view=Projects />
                        <Route path="photos" view=Pictures />
                    </Routes>
                </main>
            </div>
        </Router>
    }
}
