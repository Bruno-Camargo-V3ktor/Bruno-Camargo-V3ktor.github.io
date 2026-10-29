# Blue Rose Studio

Site estático em Rust + Dioxus, com conteúdo autoral local em HTML e catálogos JSON. Sem backend ou painel administrativo.

## Rodar

```sh
dx serve --platform web
```

## Editar conteúdo sem alterar Rust

Consulte [content/GUIA.md](content/GUIA.md). Cada item tem metadados no JSON de sua categoria e um arquivo `.html` indicado por `content_file`. O build descobre esses arquivos automaticamente.

Exemplo: edite `content/blog/caminho-de-um-pacote.html` para alterar o artigo. Para incluir uma imagem, coloque-a em `assets/media/` e use `<img src="media/arquivo.jpg" alt="Descrição">` com aspas duplas. A pasta de mídia é empacotada pelo Dioxus; não use caminhos do seu computador no HTML.

O HTML é autoral e confiável, sem sanitização para permitir formatação. Não cole scripts ou HTML desconhecido. Não há entrada HTML pública. Mudanças exigem recompilar e publicar o site.

Os projetos apontam para repositórios públicos do GitHub. Textos de blog são artigos editoriais autorais; descrições de projetos e serviços não afirmam métricas ou clientes que não foram fornecidos. Capas são ilustrações conceituais, não capturas dos produtos.

## Publicar

O branch `source` contém o projeto Rust e o conteúdo editorial. Ao receber uma atualização, o workflow `.github/workflows/deploy.yml` compila o site e substitui o branch `main` pelos arquivos renderizados. No repositório, configure GitHub Pages para publicar a partir de `main` na raiz (`/(root)`). O workflow também gera `robots.txt`, `sitemap.xml`, rotas de página e `404.html`.

## Verificar e gerar

```sh
python3 scripts/validate_content.py
cargo fmt --check
cargo check
dx build --platform web --release
```

Publique a pasta `target/dx/blue-rose-studio/release/web/public` (ou o equivalente em `CARGO_TARGET_DIR`). Configure fallback das rotas para `index.html`, preservando arquivos reais, para permitir recarregar páginas internas.

## Estrutura

- `content/`: catálogos, HTML e mídia editorial.
- `build.rs`: agrega HTML e registra mídia local durante o build.
- `src/data/`: modelo e acesso ao conteúdo compilado.
- `src/pages/` e `src/components/`: módulos com CSS próprio; máximo de 100 linhas por componente.
- `assets/styles/global.css`: reset e variáveis globais.
- `assets/styles/`: CSS modular de componentes e páginas.
- `assets/`: imagens, ícones, ilustrações, manifest e todos os estilos empacotados.
- `assets/bruno.jpeg`: foto original; proporções e efeitos definidos no CSS.
- `assets/brand/`: favicon SVG, ícones PNG e manifest. O manifest usa ícones embutidos para sobreviver ao hashing dos assets; não inclui service worker/offline.

## Identidade e contato

Base preta, texto claro, bordas neutras e iluminação azul. A rosa do hero completa uma rotação durante o surgimento das pétalas; animações respeitam `prefers-reduced-motion`.

E-mail: `contato@bluerosestudio.com.br` (verificar alias no provedor).
WhatsApp: `https://wa.me/5511996489713`. O botão abre uma conversa com mensagem inicial preenchida.

Sobre e contato estão em `src/pages/about` e `src/pages/contact`. Termos e privacidade são textos iniciais em `src/pages/info`, a revisar conforme a operação e a hospedagem adotadas. As fontes usam a pilha local do sistema; o site não precisa buscar arquivos externos para renderizar sua tipografia.
