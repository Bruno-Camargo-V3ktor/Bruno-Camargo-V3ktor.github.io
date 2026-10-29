use dioxus::prelude::*;
#[component]
pub fn Artwork(name: String, alt: String) -> Element {
    let source = match name.as_str() {
        "network" => asset!("/assets/illustrations/network.svg").to_string(),
        "design" => asset!("/assets/illustrations/design.svg").to_string(),
        "systems" => asset!("/assets/illustrations/systems.svg").to_string(),
        "learning" => asset!("/assets/illustrations/learning.svg").to_string(),
        "audio" => asset!("/assets/illustrations/audio.svg").to_string(),
        "code" => asset!("/assets/illustrations/code.svg").to_string(),
        _ => crate::data::media_url(&name).unwrap_or(name),
    };
    rsx! { img { src: source, alt, loading: "lazy", width: "1200", height: "700" } }
}
