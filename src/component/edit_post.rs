use chrono::NaiveDateTime;
use leptos::ev::{MouseEvent, SubmitEvent};
use leptos::*;
use leptos_router::*;
use serde::{Deserialize, Serialize};

use crate::component::toast::{ToastMessage, ToastType};
use crate::model::blog_post::Post;
use crate::component::blog_post::BlogPost;
use crate::repository::blog_repository::{delete_post, get_post, upsert_post};

#[derive(Params, Eq, PartialEq, Debug, Clone, Serialize, Deserialize)]
struct EditPostParams {
    post_id: Option<String>,
}

fn format_dt(datetime: NaiveDateTime) -> String {
    datetime.format("%Y-%m-%dT%H:%M").to_string()
}

#[component]
pub fn EditPost() -> impl IntoView {
    let params = use_params::<EditPostParams>();

    let post_resource = create_resource(
        move || params(), 
    
        |params| async move {
            match params {
                Ok(EditPostParams { post_id: Some(s) }) => get_post(s).await,
                _ => Ok(Post::default()),
            }
    });
    
    let action_upsert_post = create_action(|post: &Post| {
        let post = post.clone();
        async move {
            upsert_post(
                if post.id == "" {None} else { Some(post.id.clone())},
                post.dt.to_string(), 
                post.image_url.to_string(), 
                post.title.to_string(), 
                post.text.to_string(),
            ).await
        }
    });

    let action_delete_post = create_action(|id: &String| {
        let id = id.clone();
        async move {
            delete_post(id).await
        }
    });
    // let upsert_post = create_server_action::<UpsertPost>();
    

    let on_submit = move |ev: SubmitEvent| {ev.prevent_default(); post_resource.and_then(|post| action_upsert_post.dispatch(post.clone())); };
    let on_delete = move |ev: MouseEvent| {
        ev.prevent_default();
        post_resource.and_then(|post| action_delete_post.dispatch(post.id.clone()));
    };

    let set_toast: WriteSignal<ToastMessage> = expect_context();

    create_effect(move |_| {
        let delete_status = action_delete_post.value().get();
        if let Some(Ok(_)) = delete_status {
            set_toast.set(ToastMessage {
                message: String::from("Post deleted!"),
                toast_type: ToastType::Success,
                visible: true,
            });
            let navigate = use_navigate();
            navigate(format!("/",).as_str(), Default::default());
        }
    });

    
    create_effect(move |_| {
        let id = action_upsert_post.value().get();
        if let Some(Ok(id)) = id {
            // leptos::logging::log!("Get id: {}", id);
            set_toast.set(ToastMessage {
                message: String::from("Post saved!"),
                toast_type: ToastType::Success,
                visible: true,
            });
            let navigate = use_navigate();
            navigate(format!("/view/{}", id).as_str(), Default::default());
        }
    });

    view! {
        <Suspense fallback=move || view! {<p>"Loading ..."</p> }>
            <ErrorBoundary fallback = move |_| view! {<p> "Error!" </p> }>
                <div class="flex h-screen">
                        <div class="min-w-[50%] max-h-[90%] text-gray-200 dark:bg-gray-800 bg-gray-100 p-10 rounded-md">
                            <form on:submit=on_submit>
                                <label class="block mb-4">
                                    <span>Date</span>
                                    <input class="w-full mt-1 p-2 dark:bg-gray-700" type="datetime-local" id="dt" name="dt" 
                                        on:input=move |ev| {
                                            post_resource.update(|curr| {
                                                if let Some(Ok(post)) = curr {
                                                    post.dt = NaiveDateTime::parse_from_str(&event_target_value(&ev), "%Y-%m-%dT%H:%M").unwrap();
                                                }
                                            })
                                        }
                                        prop:value={move || post_resource.get().and_then(|res| res.map(|post| format_dt(post.dt)).ok())}
                                    />
                                </label>
                                <label class="block mb-4">
                                    <span>Title</span>
                                    <input class="w-full mt-1 p-2 dark:bg-gray-700" type="text" id="title" name="title" 
                                        on:input=move |ev| {
                                            post_resource.update(|curr| {
                                                if let Some(Ok(post)) = curr {
                                                    post.title = event_target_value(&ev);
                                                }
                                            })
                                        }
                                        prop:value={move || post_resource.get().and_then(|res| res.map(|post| post.title).ok())}
                                    />
                                </label>
                                <label class="block mb-4">
                                    <span>Image</span>
                                    <input class="w-full mt-1 p-2 dark:bg-gray-700" type="text" id="image_url" name="image_url" 
                                        on:input=move |ev| {
                                            post_resource.update(|curr| {
                                                if let Some(Ok(post)) = curr {
                                                    post.image_url = event_target_value(&ev);
                                                }
                                            })
                                        }
                                        prop:value={move || post_resource.get().and_then(|res| res.map(|post| post.image_url).ok())}
                                    />
                                </label>
                                <label class="block mb-4">
                                    <span>Text</span>
                                    <textarea class="w-full mt-1 p-2 dark:bg-gray-700" id="text" name="text" 
                                        on:input=move |ev| {
                                            post_resource.update(|curr| {
                                                if let Some(Ok(post)) = curr {
                                                    post.text = event_target_value(&ev);
                                                }
                                            })
                                        }
                                        prop:value={move || post_resource.get().and_then(|res| res.map(|post| post.text).ok())}
                                    />
                                </label>
                                <div class="flex justify-center pb-4">
                                    <input type="submit" value="Submit" class="mx-auto w-1/3 bg-blue-500 hover:bg-blue-700 text-white font-bold"/>
                                </div>
                                <div class="flex justify-center pb-4">
                                    <input type="button" value="Delete" class="mx-auto w-1/3 bg-red-500 hover:bg-red-700 text-white font-bold"
                                        on:click=on_delete/>
                                </div>
                            </form> 
                        </div>
                        <div>
                           {move || post_resource.and_then( |post| view! {<BlogPost post=post.clone()/>})}
                        </div>
                    </div>
            </ErrorBoundary>
        </Suspense>

    }
    
}