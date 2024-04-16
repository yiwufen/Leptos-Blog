use  leptos::*;

use crate::model::blog_post::Post;

#[component]
pub fn BlogPreviewCard(post: Post) -> impl IntoView {
    let dt = format!("{}", post.dt.format("%Y-%m-%d %H:%M"));
    view! {
        <a href={format!("view/{}", post.id)}>
            <div class="transform transition duration-300 hover:scale-105 hover:shadow-2xl dark:bg-gray-600 p-6 rounded-lg mx-4 my-2 w-96 h-48 flex">
                <img src={post.image_url} alt="Blog thumbnail" class="w-32 h-32 rounded-lg object-cover mr-4" />
                <div class="flex flex-col">
                    <h2 class="text-xl font-semibold mb-2 w-48 h-10 truncate">{post.title}</h2>
                    <p class="dark:text-gray-200 mb-4 w-48 h-18">{post.text} </p>

                    <div class="flex justify-between">
                        <span class="dark:text-gray-200">{dt}</span>
                    </div>
                </div>
            </div>
        </a>
    }
}