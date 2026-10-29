use crate::{components::card::Card, data};
use dioxus::prelude::*;
#[component]
pub fn Related(kind: String, slug: String) -> Element {
    let items: Vec<_> = data::entries(&kind)
        .into_iter()
        .filter(|entry| entry.slug != slug)
        .take(3)
        .collect();
    rsx! {
        document::Stylesheet { href: asset!("/assets/styles/components/related/styles.css") }
        if !items.is_empty() { section { class: "related", p { class: "eyebrow", "CONTINUE EXPLORANDO" } h2 { "Outras perspectivas." } div { class: "card-grid", for entry in items { Card { entry, kind: kind.clone() } } } } }
    }
}
