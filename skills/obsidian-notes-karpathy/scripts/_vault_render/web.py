from __future__ import annotations

import json
from html import escape
from pathlib import Path
from typing import Any

from _vault_common import MarkdownRecord

from .common import (
    now_iso,
    resolve_title,
    slugify_title,
    source_live_pages_from_records,
)


def _shell_html(title: str) -> str:
    escaped_title = escape(title)
    return f"""<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <meta name="viewport" content="width=device-width, initial-scale=1">
  <title>{escaped_title}</title>
  <link rel="stylesheet" href="./styles.css">
</head>
<body>
  <div class="app-shell">
    <aside class="sidebar">
      <div class="sidebar-header">
        <p class="eyebrow">KB Query Web Export</p>
        <h1 id="site-title">{escaped_title}</h1>
      </div>
      <div class="sidebar-section">
        <h2>Pages</h2>
        <nav id="page-list" class="page-list"></nav>
      </div>
      <div class="sidebar-section">
        <h2>Grounding</h2>
        <ul id="grounding-list" class="grounding-list"></ul>
      </div>
      <div class="sidebar-section">
        <h2>Status</h2>
        <dl id="status-summary" class="status-summary"></dl>
      </div>
    </aside>
    <main class="content">
      <header class="content-header">
        <p id="page-kind" class="page-kind"></p>
        <h2 id="page-title">Loading…</h2>
        <p id="page-path" class="page-path"></p>
      </header>
      <section class="card">
        <h3>Source Live Pages</h3>
        <ul id="page-sources" class="source-list"></ul>
      </section>
      <section class="card">
        <h3>Excerpt</h3>
        <p id="page-excerpt" class="page-excerpt"></p>
      </section>
      <article id="page-body" class="article-body"></article>
    </main>
  </div>
  <script src="./app.js"></script>
</body>
</html>
"""


def _css() -> str:
    return """* { box-sizing: border-box; }
body {
  margin: 0;
  font-family: Inter, "Segoe UI", sans-serif;
  background: #0b1020;
  color: #e5e7eb;
}
.app-shell {
  min-height: 100vh;
  display: grid;
  grid-template-columns: 320px 1fr;
}
.sidebar {
  border-right: 1px solid #1f2937;
  background: #111827;
  padding: 24px 20px;
}
.sidebar-header h1,
.content-header h2,
.card h3,
.sidebar-section h2 { margin: 0; }
.sidebar-section { margin-top: 28px; }
.sidebar-section h2,
.card h3 {
  margin-bottom: 12px;
  font-size: 0.95rem;
  color: #93c5fd;
}
.eyebrow, .page-kind {
  text-transform: uppercase;
  letter-spacing: 0.08em;
  font-size: 0.72rem;
  color: #60a5fa;
}
.page-list {
  display: flex;
  flex-direction: column;
  gap: 8px;
}
.page-list button {
  text-align: left;
  border: 1px solid #1f2937;
  background: #0f172a;
  color: #e5e7eb;
  padding: 10px 12px;
  border-radius: 10px;
  cursor: pointer;
}
.page-list button.active {
  border-color: #60a5fa;
  background: #172554;
}
.grounding-list,
.source-list {
  margin: 0;
  padding-left: 18px;
}
.status-summary {
  display: grid;
  grid-template-columns: auto 1fr;
  gap: 8px 12px;
  margin: 0;
}
.status-summary dt { color: #93c5fd; }
.status-summary dd { margin: 0; }
.content {
  padding: 28px;
  display: grid;
  gap: 18px;
}
.content-header {
  display: grid;
  gap: 8px;
}
.page-path {
  margin: 0;
  color: #9ca3af;
  font-family: "Cascadia Code", monospace;
  font-size: 0.85rem;
}
.card,
.article-body {
  border: 1px solid #1f2937;
  background: #111827;
  border-radius: 16px;
  padding: 20px;
}
.article-body {
  line-height: 1.7;
}
.article-body h1,
.article-body h2,
.article-body h3 {
  color: #f8fafc;
}
.article-body pre {
  overflow-x: auto;
  padding: 12px;
  border-radius: 12px;
  background: #020617;
}
.article-body code {
  font-family: "Cascadia Code", monospace;
}
.article-body p,
.article-body ul { margin: 0 0 14px; }
.article-body ul { padding-left: 20px; }
.page-excerpt { margin: 0; color: #cbd5e1; }
@media (max-width: 900px) {
  .app-shell { grid-template-columns: 1fr; }
  .sidebar { border-right: 0; border-bottom: 1px solid #1f2937; }
}
"""


