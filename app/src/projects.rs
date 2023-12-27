use leptos::*;

#[component]
pub(crate) fn Projects() -> impl IntoView {
    view! {
        <div>
            <h3 class="text-6xl xl:text-9xl">"Projects"</h3>
            <ul class="text-4xl p-4">
                <li><a href="#ultros">"ultros"</a></li>
                <li><a href="#xivsim">"XIV Crafting Optimizer Rust"</a></li>
                <li><a href="#wallabunga">"wall-a-bunga"</a></li>
            </ul>
        </div>
        <div class="text-white">
            <div id="ultros" class="w-full h-screen relative bg-cover bg-no-repeat bg-fixed" style="background-image: url(/assets/ultros_screenshot.png);">
                <div class="w-full h-screen absolute z-0 bg-gradient-to-b from-black from-5% to-black to-95% via-transparent">
                </div>
                <div class="z-10 absolute top-10 left-1/4 flex flex-col text-xl stroke-black">
                    <a class="underline font-bold text-4xl xl:text-9xl pb-10" href="https://ultros.app">
                        "ultros.app"
                    </a>
                    <p>"a final fantasy 14 marketboard tool"</p>
                    <p>"built with leptos and seaorm. still pretty jank."</p>
                    <a class="underline" href="https://github.com/akarras/ultros">"github"</a>
                </div>
            </div>
            <div id="xivsim" class="w-full h-screen relative bg-cover bg-no-repeat bg-fixed" style="background-image: url(/assets/xivcraftsim_screenshot.png);">
                <div class="w-full h-screen absolute z-0 bg-gradient-to-b from-black from-5% to-black to-95% via-transparent">
                </div>
                <div class="z-10 absolute top-10 left-1/4 flex flex-col text-xl stroke-black">
                    <a class="underline font-bold text-4xl xl:text-9xl pb-10" href="https://akarras.github.io/XIVCraftingOptimizer-rs/app/#/simulator">
                        "XIV Crafting Optimizer (in Rust)"
                    </a>
                    <p>"a tool for simulating crafting recipes & generating macros in final fantasy 14"</p>
                    <p>"forked from a different project- kind of want to remake tbh."</p>
                    <a class="underline" href="https://github.com/akarras/XIVCraftingOptimizer-rs">"github"</a>
                </div>
            </div>
            <div id="wallabunga" class="w-full h-screen relative bg-cover bg-no-repeat bg-fixed" style="background-image: url(/assets/wall_a_bunga_screenshot.png);">
                <div class="w-full h-screen absolute z-0 bg-gradient-to-b from-black from-5% to-black to-95% via-transparent">
                </div>
                <div class="z-10 absolute top-10 left-1/4 flex flex-col text-xl stroke-black">
                    <a class="underline font-bold text-4xl xl:text-9xl pb-10" href="https://github.com/akarras/wall-a-bunga">
                        "wall-a-bunga"
                    </a>
                    <p>"a simple tool to downlaod wallpapers."</p>
                    <p>"built with iced, was one of my first projects getting into rust."</p>
                </div>
            </div>
        </div>
    }
}
