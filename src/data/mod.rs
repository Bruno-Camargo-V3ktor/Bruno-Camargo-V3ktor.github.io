use dioxus::prelude::*;
use serde::Deserialize;

#[derive(Clone, PartialEq, Deserialize)]
pub struct Entry {
    pub slug: String,
    pub title: String,
    pub summary: String,

    #[serde(default)]
    pub html: String,

    pub tags: Vec<String>,
    pub label: String,
    pub url: Option<String>,
    pub repository: Option<String>,
    pub image: Option<String>,

    #[serde(default)]
    pub demo: bool,

    #[serde(default)]
    pub highlights: Vec<String>,
}

pub fn entries(kind: &str) -> Vec<Entry> {
    let source = match kind {
        "projetos" => include_str!(concat!(env!("OUT_DIR"), "/projetos.json")),
        "servicos" => include_str!(concat!(env!("OUT_DIR"), "/servicos.json")),
        "blog" => include_str!(concat!(env!("OUT_DIR"), "/blog.json")),
        "cursos" => include_str!(concat!(env!("OUT_DIR"), "/cursos.json")),
        _ => "[]",
    };
    serde_json::from_str(source).expect("Conteúdo local inválido")
}

pub fn title(kind: &str) -> &'static str {
    match kind {
        "projetos" => "Projetos",
        "servicos" => "Serviços",
        "blog" => "Blog",
        "cursos" => "Cursos",
        _ => "Conteúdo",
    }
}

pub fn valid(kind: &str) -> bool {
    ["projetos", "servicos", "blog", "cursos"].contains(&kind)
}

include!(concat!(env!("OUT_DIR"), "/media.rs"));