def _app_js() -> str:
    return """const escapeHtml = (value) =>
  value
    .replace(/&/g, "&amp;")
    .replace(/</g, "&lt;")
    .replace(/>/g, "&gt;")
    .replace(/"/g, "&quot;")
    .replace(/'/g, "&#39;");

const formatInline = (line) => {
  let text = escapeHtml(line);
  text = text.replace(/`([^`]+)`/g, "<code>$1</code>");
  text = text.replace(/\\*\\*([^*]+)\\*\\*/g, "<strong>$1</strong>");
  text = text.replace(/\\[\\[([^\\]]+)\\]\\]/g, "<code>[[$1]]</code>");
  return text;
};

const renderMarkdown = (markdown) => {
  const lines = markdown.split(/\\r?\\n/);
  const html = [];
  let inList = false;
  let inCode = false;

  const closeList = () => {
    if (inList) {
      html.push("</ul>");
      inList = false;
    }
  };

  for (const rawLine of lines) {
    const line = rawLine ?? "";
    if (line.startsWith("```")) {
      closeList();
      if (inCode) {
        html.push("</code></pre>");
        inCode = false;
      } else {
        html.push("<pre><code>");
        inCode = true;
      }
      continue;
    }
    if (inCode) {
      html.push(`${escapeHtml(line)}\\n`);
      continue;
    }
    if (!line.trim()) {
      closeList();
      continue;
    }
    if (line.startsWith("# ")) {
      closeList();
      html.push(`<h1>${formatInline(line.slice(2))}</h1>`);
      continue;
    }
    if (line.startsWith("## ")) {
      closeList();
      html.push(`<h2>${formatInline(line.slice(3))}</h2>`);
      continue;
    }
    if (line.startsWith("### ")) {
      closeList();
      html.push(`<h3>${formatInline(line.slice(4))}</h3>`);
      continue;
    }
    if (line.startsWith("- ")) {
      if (!inList) {
        html.push("<ul>");
        inList = true;
      }
      html.push(`<li>${formatInline(line.slice(2))}</li>`);
      continue;
    }
    closeList();
    html.push(`<p>${formatInline(line)}</p>`);
  }

  closeList();
  if (inCode) {
    html.push("</code></pre>");
  }
  return html.join("\\n");
};

const renderList = (items, elementId, emptyLabel) => {
  const root = document.getElementById(elementId);
  root.innerHTML = "";
  if (!items.length) {
    const li = document.createElement("li");
    li.textContent = emptyLabel;
    root.appendChild(li);
    return;
  }
  for (const item of items) {
    const li = document.createElement("li");
    li.textContent = item;
    root.appendChild(li);
  }
};

const manifestPromise = fetch("./manifest.json").then((response) => response.json());

manifestPromise.then(async (manifest) => {
  document.getElementById("site-title").textContent = manifest.title;
  renderList(manifest.source_live_pages || [], "grounding-list", "No approved live pages recorded");

  const status = document.getElementById("status-summary");
  const appendStatus = (label, value) => {
    const dt = document.createElement("dt");
    const dd = document.createElement("dd");
    dt.textContent = label;
    dd.textContent = value;
    status.appendChild(dt);
    status.appendChild(dd);
  };

  appendStatus("Mode", manifest.render_mode);
  appendStatus("Pages", String((manifest.pages || []).length));
  appendStatus("Rejected", String((manifest.rejected_source_paths || []).length));
  appendStatus("Follow-up", manifest.followup_route || "none");

  const pageList = document.getElementById("page-list");
  const buttons = [];

  const selectPage = async (page, button) => {
    for (const candidate of buttons) {
      candidate.classList.remove("active");
    }
    button.classList.add("active");
    const payload = await fetch(page.file).then((response) => response.json());
    document.getElementById("page-kind").textContent = payload.kind;
    document.getElementById("page-title").textContent = payload.title;
    document.getElementById("page-path").textContent = payload.path;
    document.getElementById("page-excerpt").textContent = payload.excerpt || "No excerpt available.";
    renderList(payload.source_live_pages || [], "page-sources", "No approved live pages recorded");
    document.getElementById("page-body").innerHTML = renderMarkdown(payload.body_markdown || "");
  };

  if (!manifest.pages.length) {
    document.getElementById("page-title").textContent = "No accepted sources";
    document.getElementById("page-path").textContent = "Only approved live pages and already-cited archived outputs can be exported.";
    document.getElementById("page-excerpt").textContent = "Add approved live source paths or archived outputs with source_live_pages metadata.";
    document.getElementById("page-body").innerHTML = "<p>This static package was generated without accepted render sources.</p>";
    return;
  }

  for (const page of manifest.pages) {
    const button = document.createElement("button");
    button.type = "button";
    button.textContent = page.title;
    button.addEventListener("click", () => selectPage(page, button));
    pageList.appendChild(button);
    buttons.push(button);
  }

  await selectPage(manifest.pages[0], buttons[0]);
});
"""


