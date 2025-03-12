use find::Find;
use leptos::task::spawn_local;
use leptos::{ev::SubmitEvent, prelude::*};
use pages::DisplayFindPage;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use std::default::Default;

pub mod find;
pub mod pages;



#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Serialize, Deserialize)]
struct GreetArgs<'a> {
    name: &'a str,
}

#[component]
pub fn App() -> impl IntoView {

    let (find, _) = signal(Find::default());

    view! {
        <main>
            <h1>"Welcome to Magnifungi"</h1>

            <DisplayFindPage
                find=find
            />

        </main>
    }
}
