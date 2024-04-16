use leptos::*;

use crate::repository::blog_repository::get_previews;
use crate::component::blog_preview_card::BlogPreviewCard;

#[component]
fn BlogDescription() -> impl IntoView {
    view! {
        <div class="p-5 flex flex-col items-center">
            <div class="mb-5 h-40 w-40 shadow-xl overflow-hidden rounded-full">
                // <img src="http://cttm.io/images/CodeToTheMoonV1Square.png" />
                <img src="assets/icon.png" />
            </div>
            <div class="p-2 text-4xl">"yiwufeng"</div>
            <div class="p-2 text-xl">"a blog write by leptos"</div>
        </div>
    }
} 

#[component]
pub fn BlogPreviews() -> impl IntoView {
    let post_resource = create_resource(
        || {}, 
        |_| async move {
            get_previews(None, None, 20, 10).await
        }
    );
    let previews_view = move || -> Option<Result<View, _>> {
        post_resource.and_then(|previews| {
            previews
                .into_iter()
                .map(|preview| {
                    view! {
                        <BlogPreviewCard post={preview.clone()} />
                    }
                })
                .collect_view()
        })
    };
    view! {
        <BlogDescription />
        <div class="dark:bg-gray-800 p-8 rounded-lg flex flex-wrap">
            <Suspense fallback=move || view! { <p> "Loading" </p>}>
                <ErrorBoundary fallback = move |_| view! {<p> "Error!" </p>}>
                    {previews_view}
                </ErrorBoundary>
            </Suspense>
        </div>
    }
}
