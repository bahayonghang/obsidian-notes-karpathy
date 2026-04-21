from __future__ import annotations


def build_report_body(title: str, source_live_pages: list[str]) -> list[str]:
    body_lines = [
        f"# {title}",
        "",
        "## Summary",
        "",
        "- This report is grounded only in approved live pages or archived outputs that cite them.",
        "",
        "## Source Live Pages",
        "",
    ]
    body_lines.extend(f"- {value}" for value in source_live_pages or ["None"])
    return body_lines
