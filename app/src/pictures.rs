use chrono::{DateTime, Utc};
use leptos::*;
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Picture {
    url: String,
    id: String,
    // date_taken: DateTime<Utc>
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Pictures {
    recent_pictures: Vec<Picture>,
}

#[server(GetPictures, "/api", "GetJSON")]
async fn get_pictures() -> Result<Pictures, ServerFnError> {
    let pictures = crate::flickr::get_flickr_pictures("198236541@N06")
        .await
        .ok_or(ServerFnError::ServerError(
            "Flickr request failed".to_string(),
        ))?;
    let photos = pictures
        .photos
        .photo
        .into_iter()
        .map(|p| Picture {
            url: p.to_image_url(Some(crate::flickr::PhotoSize::Large)),
            id: p.id,
        })
        .collect();
    Ok(Pictures {
        recent_pictures: photos,
    })
}

#[component]
pub(crate) fn Pictures(cx: Scope) -> impl IntoView {
    let recent_pictures = create_resource(cx, move || {}, move |_| get_pictures());

    view! { cx, <div>
        <a class="text-2xl font-bold" href="https://www.flickr.com/photos/198236541@N06">"flickr"</a>
        <div class="relative flex flex-row w-full snap-x snap-mandatory overflow-x-auto">
            <Suspense fallback=move || view!{cx, "loading"}>
                {move || {
                    let pictures = recent_pictures.read(cx);
                    // ignore errors for now
                    let pictures = pictures.map(|p| p.ok()).flatten();
                    pictures.map(|p| {
                        p.recent_pictures.into_iter().map(|p| view!{cx,
                            <a href=format!("https://www.flickr.com/photos/198236541@N06/{}/in/dateposted-public/", p.id) class="min-w-max snap-center snap-always p-5 hover:border-red-600 dark:hover:border-red-300">
                                <img src=p.url />
                            </a>
                        }).collect::<Vec<_>>()
                    })
                }}
            </Suspense>
        </div>

    </div>}
}
