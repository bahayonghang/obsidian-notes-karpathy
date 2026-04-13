from __future__ import annotations

import json
from datetime import UTC, datetime
from html import escape
from pathlib import Path
from typing import Any

from _vault_common import MarkdownRecord, list_field
from _vault_layout import collect_markdown_records


RENDER_MODES = ("slides", "charts", "canvas", "report", "web")
ARCHIVED_RENDER_SOURCE_KINDS = {
    "briefing",
    "qa",
    "content_output",
    "report_output",
    "slide_output",
    "chart_output",
}


def _records_by_path(vault_root: Path) -> dict[str, MarkdownRecord]:
    return {record.path: record for record in collect_markdown_records(vault_root)}


def _resolve_sources(vault_root: Path, source_paths: list[str]) -> tuple[list[MarkdownRecord], list[str]]:
    records = _records_by_path(vault_root)
    resolved: list[MarkdownRecord] = []
    rejected: list[str] = []
    for path in source_paths:
        normalized = path.replace("\\", "/")
        candidate = records.get(normalized)
        if candidate is None:
            rejected.append(normalized)
            continue
        if _is_allowed_render_source(candidate):
            resolved.append(candidate)
        else:
            rejected.append(normalized)
    return resolved, rejected


def _is_allowed_render_source(record: MarkdownRecord) -> bool:
    if record.path.startswith("wiki/live/"):
        return True
    if record.kind in ARCHIVED_RENDER_SOURCE_KINDS:
        return bool(_source_live_pages([record]))
    return False


def _source_live_pages(records: list[MarkdownRecord]) -> list[str]:
    pages: set[str] = set()
    for record in records:
        if record.path.startswith("wiki/live/"):
            pages.add(f"[[{record.path_no_ext}]]")
            continue
        for source in list_field(record.frontmatter, "source_live_pages"):
            pages.add(source)
    return sorted(pages)


def _default_output_path(mode: str, title_slug: str) -> str:
    if mode == "slides":
        return f"outputs/slides/{title_slug}.md"
    if mode == "report":
        return f"outputs/reports/{title_slug}.md"
    if mode == "charts":
        return f"outputs/charts/{title_slug}.md"
    if mode == "web":
        return f"outputs/web/{title_slug}/index.html"
    return f"outputs/charts/{title_slug}.canvas"


def _title(records: list[MarkdownRecord], explicit_title: str | None) -> str:
    if explicit_title and explicit_title.strip():
        return explicit_title.strip()
    if records:
        raw_title = records[0].frontmatter.get("title")
        if isinstance(raw_title, str) and raw_title.strip():
            return raw_title.strip()
        return records[0].basename.replace("-", " ").title()
    return "Rendered Artifact"


def _slugify_title(title: str) -> str:
    return "-".join(part for part in title.lower().replace("/", " ").replace("_", " ").split() if part)


def _write_markdown(path: Path, lines: list[str]) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text("\n".join(lines) + "\n", encoding="utf-8")


def _now_iso() -> str:
    return datetime.now(UTC).replace(microsecond=0).isoformat().replace("+00:00", "Z")


def _web_shell_html(title: str) -> str:
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


def _web_css() -> str:
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


def _web_app_js() -> str:
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


def _web_page_payload(record: MarkdownRecord, index: int) -> tuple[str, dict[str, Any], dict[str, Any]]:
    title = _title([record], None)
    file_slug = _slugify_title(f"{index:02d}-{title}") or f"page-{index:02d}"
    rel_file = f"pages/{file_slug}.json"
    source_live_pages = _source_live_pages([record])
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


