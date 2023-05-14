use leptos::*;
use crate::{blog::BlogList};
use crate::components::Card;


/// Renders the home page of your application.
#[component]
pub(crate) fn HomePage(cx: Scope) -> impl IntoView {
    view! { cx,
        <h1 class="text-3xl">"Welcome to aaron karras' personal home page!"</h1>
        <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
            <div class="col-span-2">
                "Recent blog posts:"
                <BlogList />
            </div>
            <div>
                <Card>
                    "Recent pictures:"
                    
                </Card>
                <Card>
                    "professional"
                    <ul class="list-disc">
                        <li>"Rust software engineer"</li>
                        <li>""</li>
                    </ul>
                </Card>
                <Card>
                    <div>
                    </div>
                    "currently addicted to:"
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
