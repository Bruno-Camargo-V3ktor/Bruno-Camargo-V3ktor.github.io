use crate::{components::logo::Logo, route::Route};
use dioxus::prelude::*;
#[component]
pub fn Brand() -> Element {
    rsx! {
        document::Stylesheet { href: asset!("/assets/styles/components/brand/styles.css") }
        Link { class: "brand", to: Route::Home {}, aria_label: "Blue Rose Studio — início",
            Logo { large: false }
            span { class: "brand-wordmark", span { "Blue Rose" }
                span { class: "brand-studio", aria_label: "Studio",
                    for letter in ["S", "T", "U", "D", "I", "O"] { span { aria_hidden: "true", "{letter}" } }
                }
            }
        }
    }
}
