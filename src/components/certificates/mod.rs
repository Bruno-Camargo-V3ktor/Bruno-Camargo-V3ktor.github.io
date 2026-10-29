use dioxus::prelude::*;
use serde::Deserialize;
#[derive(Deserialize)]
struct Certificate {
    title: String,
    area: String,
    skills: String,
    status: String,
    #[serde(rename = "type")]
    category: String,
    issuer: Option<String>,
    issued_at: Option<String>,
    hours: Option<u32>,
    credential_id: Option<String>,
    url: Option<String>,
}
#[component]
pub fn Certificates() -> Element {
    let items: Vec<Certificate> =
        serde_json::from_str(include_str!("../../../content/certificacoes.json"))
            .expect("Certificações inválidas");
    rsx! {
        document::Stylesheet { href: asset!("/assets/styles/components/certificates/styles.css") }
        div { class: "certificate-grid",
            for item in items {
                article { class: "certificate-card",
                    small { class: "certificate-status", "{item.status}" }
                    crate::components::logo::Logo { large: false }
                    p { class: "eyebrow", "{item.area}" }
                    h3 { "{item.title}" }
                    p { class: "certificate-skills", "{item.skills}" }
                    dl { class: "certificate-details",
                        div { dt { "Tipo" } dd { "{item.category}" } }
                        if let Some(issuer) = item.issuer { div { dt { "Instituição" } dd { "{issuer}" } } }
                        if let Some(date) = item.issued_at { div { dt { "Emissão" } dd { "{date}" } } }
                        if let Some(hours) = item.hours { div { dt { "Carga horária" } dd { "{hours} horas" } } }
                        if let Some(id) = item.credential_id { div { dt { "Credencial" } dd { "{id}" } } }
                    }
                    if let Some(url) = item.url { a { class: "text-link certificate-link", href: "{url}", target: "_blank", rel: "noopener noreferrer", "Visualizar certificado" crate::components::arrow::Arrow {} } }
                }
            }
        }
    }
}
