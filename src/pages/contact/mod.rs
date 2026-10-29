use crate::components::contact_actions::ContactActions;
use dioxus::prelude::*;
#[component]
pub fn Contact() -> Element {
    rsx! {
        document::Title { "Contato — Blue Rose Studio" }
        document::Meta { name: "description", content: "Fale com Bruno Camargo sobre desenvolvimento, automação, redes, suporte e consultoria técnica." }
        document::Link { rel: "canonical", href: "https://bluerosestudio.com.br/sobre/contato" }
        document::Stylesheet { href: asset!("/assets/styles/pages/contact/styles.css") }
        section { class: "contact-page section",
            p { class: "eyebrow", "VAMOS CONVERSAR / SÃO PAULO · BRASIL" }
            h1 { "Seu próximo projeto" br {} em { "começa aqui." } }
            p { class: "contact-lead", "Uma ideia em rascunho, um processo que precisa melhorar ou um sistema que merece evoluir. Vamos entender o que você precisa." }
            ContactActions {}
            div { class: "contact-grid",
                article { span { "CONTATO DIRETO" } h2 { "Sem intermediários." } p { "Você conversa com Bruno, responsável pelo estúdio e pelo entendimento do seu projeto." } a { href: "https://wa.me/5511996489713", target: "_blank", rel: "noopener noreferrer", "+55 (11) 99648-9713" } a { href: "mailto:contato@bluerosestudio.com.br", "contato@bluerosestudio.com.br" } }
                article { span { "PARA COMEÇAR" } h2 { "Conte o essencial." } ol { li { "Qual problema você quer resolver?" } li { "O que já existe e o que precisa ser criado?" } li { "Há um prazo ou orçamento previsto?" } } }
            }
            aside { class: "contact-note", "Não precisa chegar com tudo definido. A primeira conversa serve para organizar as possibilidades e entender o próximo passo." }
        }
    }
}