def _excerpt_from_record(record: MarkdownRecord) -> str:
    for raw_line in record.body.splitlines():
        line = raw_line.strip()
        if not line or line.startswith("#") or line.startswith("---"):
            continue
        return line[:180]
    return ""


def _page_payload(record: MarkdownRecord, index: int) -> tuple[str, dict[str, Any], dict[str, Any]]:
    title = resolve_title([record], None)
    file_slug = slugify_title(f"{index:02d}-{title}") or f"page-{index:02d}"
    rel_file = f"pages/{file_slug}.json"
    source_live_pages = source_live_pages_from_records([record])
    excerpt = _excerpt_from_record(record)
    manifest_page = {
        "title": title,
        "kind": record.kind,
        "path": record.path,
        "file": rel_file,
        "excerpt": excerpt,
        "source_live_pages": source_live_pages,
    }
    page_payload = {
        "title": title,
        "kind": record.kind,
        "path": record.path,
        "excerpt": excerpt,
        "source_live_pages": source_live_pages,
        "frontmatter": record.frontmatter,
        "body_markdown": record.body,
    }
    return rel_file, manifest_page, page_payload


def write_web_export(
    package_root: Path,
    *,
    title: str,
    sources: list[MarkdownRecord],
    source_live_pages: list[str],
    rejected_source_paths: list[str],
) -> None:
    package_root.mkdir(parents=True, exist_ok=True)
    pages_root = package_root / "pages"
    pages_root.mkdir(parents=True, exist_ok=True)

    manifest_pages: list[dict[str, Any]] = []
    for index, record in enumerate(sources, start=1):
        rel_file, manifest_page, page_payload = _page_payload(record, index)
        page_abs = package_root / rel_file
        page_abs.parent.mkdir(parents=True, exist_ok=True)
        page_abs.write_text(json.dumps(page_payload, ensure_ascii=False, indent=2), encoding="utf-8")
        manifest_pages.append(manifest_page)

    manifest = {
        "title": title,
        "render_mode": "web",
        "generated_at": now_iso(),
        "source_live_pages": source_live_pages,
        "followup_route": "none",
        "rejected_source_paths": rejected_source_paths,
        "pages": manifest_pages,
    }
    (package_root / "manifest.json").write_text(json.dumps(manifest, ensure_ascii=False, indent=2), encoding="utf-8")
    (package_root / "index.html").write_text(_shell_html(title), encoding="utf-8")
    (package_root / "styles.css").write_text(_css(), encoding="utf-8")
    (package_root / "app.js").write_text(_app_js(), encoding="utf-8")
