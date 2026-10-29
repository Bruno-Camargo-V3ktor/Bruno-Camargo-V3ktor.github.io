mod components;
mod data;
mod pages;
mod route;
use dioxus::prelude::*;
use route::Route;
fn main() {
    dioxus::launch(App);
}
#[component]
fn App() -> Element {
    rsx! {
        document::Stylesheet { href: asset!("/assets/styles/global.css") }
        document::Meta { name: "description", content: "Blue Rose Studio: desenvolvimento, automação e design por Bruno Camargo Ferreira." }
        document::Meta { property: "og:site_name", content: "Blue Rose Studio" }
        document::Meta { property: "og:type", content: "website" }
        document::Meta { property: "og:locale", content: "pt_BR" }
        document::Meta { name: "twitter:card", content: "summary_large_image" }
        document::Link { rel: "icon", r#type: "image/svg+xml", href: asset!("/assets/brand/favicon.svg") }
        document::Link { rel: "apple-touch-icon", href: asset!("/assets/brand/icon-192.png") }
        document::Link { rel: "manifest", href: asset!("/assets/brand/manifest.json") }
        document::Meta { name: "theme-color", content: "#07080a" }
        Router::<Route> {}
    }
}
