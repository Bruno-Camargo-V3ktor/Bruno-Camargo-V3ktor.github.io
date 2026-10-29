use crate::{
    components::{arrow::Arrow, logo::Logo},
    route::Route,
};
use dioxus::prelude::*;
#[component]
pub fn Hero() -> Element {
    rsx! {
        document::Stylesheet { href: asset!("/assets/styles/components/hero/styles.css") }
        section { class: "hero",
            div { class: "hero-topline", span { "ESTÚDIO INDEPENDENTE DE TECNOLOGIA" } span { "SÃO PAULO · BRASIL" } }
            div { class: "hero-art", Logo { large: true } }
            div { class: "hero-copy",
                p { class: "eyebrow", "CÓDIGO. FORMA. PROPÓSITO." }
                h1 { span { class: "hero-line", "Ideias que" } span { class: "hero-line", "ganham " em { "vida." } } }
                p { class: "hero-description", "Desenvolvimento, automação e design. Conecto tecnologia e criatividade para transformar problemas em experiências que fazem sentido." }
                div { class: "hero-actions",
                    Link { class: "button button-primary", to: Route::Catalog { kind: "projetos".into() }, "Explore os projetos" Arrow {} }
                    Link { class: "text-link", to: Route::Info { topic: "contato".into() }, "Vamos criar juntos" Arrow {} }
                }
            }
            div { class: "hero-bottom", span { "BRUNO CAMARGO FERREIRA / BLUE ROSE STUDIO" } span { "EXPLORE O ESTÚDIO" } }
        }
        div { class: "expertise-strip",
            span { "SOFTWARE" } Logo { large: false }
            span { "AUTOMAÇÃO" } Logo { large: false }
            span { "DESIGN" } Logo { large: false }
            span { "TECNOLOGIA COM INTENÇÃO" }
        }
    }
}
