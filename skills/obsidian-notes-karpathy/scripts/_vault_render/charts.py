from __future__ import annotations


def build_charts_body(title: str, source_live_pages: list[str]) -> list[str]:
    body_lines = [
        f"# {title}",
        "",
        "## Chart Intent",
        "",
        "- This file is a deterministic chart brief derived from approved knowledge.",
        "",
        "## Source Live Pages",
        "",
    ]
    body_lines.extend(f"- {value}" for value in source_live_pages or ["None"])
    return body_lines
