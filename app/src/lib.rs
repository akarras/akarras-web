mod blog;
mod components;
mod flickr;
mod home_page;
mod pictures;
mod projects;
mod toys;

use crate::{
    blog::*,
    home_page::*,
    pictures::*,
    projects::*,
    toys::{ToyPage, VehicleSim},
};
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
        <Body class="bg-neutral-100 dark:bg-neutral-950 text-base dark:text-neutral-200"/>
        // content for this welcome page
        <Router>
            <div class="container mx-auto px-4">
                <nav class="p-4 flex flex-row align-items-middle justify-items-stretch gap-4">
                    <A class="aria-current:font-bold hover:text-neutral-800 dark:hover:text-neutral-300" href="/" exact=true>"home"</A>
                    <A class="aria-current:font-bold hover:text-neutral-800 dark:hover:text-neutral-300" href="blog">"blog"</A>
                    <A class="aria-current:font-bold hover:text-neutral-800 dark:hover:text-neutral-300" href="projects">"projects"</A>
                    <A class="aria-current:font-bold hover:text-neutral-800 dark:hover:text-neutral-300" href="photos">"photos"</A>
                    <A class="aria-current:font-bold hover:text-neutral-800 dark:hover:text-neutral-300" href="toys">"toys"</A>
                    <div class="grow"></div>
                    <a class="hover:text-neutral-800 dark:hover:text-neutral-300" href="https://www.linkedin.com/in/adkarras">"linkedin"</a>
                    <a class="hover:text-neutral-800 dark:hover:text-neutral-300" href="https://github.com/akarras">"github"</a>
                    <a class="hover:text-neutral-800 dark:hover:text-neutral-300" href="mailto:aaron@akarras.com">"email"</a>
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
                        <Route path="toys" view=ToyPage />
                        <Route path="toys/ev-charger-sim" view=VehicleSim />
                    </Routes>
                </main>
            </div>
        </Router>
    }
}
