use leptos::*;

use crate::model::blog_post::Post;

#[component]
pub fn BlogPost(post: Post) -> impl IntoView {
    let dt = format!("{}", post.dt.format("%Y-%m-%d %H:%M"));
    let image_preview = move ||  {
        if post.image_url.is_empty() {
           view! {}.into_view()
        } else {
            view! {
                <img class="w-96 h-32 rounded-lg object-cover my-10" src={&post.image_url} alt="Post thumbnail" />
            }.into_view()
        }
    };
    view! {
        <div class="block p-10">
            <div class="text-xl">{&dt}</div>
            {image_preview}
            // <img class="w-96 h-32 rounded-lg object-cover my-10" src={&post.image_url} alt="Post thumbnail" />
            <div class="text-4xl pb-4">{&post.title}</div>
            <div> {&post.text} </div>
        </div>
    }
}