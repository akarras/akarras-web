# A simple blog for a simple man (in Rust)
tags: rust, blog, blah
## initial goals

- build a blog
- use rust
- use leptos
- keep it sort of simple
- use tailwindcss

Why?

I like those things. And I don’t know tailwindcss at all and want to try it.

Generally, the advice is to use the right tool for the job. And that’s great advice. Being able to use lots of tools is a great skill.

I think I lack that skill somewhat. I get opinionated and I want to use my screw driver with ratcheting action and refuse to use any others.

So is this the right tool? Not really, but it’s interesting to me.

It’s a weakness, but it happens.

## Getting Started

### leptos

So how do we build a simple blog in Rust? A tutorial if you will.

```bash
cargo install cargo-leptos
cargo leptos new --git https://github.com/leptos-rs/start-axum-workspace
```

And then I just followed the prompts! Yep it’s that easy- if you ignore the fact that there was an issue with wasm-bindgen and I actually used this https://github.com/leptos-rs/cargo-leptos/pull/126 that will hopefully get merged eventually, and also manually changed the cargo.toml again, hopefully will be addressed in this other https://github.com/leptos-rs/start-axum-workspace/pull/1 soon, but in theory it should just be that easy.

### tailwind

I’ve never used tailwind! But here’s gist of getting tailwind working with cargo leptos.

1. Create a tailwind.css under style
2. Add the tailwind-input-file so that cargo-leptos knows where it’s at!

```
# [Optional] The source CSS file. If it ends with .sass or .scss then it will be compiled by dart-sass into CSS. The CSS is optimized by Lightning CSS before being written to <site-root>/<site-pkg>/app.css
style-file = "style/main.scss"

tailwind-input-file = "style/tailwind.css" # <--- Our new tailwind css file!

# Assets source dir. All files found here will be copied and synchronized to site-root.
# The assets-dir cannot have a s
```

1. Run cargo leptos serve once, and then modify our tailwind.config.js

```bash
/** @type {import('tailwindcss').Config} */
    module.exports = {
      content: {
        relative: true,
        files: ["*.html", "./app/src/**/*.rs"], // <-- this path needs to be modifid for our workspace
      },
```

And that’s it! Tailwind css should now just work, cargo-leptos installs the binary and everything for you.

## Leptos

### Building the app’s skeleton

I have a few ideas of what I want on my site- I recently picked up photography and think it’d be cool to have a page showcasing my recent photos. Of course, I don’t really want to host those, so I’m going to try and use Flickr’s API to just show the photos I’ve recently uploaded back onto my own site.

Then I want to start a blog, in-fact this is the first post on the blog. This is a meta post. Working on my technical writing, and being able to help inform others is a goal I’ve recently set for myself, so it’s a must have for the site!

And last but not least, I want a page dedicated to the abandonware that I’ve shoveled out in recent months. Mostly just random final fantasy 14 projects, but hey, gotta show something off.

If I were just listing off the routes one by one it’d look something like this….

- /photos
- /blog
- /projects

Fortunately, our code looks very similar!

```rust
<Routes>
    <Route path="" view=|cx| view! { cx, <HomePage/> }/>
    <Route path="blog" view=|cx| view! { cx, <Blog/> }>
        <Route path=":slug" view=|cx| view! { cx, <BlogItem/> } />
        <Route path="" view=|cx| view! { cx, <BlogList/> } />
    </Route>
    <Route path="projects" view=|cx| view! { cx, <Projects/>} />
    <Route path="pictures" view=|cx| view! { cx, <Pictures/>}/>
</Routes>
```

Just to get started, my preference is to build out the app’s skeleton first and then fill in the pieces bit by bit. Each component is just an empty Leptos component in each of the modules. Nice and tidy.

