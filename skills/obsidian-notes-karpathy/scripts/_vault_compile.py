from __future__ import annotations

from collections import Counter
from pathlib import Path
from typing import Any

from _vault_common import load_markdown, parse_datetime
from _vault_layout import (
    accepted_raw_sources,
    compute_hash,
    detect_companion_skills,
    detect_layout_family,
    iso_from_timestamp,
    pdf_ingest_plan,
    raw_source_metadata,
    source_class_for_raw,
    summary_for_raw,
)


def scan_compile_delta(vault_root: Path) -> dict[str, Any]:
    items: list[dict[str, Any]] = []
    counts = Counter()
    ingest_counts = Counter()
    companion_skills = detect_companion_skills()
    companion_status = companion_skills["skills"]
    layout_family = detect_layout_family(vault_root)

    for raw_path in accepted_raw_sources(vault_root):
        rel_path = raw_path.relative_to(vault_root).as_posix()
        summary_path = summary_for_raw(vault_root, raw_path)
        raw_hash = compute_hash(raw_path)
        raw_mtime = iso_from_timestamp(raw_path.stat().st_mtime)
        item = {
            "path": rel_path,
            "summary_path": summary_path.relative_to(vault_root).as_posix(),
            "raw_hash": raw_hash,
            "raw_mtime": raw_mtime,
            "source_class": source_class_for_raw(raw_path),
            "layout_family": layout_family,
        }
        item.update(raw_source_metadata(vault_root, raw_path))

        if item["source_class"] == "paper_pdf":
            item.update(pdf_ingest_plan(vault_root, raw_path, companion_status))
        else:
            item["ingest_plan"] = "markdown"
            item["ingest_reason"] = "markdown_source"

        ingest_counts[item["ingest_plan"]] += 1
        if item["ingest_plan"] == "skip":
            counts["skipped"] += 1

        if not summary_path.exists():
            item["status"] = "new"
            item["reason"] = "missing_summary"
            counts["new"] += 1
            items.append(item)
            continue

        summary_record = load_markdown(summary_path, vault_root)
        summary_fm = summary_record.frontmatter
        summary_hash = summary_fm.get("source_hash")
        summary_mtime = parse_datetime(summary_fm.get("source_mtime"))
        raw_mtime_dt = parse_datetime(raw_mtime)

        if summary_hash:
            item["recorded_source_hash"] = summary_hash
            if summary_hash == raw_hash:
                item["status"] = "unchanged"
                item["reason"] = "source_hash_match"
                counts["unchanged"] += 1
            else:
                item["status"] = "changed"
                item["reason"] = "source_hash_mismatch"
                counts["changed"] += 1
            items.append(item)
            continue

        if summary_mtime and raw_mtime_dt and raw_mtime_dt > summary_mtime:
            item["status"] = "changed"
            item["reason"] = "source_mtime_outdated"
            item["recorded_source_mtime"] = summary_fm.get("source_mtime")
            counts["changed"] += 1
        else:
            item["status"] = "unchanged"
            item["reason"] = "source_mtime_current"
            item["recorded_source_mtime"] = summary_fm.get("source_mtime")
            counts["unchanged"] += 1

        items.append(item)

    payload_counts = {
        "new": counts["new"],
        "changed": counts["changed"],
        "unchanged": counts["unchanged"],
        "skipped": counts["skipped"],
    }

    return {
        "vault_root": str(vault_root),
        "layout_family": layout_family,
        "counts": payload_counts,
        "ingest_counts": dict(sorted(ingest_counts.items())),
        "companion_skills": companion_skills,
        "items": items,
    }
