from __future__ import annotations

import json
from pathlib import Path
from typing import Any

from .canvas import canvas_payload
from .charts import build_charts_body
from .common import (
    RENDER_MODES,
    default_output_path,
    resolve_sources,
    resolve_title,
    slugify_title,
    source_live_pages_from_records,
    write_markdown,
)
from .report import build_report_body
from .slides import build_slides_body
from .web import write_web_export


__all__ = ["render_artifact", "RENDER_MODES"]


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

    sources, rejected_source_paths = resolve_sources(vault_root, source_paths)
    resolved_title = resolve_title(sources, title)
    title_slug = slugify_title(resolved_title) or "rendered-artifact"
    rel_output_path = output_path or default_output_path(mode, title_slug)
    source_live_pages = source_live_pages_from_records(sources)
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
        write_web_export(
            package_root,
            title=resolved_title,
            sources=sources,
            source_live_pages=source_live_pages,
            rejected_source_paths=rejected_source_paths,
        )
        return payload

    if mode == "canvas":
        output_abs.parent.mkdir(parents=True, exist_ok=True)
        output_abs.write_text(
            json.dumps(canvas_payload(resolved_title, source_live_pages), ensure_ascii=False, indent=2),
            encoding="utf-8",
        )
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

    if mode == "slides":
        body_lines = build_slides_body(resolved_title, source_live_pages)
    elif mode == "charts":
        body_lines = build_charts_body(resolved_title, source_live_pages)
    else:
        body_lines = build_report_body(resolved_title, source_live_pages)

    write_markdown(output_abs, [*frontmatter_lines, *body_lines])
    return payload
