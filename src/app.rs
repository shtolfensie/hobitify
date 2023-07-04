use leptos::leptos_dom::ev::{SubmitEvent};
use leptos::*;
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Serialize, Deserialize)]
struct GreetArgs<'a> {
    name: &'a str,
}

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    let (name, set_name) = create_signal(cx, String::new());
    let (greet_msg, set_greet_msg) = create_signal(cx, String::new());

    // let update_name = move |ev| {
    //     let v = event_target_value(&ev);
    //     set_name.set(v);
    // };

    // let greet = move |ev: SubmitEvent| {
    //     ev.prevent_default();
    //     spawn_local(async move {
    //         if name.get().is_empty() {
    //             return;
    //         }
    //
    //         let args = to_value(&GreetArgs { name: &name.get() }).unwrap();
    //         // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    //         let new_msg = invoke("greet", args).await.as_string().unwrap();
    //         set_greet_msg.set(new_msg);
    //     });
    // };

    view! { cx,
        <main class="container">
            <ul>
                <Habit />
                <Habit />
                <Habit />
                <Habit />
            </ul>
        </main>
    }
}


#[component]
fn Habit(cx: Scope) -> impl IntoView {

    view! { cx,
        <li>
            <div class="w-full rounded-lg border p-4 [&:has(svg)]:pl-11 [&>svg+div]:translate-y-[-3px] [&>svg]:absolute [&>svg]:left-4 [&>svg]:top-4 [&>svg]:text-foreground">
                <p class="mb-1 font-medium leading-none tracking-tight">"Habit"</p>
                <p class="text-sm [&_p]:leading-relaxed">"Description"</p>
            </div>
        </li>
    }

}
