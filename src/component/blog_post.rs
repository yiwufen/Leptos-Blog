use leptos::*;

use crate::model::blog_post::Post;

#[component]
pub fn BlogPost(post: Post) -> impl IntoView {
    view! {
        <div class="block p-10">
            <div class="text-4xl pb-4">{&post.title}</div>
            <div> {&post.text} </div>
        </div>
    }
}