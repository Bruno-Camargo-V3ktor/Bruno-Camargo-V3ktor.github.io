use crate::route::Route;
use dioxus::prelude::*;
#[component]
pub fn NotFound(segments: Vec<String>) -> Element {
    rsx! { section { class: "section", h1 { "404" } p { "Essa página não foi encontrada." } Link { class: "button", to: Route::Home {}, "Voltar ao início" } } }
}
