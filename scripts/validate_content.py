"""Valida o contrato editorial e o limite de tamanho dos componentes."""
import json
import re
from pathlib import Path
from urllib.parse import urlparse
from html.parser import HTMLParser

class ContentImages(HTMLParser):
    def handle_starttag(self, tag, attrs):
        if tag not in ("img", "source", "video"):
            return
        for key, value in attrs:
            if key in ("src", "poster") and value and value.startswith("media/"):
                path = (root / "assets" / "media" / value.removeprefix("media/")).resolve()
                assert path.is_relative_to((root / "assets/media").resolve()) and path.is_file(), path

root = Path(__file__).resolve().parents[1]
import xml.etree.ElementTree as ET
for svg in (root / "assets").rglob("*.svg"):
    ET.parse(svg)
for path in (root / 'content').glob('*.json'):
    entries = json.loads(path.read_text())
    assert isinstance(entries, list), path
    if path.stem == "certificacoes":
        assert all(all(isinstance(item.get(key), str) and item[key].strip() for key in ("title", "area", "skills", "type", "status")) for item in entries)
        for item in entries:
            assert item.get("url") is None or (urlparse(item["url"]).scheme == "https" and urlparse(item["url"]).netloc)
            assert item.get("hours") is None or (isinstance(item["hours"], int) and item["hours"] > 0)
            for key in ("issuer", "issued_at", "credential_id"):
                assert item.get(key) is None or isinstance(item[key], str)
        continue
    slugs = set()
    for entry in entries:
        for field in ('slug', 'title', 'summary', 'content_file', 'label'):
            assert isinstance(entry[field], str) and entry[field].strip(), (path, field)
        html_path = (root / 'content' / entry['content_file']).resolve()
        assert html_path.is_relative_to((root / 'content').resolve()) and html_path.is_file(), html_path
        assert html_path.suffix == '.html'
        ContentImages().feed(html_path.read_text())
        assert re.fullmatch(r'[a-z0-9]+(?:-[a-z0-9]+)*', entry['slug']), path
        assert entry['slug'] not in slugs, (path, 'slug duplicado')
        slugs.add(entry['slug'])
        assert isinstance(entry['tags'], list)
        assert all(isinstance(tag, str) and tag.strip() for tag in entry['tags'])
        for field in ('url', 'repository', 'image'):
            value = entry[field]
            assert value is None or (field == 'image' and (value in {'code','network','design','systems','learning','audio'} or (value.startswith('media/') and (root / 'assets' / 'media' / value.removeprefix('media/')).is_file()))) or (urlparse(value).scheme == 'https' and urlparse(value).netloc), (path, field)
        for section in entry.get('sections', []):
            assert all(isinstance(section.get(key), str) and section[key].strip() for key in ('title', 'text'))
        assert isinstance(entry.get('demo', False), bool)
        if path.stem == "projetos":
            assert entry["repository"] and entry["repository"].startswith("https://github.com/Bruno-Camargo-V3ktor/")
        if path.stem == "cursos":
            assert len(entries) == 1 and entry["slug"] == "rust-essencial"
        assert all(isinstance(item, str) and item.strip() for item in entry.get('highlights', []))
        if path.stem == 'cursos':
            assert entry['url'] == "https://www.youtube.com/@v3ktor__"
for folder in ('components', 'pages'):
    for path in (root / 'src' / folder).rglob('mod.rs'):
        assert len(path.read_text().splitlines()) <= 100, path
print('Conteúdo, URLs, slugs e limite de linhas válidos.')

manifest = json.loads((root / 'assets/brand/manifest.json').read_text())
assert manifest['start_url'] == '/' and manifest['scope'] == '/'
assert {icon['sizes'] for icon in manifest['icons']} == {'192x192', '512x512'}
assert (root / 'assets/bruno.jpeg').is_file()
print('Manifest, foto e campos editoriais válidos.')
