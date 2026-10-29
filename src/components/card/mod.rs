use crate::{
    components::{arrow::Arrow, logo::Logo},
    data::Entry,
    route::Route,
};
use dioxus::prelude::*;
#[component]
pub fn Card(entry: Entry, kind: String) -> Element {
    let destination = Route::Detail {
        kind: kind.clone(),
        slug: entry.slug.clone(),
    };
    rsx! {
        document::Stylesheet { href: asset!("/assets/styles/components/card/styles.css") }
        article { class: "content-card",
            Link { class: "card-art art-{entry.slug}", to: destination.clone(), aria_label: "Conheça {entry.title}",
                    if let Some(image) = &entry.image { crate::components::artwork::Artwork { name: image.clone(), alt: format!("Ilustração editorial: {}", entry.title) } }
                    else { div { class: "card-monogram", Logo { large: false } span { "{entry.title}" } small { "BLUE ROSE / {entry.label}" } } }
            }
            div { class: "card-meta", span { "{entry.label}" } Arrow {} }
            h3 { Link { to: destination, "{entry.title}" } }
            p { "{entry.summary}" }
            div { class: "tags", for tag in &entry.tags { span { "{tag}" } } }
            if kind == "cursos" { if let Some(url) = &entry.url { a { class: "text-link", href: "{url}", target: "_blank", rel: "noopener noreferrer", "Acessar curso" Arrow {} } } }
        }
    }
}
