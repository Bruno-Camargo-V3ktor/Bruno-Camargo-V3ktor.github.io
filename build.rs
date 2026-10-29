use std::{env, fs, path::Path};
fn main() {
    println!("cargo:rerun-if-changed=content");
    let output = env::var("OUT_DIR").unwrap();
    for kind in ["projetos", "servicos", "blog", "cursos"] {
        let source = fs::read_to_string(format!("content/{kind}.json")).unwrap();
        let mut entries: serde_json::Value = serde_json::from_str(&source).unwrap();
        for entry in entries
            .as_array_mut()
            .expect("O catálogo deve ser uma lista")
        {
            if let Some(file) = entry["content_file"].as_str() {
                let base = Path::new("content").canonicalize().unwrap();
                let path = base
                    .join(file)
                    .canonicalize()
                    .expect("HTML de conteúdo ausente");
                assert!(
                    path.starts_with(&base),
                    "O HTML deve estar dentro de content/"
                );
                entry["html"] = fs::read_to_string(path).unwrap().into();
            }
        }
        fs::write(format!("{output}/{kind}.json"), entries.to_string()).unwrap();
    }
    let mut assets =
        String::from("pub fn media_url(name: &str) -> Option<String> { match name {\n");
    for file in fs::read_dir("assets/media").unwrap() {
        let path = file.unwrap().path();
        if !path.is_file() {
            continue;
        }
        let name = path.file_name().unwrap().to_str().unwrap();
        if name.starts_with('.') {
            continue;
        }
        assets.push_str(&format!(
            "{:?} => Some(dioxus::prelude::asset!({:?}).to_string()),\n",
            format!("media/{name}"),
            format!("/assets/media/{name}")
        ));
    }
    assets.push_str("_ => None } }\n");
    assets.push_str(
        "pub fn resolve_html(source: &str) -> String { let mut html = source.to_string();\n",
    );
    for file in fs::read_dir("assets/media").unwrap() {
        let path = file.unwrap().path();
        if !path.is_file() {
            continue;
        }
        let name = path.file_name().unwrap().to_str().unwrap();
        if name.starts_with('.') {
            continue;
        }
        let token = format!("media/{name}");
        assets.push_str(&format!(
            "html = html.replace({:?}, &format!(\"\\\"{{}}\\\"\", media_url({:?}).unwrap()));\n",
            format!("\"{token}\""),
            token
        ));
    }
    assets.push_str("html }\n");
    fs::write(format!("{output}/media.rs"), assets).unwrap();
}