![Untitled](https://s3-us-west-2.amazonaws.com/secure.notion-static.com/b4bdb430-0a03-4457-bbec-99472bdf40fc/Untitled.png)

### Our first bit of HTML

HTML, the world’s most renown programming language, is not my strong suite.

I think the really smart thing to do here would be for me to go grab a tailwind template for a blog and fill it all in, but I also just want to learn the basics of tailwind here and having my blog feel like an engineer’s blog instead of a designers is just fine by me.

THIS seriously took me a few hours to get working right, and I’ll explain why as I go, but here is my beautiful nav bar, enjoy the HTML and bask in it’s glory.

```html
<nav class="p-4 flex flex-row align-items-middle justify-items-stretch gap-4">
    <A class="aria-current:font-bold" href="/" exact=true>"home"</A>
    <A class="aria-current:font-bold" href="blog">"blog"</A>
    <A class="aria-current:font-bold" href="projects">"projects"</A>
    <A class="aria-current:font-bold" href="pictures">"pictures"</A>
    <div class="grow"></div>
    <a href="https://www.linkedin.com/in/adkarras">"linkedin"</a>
    <a href="https://github.com/akarras">"github"</a>
    <a href="mailto:aaron@akarras.com">"email"</a>
</nav>
```

Incidentally, this is also the first bit of tailwind I ever wrote. I think coming from raw CSS, I like it! Most of the class names are fairly predictable, but I do need to figure out if I can get the class names to auto complete inside my leptos components within Visual Studio Code. Rumblings aside, the thing that took me the longest here was trying to get the aria-current tag to work correctly.

The idea here, is that leptos’s <A> tags, note BIG <A>, not little <a> are magic. When the route is selected, it will add the aria-current attribute. Typically, I’ve just been writing my CSS selector to select the aria-current tag. Ez pz. So I googled, aria-current tailwind tag, and my first finding was a [plugin](https://github.com/thoughtbot/tailwindcss-aria-attributes) that provided aria-current! Terrific I thought, but I just couldn’t ever get it to work. I could tell that tailwind was loading the plugin, but something was wrong, and I was stumped.

Eventually I just searched “aria” on tailwind’s site, and found [the issue](https://tailwindcss.com/docs/hover-focus-and-other-states#aria-states) which was a major facepalm for me. tailwind had added support for aria tags, JUST not aria-current by default. So, I just had to edit my tailwind config with to get support for the aria-curent tag within tailwind. 

```jsx
theme: {
        extend: {
          aria: {
            current: 'current'
          }
        },
      },
```

Success!

![Screenshot 2023-05-13 at 5.06.30 PM.png](https://s3-us-west-2.amazonaws.com/secure.notion-static.com/5f24ac96-c63f-4704-af90-ea9569501ea0/Screenshot_2023-05-13_at_5.06.30_PM.png)

I didn’t really have any idea of what design I wanted, or want, but I just expect now that tailwind will make it easier for me to land on something that looks professional.

## Blog bloggity bloggers

I want to keep the blog simple. I’ve gotten used to taking notes in Notion, and recently discovered that if you copy and paste information out of notion, it ends up going into a markdown format. So here’s my plan: serve the blog out of markdown files served in a directory. At this point you might be asking yourself- couldn’t this just be a statically generated site? And yes- yes it could, but I want to make it easy for me to add posts to the site. Maybe one day I could even just pull posts directly from the notion database I started and am currently writing this in. Maybe one day…

For now though, lets just focus on the task at hand. I’m going to store all the blog posts in a single directory, and host the site off a cheap VM to start. To add a new post to the site, I will just scp the blog post directly to the server. Ez pz.

To actually build my list of blog posts, I’m going to utilize leptos server actions.

```rust
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
```

And bam- just like that I have the ability to call `get_post` or `get_blog_posts` anywhere isomorphically- on the client it just turns into an API call, and on the server it just gets a future! Neat! I had to wrap the implementation of `load_from_directory` and `load_from_file` in `#[cfg(feature = "ssr")]` to prevent them from shipping to the wasm (the wasm client won’t have all of our blog posts, that’d just be silly), but other than that, it basically just works! Except the template didn’t have get() added by default since I’ve switched to the git branch at this point.

```rust
.route("/api/*fn_name", get(leptos_axum::handle_server_fns))
```

But once I added that (and called `BlogPost::register()` & `BlogPosts::register()` it all worked seamlessly! And I get to keep all my server actions right in my `[blog.rs](http://blog.rs)` file along side my leptos components. Neat!