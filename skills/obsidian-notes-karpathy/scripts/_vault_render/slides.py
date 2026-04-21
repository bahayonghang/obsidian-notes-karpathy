from __future__ import annotations


def build_slides_body(title: str, source_live_pages: list[str]) -> list[str]:
    body_lines = [
        "marp: true",
        "theme: default",
        "paginate: true",
        "---",
        f"# {title}",
        "",
        "## Grounding",
        "",
    ]
    body_lines.extend(f"- {value}" for value in source_live_pages or ["None"])
    return body_lines
