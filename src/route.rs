use crate::components::shell::Shell;
use crate::pages::{catalog::Catalog, detail::Detail, home::Home, info::Info, not_found::NotFound};
use dioxus::prelude::*;
#[derive(Routable, Clone, PartialEq, Debug)]
pub enum Route {
    #[layout(Shell)]
    #[route("/")]
    Home {},
    #[route("/explorar/:kind")]
    Catalog { kind: String },
    #[route("/conteudo/:kind/:slug")]
    Detail { kind: String, slug: String },
    #[route("/sobre/:topic")]
    Info { topic: String },
    #[route("/:..segments")]
    NotFound { segments: Vec<String> },
}
