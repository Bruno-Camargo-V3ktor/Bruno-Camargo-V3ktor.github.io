"""Prepare a Dioxus SPA bundle for static hosting and search crawlers."""
import json
import os
from datetime import datetime, timezone
from pathlib import Path
from urllib.parse import urlparse

root = Path(__file__).resolve().parents[1]
site = os.environ.get("SITE_URL", "https://bluerosestudio.com.br").rstrip("/")
output = Path(os.environ.get("SITE_OUTPUT", root / "target/dx/blue-rose-studio/release/web/public"))
assert urlparse(site).scheme == "https" and urlparse(site).netloc, site
index = output / "index.html"
assert index.is_file(), f"Bundle não encontrado: {index}"
now = datetime.now(timezone.utc).date().isoformat()
routes = ["/", "/explorar/projetos", "/explorar/servicos", "/explorar/blog", "/explorar/cursos"]
for filename in ("projetos", "servicos", "blog", "cursos"):
    for item in json.loads((root / "content" / f"{filename}.json").read_text()):
        routes.append(f"/conteudo/{filename}/{item['slug']}")
routes += [f"/sobre/{slug}" for slug in ("bruno", "certificacoes", "contato", "termos", "privacidade")]
for route in routes:
    if route == "/":
        continue
    target = output / route.strip("/") / "index.html"
    target.parent.mkdir(parents=True, exist_ok=True)
    target.write_bytes(index.read_bytes())
(output / "404.html").write_bytes(index.read_bytes())
(output / ".nojekyll").write_text("")
(output / "CNAME").write_text(urlparse(site).netloc + "\n")
(output / "robots.txt").write_text(f"User-agent: *\nAllow: /\nSitemap: {site}/sitemap.xml\n")
items = "\n".join(f"  <url><loc>{site}{route}</loc><lastmod>{now}</lastmod></url>" for route in routes)
(output / "sitemap.xml").write_text(f'<?xml version="1.0" encoding="UTF-8"?>\n<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">\n{items}\n</urlset>\n')
print(f"Preparadas {len(routes)} URLs para {site}")
