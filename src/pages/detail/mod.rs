use crate::{
    components::{
        arrow::Arrow, artwork::Artwork, contact_actions::ContactActions, detail_body::DetailBody,
        related::Related,
    },
    data,
    pages::not_found::NotFound,
    route::Route,
};
use dioxus::prelude::*;
#[component]
pub fn Detail(kind: String, slug: String) -> Element {
    let Some(item) = data::entries(&kind)
        .into_iter()
        .find(|item| item.slug == slug)
    else {
        return rsx! { NotFound { segments: vec![kind, slug] } };
    };
    rsx! {
        document::Title { "{item.title} — Blue Rose Studio" }
        document::Meta { name: "description", content: "{item.summary}" }
        document::Link { rel: "canonical", href: "https://bluerosestudio.com.br/conteudo/{kind}/{slug}" }
        document::Meta { property: "og:title", content: "{item.title} — Blue Rose Studio" }
        document::Meta { property: "og:description", content: "{item.summary}" }
        document::Meta { property: "og:type", content: if kind == "blog" { "article" } else { "website" } }
        document::Meta { property: "og:url", content: "https://bluerosestudio.com.br/conteudo/{kind}/{slug}" }
        document::Stylesheet { href: asset!("/assets/styles/pages/detail/styles.css") }
        article { class: "detail detail-{kind} section",
            Link { class: "text-link", to: Route::Catalog { kind: kind.clone() }, "{data::title(&kind)} / Voltar à coleção" }
            header { class: "detail-header", p { class: "eyebrow", "{item.label}" } h1 { "{item.title}" } p { class: "detail-lead", "{item.summary}" } div { class: "tags", for tag in &item.tags { span { "{tag}" } } } }
            if let Some(image) = &item.image { figure { class: "detail-image", Artwork { name: image.clone(), alt: format!("Ilustração editorial: {}", item.title) } figcaption { "Ilustração conceitual · Blue Rose Studio" } } }
            DetailBody { entry: item.clone(), kind: kind.clone() }
            div { class: "detail-links",
                if let Some(url) = &item.repository { a { class: "button", href: "{url}", target: "_blank", rel: "noopener noreferrer", "Abrir repositório" Arrow {} } }
                if let Some(url) = &item.url { a { class: "button button-primary", href: "{url}", target: "_blank", rel: "noopener noreferrer", "Abrir na plataforma" Arrow {} } }
            }
            if kind == "servicos" { section { class: "detail-cta", p { class: "eyebrow", "DO CONTEXTO À SOLUÇÃO" } h2 { "Vamos entender seu projeto?" } p { "Escopo, prazo e investimento são definidos a partir da sua necessidade." } ContactActions {} } }
            Related { kind: kind.clone(), slug }
        }
    }
}
