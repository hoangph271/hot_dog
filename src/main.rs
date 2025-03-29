mod backend;
mod global_assets;

use backend::save_dog;
use dioxus::prelude::*;
use global_assets::CSS;

fn main() {
    dioxus::launch(App);
}

#[component]
fn Title() -> Element {
    rsx! {
        div {
            id: "title",
            h1 { "HotDog! 🌭" }
        }
    }
}

#[derive(serde::Deserialize)]
struct DogApiResponse {
    message: String,
}

async fn fetch_dog_image() -> String {
    let response = reqwest::get("https://dog.ceo/api/breeds/image/random")
        .await
        .unwrap()
        .json::<DogApiResponse>()
        .await
        .unwrap();

    response.message
}

#[component]
fn DogView() -> Element {
    let mut img_src_resource = use_resource(fetch_dog_image);
    let mut is_saving = use_signal(|| false);

    let mut fetch_new = move || {
        img_src_resource.restart();
    };

    let mut handle_save_dog = move || {
        if let Some(img_src) = img_src_resource.cloned() {
            is_saving.set(true);

            spawn(async move {
                _ = save_dog(img_src).await;

                is_saving.set(false);

                fetch_new();
            });
        }
    };

    let mut handle_skip_dog = move || {
        img_src_resource.restart();
    };

    let img_src = img_src_resource.cloned().unwrap_or_default();

    let is_busy = use_memo(move || *is_saving.read());

    rsx! {
        div {
            id: "dogview",
            img {
                src: "{img_src}"
            }
        }
        div {
            id: "buttons",
            button {
                id: "skip",
                disabled: is_busy,
                onclick: move |_| handle_skip_dog(),
                "#skip"
            }
            button {
                id: "save",
                disabled: is_busy,
                onclick: move |_| handle_save_dog(),
                if is_busy() {
                    "..."
                } else {
                    "#save"
                }
            }
        }
    }
}

#[component]
fn App() -> Element {
    rsx! {
        document::Stylesheet { href: CSS }

        div {
            id: "App",
            Title {}
            DogView {}
        }
    }
}