def _write_web_export(
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
        rel_file, manifest_page, page_payload = _web_page_payload(record, index)
        page_abs = package_root / rel_file
        page_abs.parent.mkdir(parents=True, exist_ok=True)
        page_abs.write_text(json.dumps(page_payload, ensure_ascii=False, indent=2), encoding="utf-8")
        manifest_pages.append(manifest_page)

    manifest = {
        "title": title,
        "render_mode": "web",
        "generated_at": _now_iso(),
        "source_live_pages": source_live_pages,
        "followup_route": "none",
        "rejected_source_paths": rejected_source_paths,
        "pages": manifest_pages,
    }
    (package_root / "manifest.json").write_text(json.dumps(manifest, ensure_ascii=False, indent=2), encoding="utf-8")
    (package_root / "index.html").write_text(_web_shell_html(title), encoding="utf-8")
    (package_root / "styles.css").write_text(_web_css(), encoding="utf-8")
    (package_root / "app.js").write_text(_web_app_js(), encoding="utf-8")


def _canvas_payload(title: str, source_live_pages: list[str]) -> dict[str, Any]:
    return {
        "nodes": [
            {
                "id": "title",
                "type": "text",
                "x": 80,
                "y": 80,
                "width": 320,
                "height": 80,
                "text": title,
            },
            {
                "id": "sources",
                "type": "text",
                "x": 80,
                "y": 200,
                "width": 520,
                "height": 220,
                "text": "\n".join(source_live_pages or ["No approved live pages supplied"]),
            },
        ],
        "edges": [],
    }


def render_artifact(
    vault_root: Path,
    *,
    mode: str,
    source_paths: list[str],
    output_path: str | None = None,
    title: str | None = None,
    write: bool = False,
) -> dict[str, Any]:
    if mode not in RENDER_MODES:
        raise ValueError(f"Unsupported render mode: {mode}")

    sources, rejected_source_paths = _resolve_sources(vault_root, source_paths)
    resolved_title = _title(sources, title)
    title_slug = _slugify_title(resolved_title) or "rendered-artifact"
    rel_output_path = output_path or _default_output_path(mode, title_slug)
    source_live_pages = _source_live_pages(sources)
    payload = {
        "vault_root": str(vault_root),
        "mode": mode,
        "requested_source_paths": [path.replace("\\", "/") for path in source_paths],
        "source_paths": [record.path for record in sources],
        "rejected_source_paths": rejected_source_paths,
        "source_live_pages": source_live_pages,
        "output_path": rel_output_path,
        "title": resolved_title,
        "followup_route": "none",
    }

    if not write:
        return payload

    output_abs = vault_root / rel_output_path
    if mode == "web":
        package_root = output_abs.parent
        payload["package_root"] = package_root.relative_to(vault_root).as_posix()
        _write_web_export(
            package_root,
            title=resolved_title,
            sources=sources,
            source_live_pages=source_live_pages,
            rejected_source_paths=rejected_source_paths,
        )
        return payload

    if mode == "canvas":
        output_abs.parent.mkdir(parents=True, exist_ok=True)
        output_abs.write_text(json.dumps(_canvas_payload(resolved_title, source_live_pages), ensure_ascii=False, indent=2), encoding="utf-8")
        return payload

    frontmatter_lines = [
        "---",
        f'title: "{resolved_title}"',
        f'render_mode: "{mode}"',
        "source_live_pages:",
    ]
    frontmatter_lines.extend(f'  - "{value}"' for value in source_live_pages or [""])
    frontmatter_lines.extend(
        [
            'followup_route: "none"',
            "---",
            "",
        ]
    )

    body_lines: list[str]
    if mode == "slides":
        body_lines = [
            "marp: true",
            "theme: default",
            "paginate: true",
            "---",
            f"# {resolved_title}",
            "",
            "## Grounding",
            "",
        ]
        body_lines.extend(f"- {value}" for value in source_live_pages or ["None"])
    elif mode == "charts":
        body_lines = [
            f"# {resolved_title}",
            "",
            "## Chart Intent",
            "",
            "- This file is a deterministic chart brief derived from approved knowledge.",
            "",
            "## Source Live Pages",
            "",
        ]
        body_lines.extend(f"- {value}" for value in source_live_pages or ["None"])
    else:
        body_lines = [
            f"# {resolved_title}",
            "",
            "## Summary",
            "",
            "- This report is grounded only in approved live pages or archived outputs that cite them.",
            "",
            "## Source Live Pages",
            "",
        ]
        body_lines.extend(f"- {value}" for value in source_live_pages or ["None"])

    _write_markdown(output_abs, [*frontmatter_lines, *body_lines])
    return payload
