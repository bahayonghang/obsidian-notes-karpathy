use std::fs;
use std::path::Path;

use anyhow::Result;
use serde_json::{Value, json};

use crate::common::{MarkdownRecord, json_string, list_field, now_iso, write_markdown};
use crate::layout::collect_markdown_records;
use crate::payload::RenderResult;

pub const RENDER_MODES: [&str; 5] = ["slides", "charts", "canvas", "report", "web"];

pub fn render_artifact(
    vault_root: &Path,
    mode: &str,
    source_paths: &[String],
    output_path: Option<&str>,
    title: Option<&str>,
    write: bool,
) -> Result<Value> {
    if !RENDER_MODES.contains(&mode) {
        anyhow::bail!("Unsupported render mode: {mode}");
    }

    let (sources, rejected_source_paths) = resolve_sources(vault_root, source_paths)?;
    let resolved_title = resolve_title(&sources, title);
    let title_slug = slugify_title(&resolved_title);
    let rel_output_path = output_path
        .map(|value| value.to_string())
        .unwrap_or_else(|| default_output_path(mode, &title_slug));
    let source_live_pages = source_live_pages_from_records(&sources);
    let mut result = RenderResult {
        vault_root: crate::common::normalize_path_string(vault_root.to_string_lossy().as_ref()),
        mode: mode.to_string(),
        requested_source_paths: source_paths.to_vec(),
        source_paths: sources.iter().map(|record| record.path.clone()).collect(),
        rejected_source_paths,
        source_live_pages: source_live_pages.clone(),
        output_path: rel_output_path.clone(),
        title: resolved_title.clone(),
        followup_route: "none".to_string(),
        package_root: None,
    };

    if !write {
        return Ok(serde_json::to_value(&result)?);
    }

    let output_abs = vault_root.join(&rel_output_path);
    if mode == "web" {
        let package_root = output_abs.parent().unwrap_or(&output_abs);
        write_web_export(
            package_root,
            &resolved_title,
            &sources,
            &source_live_pages,
            &result.rejected_source_paths,
        )?;
        result.package_root = Some(crate::common::relative_posix(package_root, vault_root));
        return Ok(serde_json::to_value(&result)?);
    }

    if mode == "canvas" {
        if let Some(parent) = output_abs.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(
            &output_abs,
            serde_json::to_string_pretty(&canvas_payload(&resolved_title, &source_live_pages))?,
        )?;
        return Ok(serde_json::to_value(&result)?);
    }

    let mut frontmatter_lines = vec![
        "---".to_string(),
        format!("title: {}", json_string(&resolved_title)),
        format!("render_mode: {}", json_string(mode)),
        "source_live_pages:".to_string(),
    ];
    if source_live_pages.is_empty() {
        frontmatter_lines.push(r#"  - ""#.to_string());
    } else {
        for value in &source_live_pages {
            frontmatter_lines.push(format!("  - {}", json_string(value)));
        }
    }
    frontmatter_lines.push(r#"followup_route: "none""#.to_string());
    frontmatter_lines.push("---".to_string());
    frontmatter_lines.push(String::new());

    let body_lines = match mode {
        "slides" => build_slides_body(&resolved_title, &source_live_pages),
        "charts" => build_charts_body(&resolved_title, &source_live_pages),
        _ => build_report_body(&resolved_title, &source_live_pages),
    };
    let mut lines = frontmatter_lines;
    lines.extend(body_lines);
    write_markdown(&output_abs, &lines)?;
    Ok(serde_json::to_value(&result)?)
}

fn records_by_path(vault_root: &Path) -> Result<std::collections::HashMap<String, MarkdownRecord>> {
    Ok(collect_markdown_records(vault_root)?
        .into_iter()
        .map(|record| (record.path.clone(), (*record).clone()))
        .collect())
}

fn resolve_sources(
    vault_root: &Path,
    source_paths: &[String],
) -> Result<(Vec<MarkdownRecord>, Vec<String>)> {
    let records = records_by_path(vault_root)?;
    let mut resolved = Vec::new();
    let mut rejected = Vec::new();
    for path in source_paths {
        let normalized = path.replace('\\', "/");
        let Some(candidate) = records.get(&normalized).cloned() else {
            rejected.push(normalized);
            continue;
        };
        if is_allowed_render_source(&candidate) {
            resolved.push(candidate);
        } else {
            rejected.push(normalized);
        }
    }
    Ok((resolved, rejected))
}

fn is_allowed_render_source(record: &MarkdownRecord) -> bool {
    if record.path.starts_with("wiki/live/") {
        return true;
    }
    let archived_kinds = [
        "briefing",
        "qa",
        "content_output",
        "report_output",
        "slide_output",
        "chart_output",
    ];
    archived_kinds.contains(&record.kind.as_str())
        && !source_live_pages_from_records(std::slice::from_ref(record)).is_empty()
}

fn source_live_pages_from_records(records: &[MarkdownRecord]) -> Vec<String> {
    let mut pages = std::collections::BTreeSet::new();
    for record in records {
        if record.path.starts_with("wiki/live/") {
            pages.insert(format!("[[{}]]", record.path_no_ext()));
            continue;
        }
        for source in list_field(&record.frontmatter, "source_live_pages") {
            pages.insert(source);
        }
    }
    pages.into_iter().collect()
}

fn default_output_path(mode: &str, title_slug: &str) -> String {
    match mode {
        "slides" => format!("outputs/slides/{title_slug}.md"),
        "report" => format!("outputs/reports/{title_slug}.md"),
        "charts" => format!("outputs/charts/{title_slug}.md"),
        "web" => format!("outputs/web/{title_slug}/index.html"),
        _ => format!("outputs/charts/{title_slug}.canvas"),
    }
}

fn resolve_title(records: &[MarkdownRecord], explicit_title: Option<&str>) -> String {
    if let Some(title) = explicit_title
        && !title.trim().is_empty()
    {
        return title.trim().to_string();
    }
    if let Some(record) = records.first() {
        if let Some(title) = record.frontmatter.get("title").and_then(Value::as_str)
            && !title.trim().is_empty()
        {
            return title.trim().to_string();
        }
        return record.basename().replace('-', " ").to_string();
    }
    "Rendered Artifact".to_string()
}

pub fn slugify_title(title: &str) -> String {
    title
        .to_lowercase()
        .replace(['/', '_'], " ")
        .split_whitespace()
        .collect::<Vec<_>>()
        .join("-")
}

fn build_slides_body(title: &str, source_live_pages: &[String]) -> Vec<String> {
    vec![
        format!("# {title}"),
        String::new(),
        "## Grounding".to_string(),
        String::new(),
        format!("- source_live_pages: {}", source_live_pages.join(", ")),
    ]
}

fn build_charts_body(title: &str, source_live_pages: &[String]) -> Vec<String> {
    vec![
        format!("# {title}"),
        String::new(),
        "## Chart Brief".to_string(),
        String::new(),
        format!("- source_live_pages: {}", source_live_pages.join(", ")),
    ]
}

fn build_report_body(title: &str, source_live_pages: &[String]) -> Vec<String> {
    vec![
        format!("# {title}"),
        String::new(),
        "## Summary".to_string(),
        String::new(),
        format!("- followup_route: none"),
        format!("- source_live_pages: {}", source_live_pages.join(", ")),
    ]
}

fn canvas_payload(title: &str, source_live_pages: &[String]) -> Value {
    json!({
        "type": "canvas",
        "title": title,
        "source_live_pages": source_live_pages,
    })
}

fn write_web_export(
    package_root: &Path,
    title: &str,
    sources: &[MarkdownRecord],
    source_live_pages: &[String],
    rejected_source_paths: &[String],
) -> Result<()> {
    fs::create_dir_all(package_root.join("pages"))?;
    let mut manifest_pages = Vec::new();
    for (index, record) in sources.iter().enumerate() {
        let title_value = resolve_title(std::slice::from_ref(record), None);
        let file_slug = slugify_title(&format!("{:02}-{title_value}", index + 1));
        let rel_file = format!("pages/{file_slug}.json");
        let excerpt = excerpt_from_record(record);
        let page_payload = json!({
            "title": title_value,
            "kind": record.kind,
            "path": record.path,
            "excerpt": excerpt,
            "source_live_pages": source_live_pages_from_records(std::slice::from_ref(record)),
            "frontmatter": record.frontmatter,
            "body_markdown": record.body,
        });
        fs::write(
            package_root.join(&rel_file),
            serde_json::to_string_pretty(&page_payload)?,
        )?;
        manifest_pages.push(json!({
            "title": resolve_title(std::slice::from_ref(record), None),
            "kind": record.kind,
            "path": record.path,
            "file": rel_file,
            "excerpt": excerpt,
            "source_live_pages": source_live_pages_from_records(std::slice::from_ref(record)),
        }));
    }

    let manifest = json!({
        "title": title,
        "render_mode": "web",
        "generated_at": now_iso(),
        "source_live_pages": source_live_pages,
        "followup_route": "none",
        "rejected_source_paths": rejected_source_paths,
        "pages": manifest_pages,
    });
    fs::write(
        package_root.join("manifest.json"),
        serde_json::to_string_pretty(&manifest)?,
    )?;
    fs::write(package_root.join("index.html"), shell_html(title))?;
    fs::write(package_root.join("styles.css"), css())?;
    fs::write(package_root.join("app.js"), app_js())?;
    Ok(())
}

fn excerpt_from_record(record: &MarkdownRecord) -> String {
    for raw_line in record.body.lines() {
        let line = raw_line.trim();
        if !line.is_empty() && !line.starts_with('#') && !line.starts_with("---") {
            return line.chars().take(180).collect();
        }
    }
    String::new()
}

fn shell_html(title: &str) -> String {
    format!(
        r#"<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <meta name="viewport" content="width=device-width, initial-scale=1">
  <title>{}</title>
  <link rel="stylesheet" href="./styles.css">
</head>
<body>
  <div class="app-shell">
    <aside class="sidebar">
      <div class="sidebar-header">
        <p class="eyebrow">KB Query Web Export</p>
        <h1 id="site-title">{}</h1>
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
"#,
        title, title
    )
}

fn css() -> &'static str {
    r#"* { box-sizing: border-box; }
body { margin: 0; font-family: Inter, "Segoe UI", sans-serif; background: #0b1020; color: #e5e7eb; }
.app-shell { min-height: 100vh; display: grid; grid-template-columns: 320px 1fr; }
.sidebar { border-right: 1px solid #1f2937; background: #111827; padding: 24px 20px; }
.sidebar-header h1, .content-header h2, .card h3, .sidebar-section h2 { margin: 0; }
.sidebar-section { margin-top: 28px; }
.sidebar-section h2, .card h3 { margin-bottom: 12px; font-size: 0.95rem; color: #93c5fd; }
.eyebrow, .page-kind { text-transform: uppercase; letter-spacing: 0.08em; font-size: 0.72rem; color: #60a5fa; }
.page-list { display: flex; flex-direction: column; gap: 8px; }
.page-list button { text-align: left; border: 1px solid #1f2937; background: #0f172a; color: #e5e7eb; padding: 10px 12px; border-radius: 10px; cursor: pointer; }
.page-list button.active { border-color: #60a5fa; background: #172554; }
.grounding-list, .source-list { margin: 0; padding-left: 18px; }
.status-summary { display: grid; grid-template-columns: auto 1fr; gap: 8px 12px; margin: 0; }
.status-summary dt { color: #93c5fd; }
.status-summary dd { margin: 0; }
.content { padding: 28px; display: grid; gap: 18px; }
.content-header { display: grid; gap: 8px; }
.page-path { margin: 0; color: #9ca3af; font-family: "Cascadia Code", monospace; font-size: 0.85rem; }
.card, .article-body { border: 1px solid #1f2937; background: #111827; border-radius: 16px; padding: 20px; }
.article-body { line-height: 1.7; }
.article-body h1, .article-body h2, .article-body h3 { color: #f8fafc; }
.article-body p, .article-body ul { margin: 0 0 14px; }
.article-body ul { padding-left: 20px; }
.page-excerpt { margin: 0; color: #cbd5e1; }
@media (max-width: 900px) {
  .app-shell { grid-template-columns: 1fr; }
  .sidebar { border-right: 0; border-bottom: 1px solid #1f2937; }
}"#
}

fn app_js() -> &'static str {
    r#"const manifestPromise = fetch("./manifest.json").then((response) => response.json());
manifestPromise.then((manifest) => {
  document.getElementById("site-title").textContent = manifest.title;
  document.getElementById("page-title").textContent = manifest.pages.length ? manifest.pages[0].title : "No accepted sources";
  document.getElementById("page-path").textContent = manifest.pages.length ? manifest.pages[0].path : "Only approved live pages and already-cited archived outputs can be exported.";
  document.getElementById("page-excerpt").textContent = manifest.pages.length ? manifest.pages[0].excerpt : "Add approved live source paths or archived outputs with source_live_pages metadata.";
});"#
}
