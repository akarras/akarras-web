use crate::components::Card;
use crate::pictures::SmallPhotos;
use leptos::prelude::*;

#[component]
pub(crate) fn HomePage() -> impl IntoView {
    view! {
        <div class="max-w-4xl mx-auto py-8">
            // Hero
            <div class="mb-12">
                <h1 class="text-5xl font-bold mb-2">"Aaron Karras"</h1>
                <p class="text-2xl text-neutral-600 dark:text-neutral-400 mb-4">"rust developer"</p>
                <p class="text-lg max-w-2xl">
                    "software engineer who loves building things with Rust. currently working with systems programming, web apps, and anything that catches my curiosity."
                </p>
            </div>

            // Skills
            <div class="mb-12">
                <h2 class="text-2xl font-bold mb-4">"what I work with"</h2>
                <div class="grid grid-cols-2 md:grid-cols-4 gap-2">
                    <Card><span class="font-semibold">"Rust"</span></Card>
                    <Card><span class="font-semibold">"C++"</span></Card>
                    <Card><span class="font-semibold">"Leptos"</span></Card>
                    <Card><span class="font-semibold">"WebAssembly"</span></Card>
                    <Card><span class="font-semibold">"Systems Programming"</span></Card>
                    <Card><span class="font-semibold">"Networking"</span></Card>
                    <Card><span class="font-semibold">"Linux"</span></Card>
                    <Card><span class="font-semibold">"Docker"</span></Card>
                </div>
            </div>

            // Featured projects
            <div class="mb-12">
                <h2 class="text-2xl font-bold mb-4">"featured projects"</h2>
                <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                    <Card>
                        <a class="underline font-bold text-xl" href="https://ultros.app">"ultros"</a>
                        <p class="mt-2">"a Final Fantasy XIV marketboard tool built with Leptos and SeaORM"</p>
                    </Card>
                    <Card>
                        <a class="underline font-bold text-xl" href="https://github.com/akarras/wall-a-bunga">"wall-a-bunga"</a>
                        <p class="mt-2">"a wallpaper downloader built with Iced, one of my first Rust projects"</p>
                    </Card>
                </div>
                <div class="mt-4">
                    <a class="underline text-lg hover:text-neutral-800 dark:hover:text-neutral-300" href="/projects">"see all projects →"</a>
                </div>
            </div>

            // Recent photos
            <div class="mb-12">
                <h2 class="text-2xl font-bold mb-4">"recent photos"</h2>
                <Card>
                    <SmallPhotos />
                </Card>
            </div>
        </div>
    }
}
