# Escrever sem mexer no Rust

1. Abra o arquivo JSON da categoria (`blog.json`, `cursos.json`, `projetos.json` ou `servicos.json`). Duplique um item, escolha um `slug` único e altere título, resumo, tags, imagem e destaques.
2. No campo `content_file`, indique um arquivo relativo a esta pasta, por exemplo `blog/meu-artigo.html`.
3. Crie esse arquivo e escreva o corpo em HTML. Não inclua `<html>`, `<head>` ou `<body>`: o site já monta a página.
4. Coloque imagens em `assets/media/` e use `src="media/minha-imagem.jpg"`. Use aspas duplas e arquivos diretamente nessa pasta. O build inclui e resolve os endereços automaticamente. A capa no JSON também pode ser `"image": "media/minha-imagem.jpg"`.
5. Execute `python3 scripts/validate_content.py` e depois `dx build --platform web --release`. Publique a pasta gerada. Nenhuma alteração em Rust é necessária.

## Exemplo de corpo HTML

```html
<section>
  <h2>O problema</h2>
  <p>Um texto com <strong>destaque</strong>, <em>ênfase</em> e um
     <a href="https://example.com">link de referência</a>.</p>
  <ul><li>Primeiro ponto</li><li>Segundo ponto</li></ul>
</section>
<figure>
  <img src="media/minha-imagem.jpg" alt="Descrição da imagem">
  <figcaption>Legenda opcional.</figcaption>
</figure>
<blockquote>Uma ideia que merece destaque.</blockquote>
<pre><code>fn main() {
    println!("Olá!");
}</code></pre>
<div class="callout"><p>Uma observação importante.</p></div>
<div class="columns">
  <section><h2>Contexto</h2><p>...</p></section>
  <section><h2>Resultado</h2><p>...</p></section>
</div>
<details>
  <summary>Módulo do curso</summary>
  <p>Descrição das aulas e objetivos.</p>
</details>
```

Também há estilos para `h3`, `ol`, `table`, `th`, `td`, `hr` e `video`. Para símbolos de código, escape `<` como `&lt;` e `&` como `&amp;`.

## Campos do catálogo

- `slug`: identificador sem espaços (ex.: `meu-artigo`). Define a URL.
- `title`, `summary`, `label`, `tags`: cabeçalho e cartão.
- `content_file`: corpo HTML local, obrigatório.
- `highlights`: lista da ficha lateral.
- `image`: capa; aceita `media/arquivo.jpg`, URL HTTPS ou uma ilustração existente (`code`, `network`, `design`, `systems`, `learning`, `audio`).
- `url`: destino público do projeto ou do curso.
- `repository`: link do repositório; `null` quando não houver.
- `demo`: metadado opcional para conteúdo de trabalho em andamento; não é exibido ao visitante.

O HTML é conteúdo autoral local e é incorporado como escrito; não use scripts, handlers JavaScript ou conteúdo não confiável. Não existe formulário público que aceite HTML. A atualização continua exigindo build/publicação; não há painel administrativo nem backend.

Os layouts se adaptam à categoria: leitura editorial para blog, módulos e ficha para cursos, capa ampla para projetos e chamada de contato para serviços.

## Certificações
Edite `certificacoes.json`: `title`, `area`, `skills`, `type`, `issuer`, `status` e os campos opcionais. O arquivo contém somente as duas formações em andamento. Os campos `demo` dos catálogos são metadados internos e não aparecem na interface.

### Campos dos certificados
`type`: tipo (curso livre, certificação técnica, graduação etc.); `issuer`: instituição; `issued_at`: data de emissão; `hours`: carga horária numérica; `credential_id`: identificação; `url`: link HTTPS para visualizar/verificar o certificado. Campos opcionais podem ficar `null` e não aparecem no card. O botão “Visualizar certificado” aparece quando `url` estiver preenchido.
