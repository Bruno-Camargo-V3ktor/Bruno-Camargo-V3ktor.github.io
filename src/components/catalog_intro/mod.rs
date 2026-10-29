use crate::{components::artwork::Artwork, data};
use dioxus::prelude::*;
#[component]
pub fn CatalogIntro(kind: String) -> Element {
    let (headline, text, image) = match kind.as_str() {
        "projetos" => (
            "Ideias exploradas. Soluções construídas.",
            "Software, identidade e experimentação. Conheça o contexto, as decisões e as tecnologias por trás de cada trabalho.",
            "code",
        ),
        "servicos" => (
            "Tecnologia a serviço do seu próximo passo.",
            "Do primeiro diagnóstico à entrega: desenvolvimento, automação, infraestrutura e design conectados ao que você precisa resolver.",
            "network",
        ),
        "blog" => (
            "Notas de quem gosta de ir mais fundo.",
            "Programação, sistemas, redes e os bastidores do aprendizado. Um espaço para compartilhar perguntas, descobertas e ideias.",
            "systems",
        ),
        _ => (
            "Aprender criando. Entender fazendo.",
            "Trilhas de estudo sobre desenvolvimento e tecnologia. Conteúdos gratuitos e pagos, com acesso pela plataforma de cada curso.",
            "learning",
        ),
    };
    rsx! {
        document::Stylesheet { href: asset!("/assets/styles/components/catalog_intro/styles.css") }
        header { class: "catalog-intro",
            div { p { class: "eyebrow", "BLUE ROSE STUDIO / {data::title(&kind)}" } h1 { "{data::title(&kind)}" } h2 { "{headline}" } p { "{text}" } }
            div { class: "catalog-intro-art", Artwork { name: image, alt: "Ilustração editorial sobre tecnologia" } }
        }
    }
}
