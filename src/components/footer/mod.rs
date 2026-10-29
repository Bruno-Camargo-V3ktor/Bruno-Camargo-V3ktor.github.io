use crate::{components::brand::Brand, route::Route};
use dioxus::prelude::*;
#[component]
pub fn Footer() -> Element {
    rsx! {
        document::Stylesheet { href: asset!("/assets/styles/components/footer/styles.css") }
        footer { class: "footer",
            Brand {}
            p { "Tecnologia com intenção. Feito em Rust." }
            nav { aria_label: "Informações institucionais",
                Link { to: Route::Info { topic: "bruno".into() }, "Sobre" }
                Link { to: Route::Info { topic: "contato".into() }, "Contato" }
                Link { to: Route::Info { topic: "termos".into() }, "Termos de serviço" }
                Link { to: Route::Info { topic: "privacidade".into() }, "Privacidade" }
            }
            small { "© 2026 Blue Rose Studio" }
        }
    }
}
