use crate::{
    components::{arrow::Arrow, card::Card},
    data,
    route::Route,
};
use dioxus::prelude::*;
#[component]
pub fn Section(kind: String, subtitle: String) -> Element {
    let items = data::entries(&kind);
    rsx! {
        document::Stylesheet { href: asset!("/assets/styles/components/section/styles.css") }
        section { class: "section", id: "{kind}",
            div { class: "section-heading", p { class: "eyebrow", "{subtitle}" } h2 { "{data::title(&kind)}" } }
            if items.is_empty() {
                div { class: "empty-state", h3 { if kind == "blog" { "Conhecimento em construção." } else { "Aprender também faz parte." } } p { if kind == "blog" { "Artigos sobre tecnologia, desenvolvimento e os bastidores do estúdio. Em breve, por aqui." } else { "Cursos gratuitos e pagos terão seus links reunidos aqui assim que estiverem disponíveis." } } }
            } else {
                div { class: "card-grid", for entry in items.into_iter().take(3) { Card { entry, kind: kind.clone() } } }
            }
            Link { class: "text-link section-more", to: Route::Catalog { kind: kind.clone() }, "Veja mais" Arrow {} }
        }
    }
}
