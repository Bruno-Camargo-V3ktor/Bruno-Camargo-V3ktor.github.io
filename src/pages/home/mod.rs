use crate::{
    components::{arrow::Arrow, hero::Hero, section::Section},
    route::Route,
};
use dioxus::prelude::*;
#[component]
pub fn Home() -> Element {
    rsx! {
        document::Title { "Blue Rose Studio — Código, forma e propósito" }
        document::Meta { name: "description", content: "Desenvolvimento de software, automação e tecnologia por Bruno Camargo Ferreira. Conheça projetos, serviços e conteúdo da Blue Rose Studio." }
        document::Link { rel: "canonical", href: "https://bluerosestudio.com.br/" }
        document::Meta { property: "og:title", content: "Blue Rose Studio — Código, forma e propósito" }
        document::Meta { property: "og:description", content: "Desenvolvimento de software, automação e tecnologia por Bruno Camargo Ferreira." }
        document::Meta { property: "og:url", content: "https://bluerosestudio.com.br/" }
        document::Stylesheet { href: asset!("/assets/styles/pages/home/styles.css") }
        div { class: "home-flow",
        Hero {}
        Section { kind: "projetos", subtitle: "IDEIAS EM MOVIMENTO" }
        Section { kind: "servicos", subtitle: "O QUE PODEMOS CRIAR" }
        section { class: "home-about section",
            div { p { class: "eyebrow", "QUEM ESTÁ POR TRÁS" } h2 { "Prazer, Bruno." } img { class: "home-portrait", src: asset!("/assets/bruno.jpeg"), alt: "Bruno Camargo", loading: "lazy", width: "1254", height: "1254" } }
            div { p { class: "home-statement", "Entre a lógica do código e a liberdade de criar." } p { "Sou Bruno Camargo Ferreira, fundador da Blue Rose Studio e estudante de Ciência da Computação e Cibersegurança. Meu trabalho conecta desenvolvimento de software, infraestrutura e design." }
                Link { class: "text-link", to: Route::Info { topic: "bruno".into() }, "Veja mais" Arrow {} }
            }
        }
        Section { kind: "blog", subtitle: "NOTAS DO ESTÚDIO" }
        Section { kind: "cursos", subtitle: "CONHECIMENTO COMPARTILHADO" }
        section { class: "section home-certifications", p { class: "eyebrow", "APRENDIZADO CONTÍNUO" } h2 { "Certificações" } crate::components::certificates::Certificates {} Link { class: "text-link", to: Route::Info { topic: "certificacoes".into() }, "Veja mais" Arrow {} } }
        section { class: "section home-contact", p { class: "eyebrow", "PRÓXIMO PASSO" } h2 { "Uma boa ideia merece" br {} em { "uma conversa." } } crate::components::contact_actions::ContactActions {} Link { class: "text-link home-contact-more", to: Route::Info { topic: "contato".into() }, "Conheça os canais de contato" Arrow {} } }
        }
    }
}
