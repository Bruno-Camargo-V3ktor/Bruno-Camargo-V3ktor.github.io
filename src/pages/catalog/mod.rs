use crate::{components::card::Card, data, pages::not_found::NotFound};
use dioxus::prelude::*;
#[component]
pub fn Catalog(kind: String) -> Element {
    rsx! { CatalogContent { key: "{kind}", kind } }
}

#[component]
fn CatalogContent(kind: String) -> Element {
    let mut query = use_signal(String::new);
    let mut tag = use_signal(|| "Todos".to_string());
    use_effect(use_reactive!(|(kind)| {
        let _ = kind;
        query.set(String::new());
        tag.set("Todos".to_string());
    }));
    if !data::valid(&kind) {
        return rsx! { NotFound { segments: vec![kind] } };
    }
    let items = data::entries(&kind);
    let mut tags: Vec<String> = items.iter().flat_map(|item| item.tags.clone()).collect();
    tags.sort();
    tags.dedup();
    let search = query().to_lowercase();
    let filtered: Vec<_> = items
        .into_iter()
        .filter(|item| {
            (tag() == "Todos" || item.tags.contains(&tag()))
                && format!("{} {} {}", item.title, item.summary, item.tags.join(" "))
                    .to_lowercase()
                    .contains(&search)
        })
        .collect();
    rsx! {
        document::Title { "{data::title(&kind)} — Blue Rose Studio" }
        document::Meta { name: "description", content: "Explore {data::title(&kind)} da Blue Rose Studio: projetos, serviços, artigos e cursos de tecnologia por Bruno Camargo Ferreira." }
        document::Link { rel: "canonical", href: "https://bluerosestudio.com.br/explorar/{kind}" }
        document::Meta { property: "og:title", content: "{data::title(&kind)} — Blue Rose Studio" }
        document::Meta { property: "og:url", content: "https://bluerosestudio.com.br/explorar/{kind}" }
        document::Stylesheet { href: asset!("/assets/styles/pages/catalog/styles.css") }
        section { class: "catalog section",
            crate::components::catalog_intro::CatalogIntro { kind: kind.clone() }
            div { class: "catalog-controls",
                label { r#for: "search", "Pesquisar" }
                input { id: "search", r#type: "search", placeholder: "Título, assunto ou tecnologia…", value: "{query}", oninput: move |event| query.set(event.value()) }
                label { r#for: "tag", "Filtrar por tema" }
                select { id: "tag", value: "{tag}", onchange: move |event| tag.set(event.value()), option { value: "Todos", "Todos" } for item in tags { option { key: "{item}", value: "{item}", "{item}" } } }
            }
            p { class: "catalog-count", aria_live: "polite", "{filtered.len()} resultado(s)" }
            if filtered.is_empty() { div { class: "empty-state", h2 { "Nada por aqui ainda." } p { "Experimente outra busca ou volte em breve para conferir as novidades." } } }
            div { class: "card-grid", for entry in filtered { Card { key: "{entry.slug}", entry, kind: kind.clone() } } }
        }
    }
}
