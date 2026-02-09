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

#[derive(Copy, Clone, Serialize, Deserialize, Debug)]
pub enum PhotoSize {
    Large,
}

impl PhotoSize {
    #[cfg(feature = "ssr")]
    fn get_suffix(&self) -> &'static str {
        match self {
            PhotoSize::Large => "b",
        }
    }
}

impl Photo {
    #[cfg(feature = "ssr")]
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
pub(crate) async fn get_flickr_pictures(user_id: &'static str) -> Option<FlickrGetPhotosResponse> {
    use log::info;

    use crate::flickr::cache::FlickrApiCache;
    if let Some(pictures) = FlickrApiCache::get_cached_user_pictures(user_id).await {
        info!("returning cached flickr response");
        return Some(pictures);
    }
    let api_key = std::env::var("FLICKR_KEY").ok()?;
    let url = format!("https://www.flickr.com/services/rest/?method=flickr.people.getPublicPhotos&api_key={api_key}&user_id={user_id}&format=json&nojsoncallback=1");
    let client = reqwest::Client::new();
    info!("fetching url {url}");
    let pictures: Option<FlickrGetPhotosResponse> =
        client.get(url).send().await.ok()?.json().await.ok();
    if let Some(pictures) = &pictures {
        info!("setting cached flickr response");
        FlickrApiCache::set_cached_user_pictures(user_id, pictures.clone()).await;
    }
    pictures
}

#[cfg(feature = "ssr")]
mod cache {
    use retainer::Cache;
    use std::sync::Arc;
    use std::time::Duration;
    use tokio::sync::OnceCell;

    use super::FlickrGetPhotosResponse;

    pub(crate) struct FlickrApiCache;

    #[derive(Clone)]
    struct CacheImpl {
        cache: Arc<retainer::Cache<&'static str, FlickrGetPhotosResponse>>,
    }

    #[cfg(feature = "ssr")]
    impl FlickrApiCache {
        async fn get_cache() -> &'static CacheImpl {
            static INSTANCE: OnceCell<CacheImpl> = OnceCell::const_new();
            INSTANCE
                .get_or_init(|| async {
                    let i = CacheImpl {
                        cache: Arc::new(Cache::new()),
                    };
                    let clone = i.clone();
                    let _monitor = tokio::spawn(async move {
                        clone.cache.monitor(4, 0.25, Duration::from_secs(60)).await
                    });
                    i
                })
                .await
        }

        pub(crate) async fn get_cached_user_pictures(
            user_id: &'static str,
        ) -> Option<FlickrGetPhotosResponse> {
            let cache = Self::get_cache().await;
            cache.cache.get(&user_id).await.map(|c| c.to_owned())
        }

        pub(crate) async fn set_cached_user_pictures(
            user_id: &'static str,
            flickr: FlickrGetPhotosResponse,
        ) {
            let cache = Self::get_cache().await;
            cache
                .cache
                .insert(user_id, flickr, Duration::from_secs(60 * 60))
                .await;
        }
    }
}
