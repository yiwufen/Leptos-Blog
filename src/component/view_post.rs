use leptos::*;
use leptos_router::*;
use serde::{Deserialize, Serialize};

use crate::model::blog_post::Post;
use crate::component::blog_post::BlogPost;
use crate::repository::blog_repository::get_post;

#[derive(Params, Eq, PartialEq, Debug, Clone, Serialize, Deserialize)]
struct EditPostParams {
    post_id: Option<String>,
}

#[component]
pub fn ViewPost() -> impl IntoView {
    let params = use_params::<EditPostParams>();

    let post_resource = create_resource(
        move || params(), 
    
        |params| async move {
            match params {
                Ok(EditPostParams { post_id: Some(s) }) => get_post(s).await,
                _ => Ok(Post::default()),
            }
    });

    let post_view = move || {
        post_resource.and_then( |post| view! {
            <div class="w-full flex justify-center">
                <div class="max-w-[800]">
                    <div class="flex justify-center pt-10">
                        <a href={format!("/edit/{}", &post.id)}>Edit</a>
                    </div>
                    <BlogPost post=post.clone()/>
                </div>
            </div>
        })
    };
    
    


    view! {
        <Suspense fallback=move || view! {<p>"Loading ..."</p> }>
            <ErrorBoundary fallback = move |_| view! {<p> "Error!" </p> }>
                {post_view}
            </ErrorBoundary>
        </Suspense>

    }
    
}