use std::time::Duration;

use leptos::*;

#[derive(Clone)]
pub enum ToastType {
    Success,
    Error,
}

#[derive(Clone)]
pub struct ToastMessage {
    pub message: String,
    pub toast_type: ToastType,
    pub visible: bool,
}

#[component]
pub fn Toast() -> impl IntoView {
    let (toast, set_toast) = create_signal(ToastMessage {
        message: "".to_string(),
        toast_type: ToastType::Success,
        visible: false,
    });
    provide_context(set_toast);
    let base_toast_class = "fixed bottom-10 left-1/2 transform -translate-x-1/2 text-white px-4 py-2 rounded shadow-lg";

    let toast_classes = move || -> String {
        let t = toast.get();
        let background_class = match t.toast_type {
            ToastType::Success => "bg-green-500",
            ToastType::Error => "bg-red-500",
        };
        let opacity_class = if t.visible { "opacity-1" } else { "opacity-0" };

        format!("{} {} {}", base_toast_class, background_class, opacity_class)
    };
    create_effect(move |_| {
        let t = toast.get();
        if t.visible {
            set_timeout(
                move || {
                    set_toast.update(|msg| {
                        msg.visible = false;
                    });
                },
                Duration::new(4, 0),
            );
        }
    });
    view! {
        <div id="toast" class={toast_classes}>
            {move || toast.get().message}
        </div>
    }
}