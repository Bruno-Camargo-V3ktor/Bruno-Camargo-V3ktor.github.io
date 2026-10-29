use crate::{
    components::{footer::Footer, navbar::Navbar},
    route::Route,
};
use dioxus::prelude::*;
#[component]
pub fn Shell() -> Element {
    let route = use_route::<Route>();
    use_effect(use_reactive!(|(route)| {
        let _ = route;
        document::eval("window.scrollTo({top:0,left:0,behavior:'instant'});");
        document::eval(include_str!("../../../assets/scripts/motion.js"));
    }));
    rsx! {
        document::Stylesheet { href: asset!("/assets/styles/components/shell/styles.css") }
        a { class: "skip-link", href: "#main", "Pular para o conteúdo" }
        Navbar {}
        main { id: "main", Outlet::<Route> {} }
        Footer {}
    }
}
