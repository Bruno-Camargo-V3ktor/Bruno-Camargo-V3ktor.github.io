use crate::data::Entry;
use dioxus::prelude::*;
#[component]
pub fn DetailBody(entry: Entry, kind: String) -> Element {
    rsx! {
        document::Stylesheet { href: asset!("/assets/styles/components/detail_body/styles.css") }
        div { class: "detail-layout",
            div { class: "detail-reading rich-content", dangerous_inner_html: crate::data::resolve_html(&entry.html) }
            aside { class: "detail-sidebar",
                p { class: "eyebrow", if kind == "cursos" { "SUA TRILHA" } else if kind == "projetos" { "FICHA DO PROJETO" } else { "SOBRE O CONTEÚDO" } }
                div { class: "detail-facts", h3 { "Em destaque" } ul { for highlight in &entry.highlights { li { "{highlight}" } } } }
                p { class: "detail-author", "Bruno Camargo Ferreira" br {} "Blue Rose Studio" }
            }
        }
    }
}
