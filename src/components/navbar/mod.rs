use crate::{components::brand::Brand, route::Route};
use dioxus::prelude::*;
#[component]
pub fn Navbar() -> Element {
    let mut open = use_signal(|| false);
    rsx! {
        document::Stylesheet { href: asset!("/assets/styles/components/navbar/styles.css") }
        header { class: "navbar",
            Brand {}
            button { class: "navbar-toggle", aria_expanded: "{open}", aria_controls: "navigation", onclick: move |_| open.toggle(), if open() { "Fechar" } else { "Menu" } }
            nav { id: "navigation", class: if open() { "navbar-links is-open" } else { "navbar-links" }, aria_label: "Navegação principal",
                for (label, kind) in [("Projetos", "projetos"), ("Serviços", "servicos"), ("Blog", "blog"), ("Cursos", "cursos")] {
                    Link { to: Route::Catalog { kind: kind.into() }, onclick: move |_| open.set(false), "{label}" }
                }
                Link { class: "navbar-contact", to: Route::Info { topic: "contato".into() }, onclick: move |_| open.set(false), "Vamos conversar" }
            }
        }
    }
}
