use serde::Deserialize;
use serde::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FlickrGetPhotosResponse {
    pub photos: Photos,
    pub stat: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Photos {
    pub page: i64,
    pub pages: i64,
    pub perpage: i64,
    pub total: i64,
    pub photo: Vec<Photo>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Photo {
    pub id: String,
    pub owner: String,
    pub secret: String,
    pub server: String,
    pub farm: i64,
    pub title: String,
    #[serde(rename = "ispublic")]
    pub is_public: i64,
    #[serde(rename = "isfriend")]
    pub is_friend: i64,
    #[serde(rename = "isfamily")]
    pub is_family: i64,
}

pub enum PhotoSize {
    Large,
}

impl PhotoSize {
    fn get_suffix(&self) -> &'static str {
        match self {
            PhotoSize::Large => "b",
        }
    }
}

impl Photo {
    pub(crate) fn to_image_url(&self, size: Option<PhotoSize>) -> String {
        let server = &self.server;
        let id = &self.id;
        let secret = &self.secret;
        if let Some(size) = size {
            let suffix = size.get_suffix();
            format!("https://live.staticflickr.com/{server}/{id}_{secret}_{suffix}.jpg")
        } else {
            format!("https://live.staticflickr.com/{server}/{id}_{secret}.jpg")
        }
    }
}

#[cfg(feature = "ssr")]
pub(crate) async fn get_flickr_pictures(user_id: &str) -> Option<FlickrGetPhotosResponse> {
    use leptos::tracing::info;

    let api_key = std::env::var("FLICKR_KEY").expect("flickr api key env needs to be set");
    let url = format!("https://www.flickr.com/services/rest/?method=flickr.people.getPublicPhotos&api_key={api_key}&user_id={user_id}&format=json&nojsoncallback=1");
    let client = reqwest::Client::new();
    info!("fetching url {url}");
    client.get(url).send().await.ok()?.json().await.ok()
}
