use crate::components::*;
use chrono::offset::Utc;
use chrono::DateTime;
use leptos::*;
use leptos_router::*;
use serde::{Deserialize, Serialize};

#[cfg(feature = "ssr")]
fn get_blog_directory() -> String {
    std::env::var("BLOG_DIRECTORY")
        .clone()
        .unwrap_or("./blog".to_string())
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct PostDetails {
    title: String,
    slug: String,
    tags: Vec<String>,
    /// first few hundred words of the post
    peek: String,
    created: DateTime<Utc>,
    modified: DateTime<Utc>,
}

impl PostDetails {
    #[cfg(feature = "ssr")]
    async fn get_post_details(path: &std::path::PathBuf) -> Option<Self> {
        use itertools::Itertools;
        use tokio::io::AsyncReadExt;

        let mut post = tokio::fs::File::open(path).await.ok()?;
        let meta = post.metadata().await.ok()?;
        let created = meta.created().ok()?;
        let modified = meta.modified().ok()?;
        let file_name = path.file_name()?;
        let (path, extension) = file_name.to_str()?.split_once(".")?;
        let mut post_content = String::new();
        post.read_to_string(&mut post_content).await.ok()?;
        let mut lines = post_content.lines();
        let first_line = lines.next()?;
        let second_line = lines.next()?;
        // title line should have a # at the start. If it does, trim it and make that the title
        let title = if first_line.chars().next() == Some('#') {
            first_line[1..].trim().to_string()
        } else {
            first_line.trim().to_string()
        };
        let tags = if second_line.starts_with("tags:") {
            second_line[6..].split(",").map(|s| s.trim().to_string()).collect()
        } else {
            vec![]
        };
        let peek : String = lines.take(10).join("\n");
        (extension == "md").then(|| {})?;
        Some(PostDetails {
            title,
            slug: path.to_string(),
            created: created.into(),
            modified: modified.into(),
            tags: tags,
            peek,
        })
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Post {
    content: String,
    details: PostDetails,
}

impl Post {
    #[cfg(feature = "ssr")]
    async fn load_from_file(slug: &str) -> Option<Self> {
        use std::{str::FromStr, path::PathBuf};

        use itertools::Itertools;
        let mut path = PathBuf::from_str(&get_blog_directory()).unwrap();
        path.push(format!("{slug}.md"));
        let content = tokio::fs::read_to_string(&path).await.ok()?;
        let details = PostDetails::get_post_details(&path).await?;
        let content = content.lines().skip(2).join("\n");
        Some(Self { content, details })
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Blog {
    posts: Vec<PostDetails>,
}

impl Blog {
    #[cfg(feature = "ssr")]
    async fn load_from_directory() -> Option<Self> {
        let mut dir = tokio::fs::read_dir(get_blog_directory()).await.ok()?;
        let mut posts = vec![];
        while let Some(file_entry) = dir.next_entry().await.ok()? {
            if let Some(details) = PostDetails::get_post_details(&file_entry.path()).await {
                posts.push(details);
            }
        }
        Some(Self { posts })
    }
}

#[server(BlogPosts, "/api", "GetJSON")]
pub(crate) async fn get_blog_posts() -> Result<Blog, ServerFnError> {
    Blog::load_from_directory()
        .await
        .ok_or(ServerFnError::ServerError(
            "Unable to get blog posts".to_string(),
        ))
}

#[server(BlogPost, "/api", "GetJSON")]
pub(crate) async fn get_post(slug: String) -> Result<Post, ServerFnError> {
    Post::load_from_file(&slug)
        .await
        .ok_or(ServerFnError::ServerError(format!(
            "Unable to get post with slug: {slug}"
        )))
}

#[component]
pub(crate) fn Blog(cx: Scope) -> impl IntoView {
    view! {cx, <div class="grid grid-cols-1 md:grid-cols-2">
            <Outlet/>
        </div>
    }
}

#[component]
pub(crate) fn BlogList(cx: Scope) -> impl IntoView {
    let blog_posts = create_resource(cx, || {}, |()| async move { get_blog_posts().await });
    view! {cx,
        <Suspense fallback=move || view!{cx, "Loading"}>
            {move || {
                let blog = blog_posts.read(cx).map(|p| p.ok()).flatten();
                blog.map(move |blog| {
                    blog.posts.into_iter().map(move |post| {
                        view!{cx,
                            <A href=format!("/blog/{}", post.slug)>
                                <Card>
                                    <span class="text-3xl font-bold">{&post.title}</span>
                                    <div class="flex flex-row gap-2">
                                        <span>"tags:"</span>
                                        {post.tags.into_iter().map(|tag| view!{cx, <span class="">{tag}</span>}).collect::<Vec<_>>()}
                                    </div>
                                    <Markdown text=post.peek />
                                    "..."
                                    <div class="text-lg font-bold">"read more"</div>
                                </Card>
                            </A>
                        }
                    }).collect::<Vec<_>>()
                })
            }}
        </Suspense>
    }
}

#[component]
pub(crate) fn BlogItem(cx: Scope) -> impl IntoView {
    let params = use_params_map(cx);
    let post = create_resource(
        cx,
        move || params.with(|p| p.get("slug").cloned()),
        move |slug| async move {
            if let Some(slug) = slug {
                get_post(slug).await
            } else {
                Err(ServerFnError::MissingArg("No slug provided".to_string()))
            }
        },
    );
    view! {cx,
        <Suspense fallback=move || view!{cx, "Loading"}>
            {move || {
                    let post = post.read(cx);
                    // ignoring errors for now
                    let post = post.map(|p| p.ok()).flatten();
                    post.map(|post| {
                        view!{cx, <div>
                            <div class="text-4xl font-bold">{post.details.title}</div>
                            <Markdown text=post.content />
                        </div>}
                    })
                }
            }
        </Suspense>
    }
}
