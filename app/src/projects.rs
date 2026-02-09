use leptos::prelude::*;

#[component]
pub(crate) fn Projects() -> impl IntoView {
    view! {
        <div class="py-8">
            <h3 class="text-6xl xl:text-9xl font-bold gradient-text mb-4">"Projects"</h3>
            <ul class="text-4xl p-4 space-y-2">
                <li><a class="text-amber-600 dark:text-teal-400 hover:underline" href="#ultros">"ultros"</a></li>
                <li><a class="text-amber-600 dark:text-teal-400 hover:underline" href="#xivsim">"XIV Crafting Optimizer Rust"</a></li>
                <li><a class="text-amber-600 dark:text-teal-400 hover:underline" href="#wallabunga">"wall-a-bunga"</a></li>
            </ul>
        </div>
        <div class="text-white">
            <div id="ultros" class="w-full h-screen relative bg-cover bg-no-repeat bg-fixed rounded-2xl overflow-hidden my-4" style="background-image: url(/assets/ultros_screenshot.png);">
                <div class="w-full h-screen absolute z-0 bg-gradient-to-b from-slate-900/90 from-5% to-slate-900/90 to-95% via-transparent">
                </div>
                <div class="z-10 absolute top-10 left-1/4 flex flex-col text-xl">
                    <a class="font-bold text-4xl xl:text-9xl pb-10 text-amber-400 hover:underline" href="https://ultros.app">
                        "ultros.app"
                    </a>
                    <p>"a Final Fantasy XIV marketboard tool"</p>
                    <p>"built with Leptos and SeaORM for tracking prices and sales across servers"</p>
                    <a class="text-teal-400 hover:underline mt-2" href="https://github.com/akarras/ultros">"github"</a>
                </div>
            </div>
            <div id="xivsim" class="w-full h-screen relative bg-cover bg-no-repeat bg-fixed rounded-2xl overflow-hidden my-4" style="background-image: url(/assets/xivcraftsim_screenshot.png);">
                <div class="w-full h-screen absolute z-0 bg-gradient-to-b from-slate-900/90 from-5% to-slate-900/90 to-95% via-transparent">
                </div>
                <div class="z-10 absolute top-10 left-1/4 flex flex-col text-xl">
                    <a class="font-bold text-4xl xl:text-9xl pb-10 text-amber-400 hover:underline" href="https://akarras.github.io/XIVCraftingOptimizer-rs/app/#/simulator">
                        "XIV Crafting Optimizer (in Rust)"
                    </a>
                    <p>"a tool for simulating crafting recipes and generating macros in Final Fantasy XIV"</p>
                    <p>"forked and rewritten in Rust with WebAssembly"</p>
                    <a class="text-teal-400 hover:underline mt-2" href="https://github.com/akarras/XIVCraftingOptimizer-rs">"github"</a>
                </div>
            </div>
            <div id="wallabunga" class="w-full h-screen relative bg-cover bg-no-repeat bg-fixed rounded-2xl overflow-hidden my-4" style="background-image: url(/assets/wall_a_bunga_screenshot.png);">
                <div class="w-full h-screen absolute z-0 bg-gradient-to-b from-slate-900/90 from-5% to-slate-900/90 to-95% via-transparent">
                </div>
                <div class="z-10 absolute top-10 left-1/4 flex flex-col text-xl">
                    <a class="font-bold text-4xl xl:text-9xl pb-10 text-amber-400 hover:underline" href="https://github.com/akarras/wall-a-bunga">
                        "wall-a-bunga"
                    </a>
                    <p>"a simple tool to download wallpapers"</p>
                    <p>"built with Iced, one of my first projects getting into Rust"</p>
                </div>
            </div>
        </div>
    }
}
