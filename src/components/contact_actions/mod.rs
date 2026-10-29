use crate::components::arrow::Arrow;
use dioxus::prelude::*;
#[component]
pub fn ContactActions() -> Element {
    rsx! {
        document::Stylesheet { href: asset!("/assets/styles/components/contact_actions/styles.css") }
        div { class: "contact-actions",
            a { class: "button button-primary", href: "https://wa.me/5511996489713?text=Conte%20o%20essencial.%0A%0A1.%20Qual%20problema%20voc%C3%AA%20quer%20resolver%3F%0A%0A2.%20O%20que%20j%C3%A1%20existe%20e%20o%20que%20precisa%20ser%20criado%3F%0A%0A3.%20H%C3%A1%20um%20prazo%20ou%20or%C3%A7amento%20previsto%3F", target: "_blank", rel: "noopener noreferrer", "Conversar no WhatsApp" Arrow {} }
            a { class: "button", href: "mailto:contato@bluerosestudio.com.br", "Enviar um e-mail" Arrow {} }
        }
    }
}
