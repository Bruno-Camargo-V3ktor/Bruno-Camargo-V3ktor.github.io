use crate::pages::not_found::NotFound;
use dioxus::prelude::*;
#[component]
pub fn Info(topic: String) -> Element {
    if topic == "bruno" {
        return rsx! { crate::pages::about::About {} };
    }
    if topic == "contato" {
        return rsx! { crate::pages::contact::Contact {} };
    }
    if topic == "certificacoes" {
        return rsx! { section { class: "section", h1 { "Certificações" } crate::components::certificates::Certificates {} } };
    }
    let (title, paragraphs) = match topic.as_str() {
        "bruno" => (
            "Tecnologia com olhar criativo.",
            vec![
                "Sou Bruno Camargo Ferreira, fundador da Blue Rose Studio. Estudo Ciência da Computação e atuo com desenvolvimento, infraestrutura e design.",
                "Tenho interesse especial no ecossistema Rust, na integração de sistemas e em interfaces que tornam a tecnologia mais acessível. Pondus, Eternize e Solaris são alguns dos projetos que fazem parte desse percurso.",
            ],
        ),
        "certificacoes" => (
            "Certificações",
            vec![
                "As certificações serão publicadas aqui com instituição emissora, data e endereço de verificação. Nenhuma certificação foi cadastrada ainda.",
            ],
        ),
        "contato" => (
            "Vamos criar algo juntos.",
            vec![
                "Conte sobre o problema que você quer resolver, o contexto do projeto e o prazo desejado.",
                "Entre em contato pelo e-mail contato@bluerosestudio.com.br.",
            ],
        ),
        "termos" => (
            "Termos de serviço",
            vec![
                "Este site apresenta o portfólio e os serviços da Blue Rose Studio. A apresentação de um serviço não constitui uma proposta comercial ou contratação automática.",
                "Escopo, valores, prazos, entregas e condições de cada projeto serão definidos em proposta e contrato específicos. Cursos e repositórios externos seguem as condições das respectivas plataformas.",
            ],
        ),
        "privacidade" => (
            "Privacidade",
            vec![
                "Esta versão não possui formulário de coleta, contas de usuário ou ferramentas de análise de visitantes. A pesquisa e os filtros de conteúdo são executados no navegador.",
                "O provedor de hospedagem poderá processar registros técnicos de acesso. Links externos levam a serviços com suas próprias políticas. Esta informação deverá ser atualizada quando a hospedagem e o canal de contato forem definidos.",
            ],
        ),
        _ => return rsx! { NotFound { segments: vec![topic] } },
    };
    rsx! {
        document::Title { "{title} — Blue Rose Studio" }
        document::Meta { name: "description", content: "Informações sobre {title} da Blue Rose Studio." }
        document::Link { rel: "canonical", href: "https://bluerosestudio.com.br/sobre/{topic}" }
        document::Stylesheet { href: asset!("/assets/styles/pages/info/styles.css") }
        section { class: "info section", p { class: "eyebrow", "BLUE ROSE STUDIO" } h1 { "{title}" }
            for paragraph in paragraphs { p { "{paragraph}" } }
            if topic == "contato" { a { class: "button button-primary", href: "mailto:contato@bluerosestudio.com.br", "Escrever para o estúdio" } }
        }
    }
}
