use crate::blog::BlogList;
use crate::components::Card;
use crate::pictures::SmallPhotos;
use leptos::*;

/// Renders the home page of your application.
#[component]
pub(crate) fn HomePage() -> impl IntoView {
    view! {
        <h1 class="text-3xl">"Welcome to Aaron Karras' personal home page!"</h1>
        <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
            <div class="col-span-2">
                <h2 class="text-xl font-bold">"Recent blog posts:"</h2>
                <BlogList />
            </div>
            <div>
                <div class="font-bold font-lg">"Recent photos"</div>
                <Card>
                    <SmallPhotos />
                </Card>
                <div class="font-bold font-lg">"professional"</div>
                <Card>
                    <ul class="list-disc">
                        <li>"Rustacean for the past two years"</li>
                        <li>"C++ networking in systems"</li>
                        <li>"building various side projects"</li>
                        <li>"chronically curious"</li>
                    </ul>
                </Card>
                <div class="font-bold font-lg">"in my brain"</div>
                <Card>
                    <ul class="list-disc">
                        <li>"gaming - playing ffxiv, overwatch, diablo 4"</li>
                        <li>"electric vehicles & charging infrastructure"</li>
                        <li>"photography"</li>
                    </ul>
                </Card>
            </div>
        </div>
    }
}
