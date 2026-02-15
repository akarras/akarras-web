mod components;
mod flickr;
mod home_page;
pub mod not_found;
mod pictures;
mod projects;
mod toys;

use crate::{
    home_page::*,
    pictures::*,
    projects::*,
    toys::{ToyPage, VehicleSim},
};
use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{components::*, path};

pub mod error_template;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    view! {
        <Title text="aaron karras' personal home page"/>
        // content for this welcome page
        <Router>
            <div class="container mx-auto px-4">
                <nav class="py-6 px-4 flex flex-row items-center gap-6 flex-wrap text-lg">
                    <A attr:class="nav-link aria-current:font-bold aria-current:text-amber-600 dark:aria-current:text-teal-400 hover:text-amber-600 dark:hover:text-teal-400 transition-colors" href="/" exact=true>"home"</A>
                    <A attr:class="nav-link aria-current:font-bold aria-current:text-amber-600 dark:aria-current:text-teal-400 hover:text-amber-600 dark:hover:text-teal-400 transition-colors" href="projects">"projects"</A>
                    <A attr:class="nav-link aria-current:font-bold aria-current:text-amber-600 dark:aria-current:text-teal-400 hover:text-amber-600 dark:hover:text-teal-400 transition-colors" href="photos">"photos"</A>
                    <A attr:class="nav-link aria-current:font-bold aria-current:text-amber-600 dark:aria-current:text-teal-400 hover:text-amber-600 dark:hover:text-teal-400 transition-colors" href="toys">"toys"</A>
                    <div class="grow"></div>
                    <a class="nav-link hover:text-amber-600 dark:hover:text-teal-400 transition-colors" href="https://www.linkedin.com/in/adkarras">"linkedin"</a>
                    <a class="nav-link hover:text-amber-600 dark:hover:text-teal-400 transition-colors" href="https://github.com/akarras">"github"</a>
                    <a class="nav-link hover:text-amber-600 dark:hover:text-teal-400 transition-colors" href="mailto:aaron@akarras.com">"email"</a>
                </nav>
                <main>
                    <Routes fallback=|| "Not found.">
                        <Route path=path!("/") view=HomePage />
                        <Route path=path!("/projects") view=Projects />
                        <Route path=path!("/photos") view=Pictures />
                        <Route path=path!("/toys") view=ToyPage />
                        <Route path=path!("/toys/ev-charger-sim") view=VehicleSim />
                    </Routes>
                </main>
            </div>
        </Router>
    }
}
