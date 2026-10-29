use crate::components::contact_actions::ContactActions;
use dioxus::prelude::*;
#[component]
pub fn About() -> Element {
    rsx! {
        document::Title { "Bruno Camargo — Blue Rose Studio" }
        document::Meta { name: "description", content: "Conheça Bruno Camargo Ferreira: profissional de TI, desenvolvedor full-stack, estudante de Ciência da Computação e Cibersegurança." }
        document::Link { rel: "canonical", href: "https://bluerosestudio.com.br/sobre/bruno" }
        document::Stylesheet { href: asset!("/assets/styles/pages/about/styles.css") }
        section { class: "about-page section",
            div { class: "about-intro",
                div { p { class: "eyebrow", "POR TRÁS DO ESTÚDIO / BRUNO CAMARGO" }
                    h1 { "Curioso por natureza." br {} em { "Criador por escolha." } }
                    p { class: "about-lead", "Atuo na área de TI da Albras Metalúrgica e me desenvolvo como profissional full-stack. Gosto de entender o que acontece por baixo da interface e de transformar necessidades em software claro e útil." }
                    p { "Sou Bruno Camargo Ferreira, fundador da Blue Rose Studio. Trabalho com tecnologias como Java, Spring Boot, JavaScript, TypeScript, React, React Native, Node.js, Python e bancos de dados. Também estudo Rust, redes e cibersegurança para ampliar meu repertório técnico." }
                    ContactActions {}
                }
                figure { class: "about-portrait", img { src: asset!("/assets/bruno.jpeg"), alt: "Bruno Camargo Ferreira", width: "1254", height: "1254" } figcaption { span { "BRUNO CAMARGO" } span { "DESENVOLVEDOR & FUNDADOR" } } }
            }
            div { class: "about-story", p { class: "eyebrow", "ALÉM DA SUPERFÍCIE" } h2 { "Baixo nível. Visão ampla." }
                p { "Tenho uma paixão especial por programação de baixo nível: memória, formatos binários, Assembly e arquitetura de computadores. Rust é uma das ferramentas com as quais exploro essa relação entre controle, segurança e desempenho." }
                p { "Redes também fazem parte dessa curiosidade. Gosto de estudar protocolos, observar o caminho dos dados e entender as camadas que fazem uma comunicação acontecer. HTTP/2, QUIC, Linux e parsers são temas que aparecem nos meus estudos e experimentos." }
                p { "Estudar, para mim, é construir: ler, testar uma hipótese, desmontar um problema e escrever algo que torne o aprendizado concreto. Essa mesma atenção aos fundamentos acompanha meu trabalho com APIs, automações e interfaces." }
            }
            div { class: "about-education", p { class: "eyebrow", "DUAS FORMAÇÕES, PERSPECTIVAS COMPLEMENTARES" } h2 { "Construir. Compreender. Proteger." }
                div { class: "education-grid",
                    article { span { "BACHARELADO" } h3 { "Ciência da Computação" } p { "Universidade São Judas Tadeu" } small { "Formação em andamento" } }
                    article { span { "TECNÓLOGO" } h3 { "Cibersegurança" } p { "Faculdade Impacta" } small { "Formação em andamento" } }
                }
            }
            div { class: "about-interests", p { class: "eyebrow", "O QUE ME MOVE" }
                for (title, text) in [("Sistemas por dentro", "Memória, concorrência, hardware e formatos binários."), ("Conexões que funcionam", "Protocolos, Linux, infraestrutura e integrações."), ("Clareza na experiência", "Interfaces e design que dão forma à complexidade.")] {
                    article { h3 { "{title}" } p { "{text}" } }
                }
            }
        }
    }
}
