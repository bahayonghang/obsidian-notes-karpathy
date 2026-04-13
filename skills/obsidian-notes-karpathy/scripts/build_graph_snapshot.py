#!/usr/bin/env python3

from __future__ import annotations

import argparse
import json
from datetime import UTC, datetime
from pathlib import Path

from _vault_utils import collect_markdown_records, json_dump, list_field, live_records


def _append_audit_event(vault_root: Path, action: str, payload: dict[str, object]) -> None:
    audit_path = vault_root / "outputs" / "audit" / "operations.jsonl"
    audit_path.parent.mkdir(parents=True, exist_ok=True)
    entry = {
        "timestamp": datetime.now(UTC).replace(microsecond=0).isoformat().replace("+00:00", "Z"),
        "action": action,
        "payload": payload,
    }
    with audit_path.open("a", encoding="utf-8") as handle:
        handle.write(json.dumps(entry, ensure_ascii=False) + "\n")


def build_graph_snapshot(vault_root: Path) -> dict[str, object]:
    records = [
        record
        for record in live_records(collect_markdown_records(vault_root))
        if record.kind in {"summary", "concept", "entity", "procedure", "topic"}
    ]
    nodes: list[dict[str, object]] = []
    edges: list[dict[str, object]] = []
    edge_type_counts: dict[str, int] = {}
    generated_at = datetime.now(UTC).replace(microsecond=0).isoformat().replace("+00:00", "Z")
    for record in records:
        related = list_field(record.frontmatter, "related")
        question_links = list_field(record.frontmatter, "question_links")
        relationship_notes = list_field(record.frontmatter, "relationship_notes")
        sources = list_field(record.frontmatter, "sources")
        nodes.append(
            {
                "id": record.path[:-3],
                "path": record.path,
                "kind": record.kind,
                "title": record.frontmatter.get("title") or record.basename,
                "visibility_scope": record.frontmatter.get("visibility_scope") or "shared",
                "topic_hub": record.frontmatter.get("topic_hub"),
                "source_count": len(sources),
                "related_count": len(related),
                "question_link_count": len(question_links),
                "has_relationship_notes": bool(relationship_notes),
                "graph_required": str(record.frontmatter.get("graph_required") or "").strip().lower() in {"true", "yes", "1"},
                "confidence_score": record.frontmatter.get("confidence_score"),
                "confidence_band": record.frontmatter.get("confidence_band"),
                "approved_at": record.frontmatter.get("approved_at"),
                "last_confirmed_at": record.frontmatter.get("last_confirmed_at"),
            }
        )
        for relation in ("related", "supersedes", "superseded_by"):
            targets = list_field(record.frontmatter, relation)
            edge_type_counts[relation] = edge_type_counts.get(relation, 0) + len(targets)
            for target in targets:
                edges.append(
                    {
                        "source": record.path[:-3],
                        "relation": relation,
                        "target": target,
                        "source_kind": record.kind,
                        "source_visibility_scope": record.frontmatter.get("visibility_scope") or "shared",
                    }
                )

    return {
        "vault_root": str(vault_root),
        "generated_at": generated_at,
        "candidate_policy": "candidate-only",
        "node_count": len(nodes),
        "edge_count": len(edges),
        "edge_type_counts": dict(sorted(edge_type_counts.items())),
        "nodes": nodes,
        "edges": edges,
    }


def write_graph_snapshot(vault_root: Path, payload: dict[str, object]) -> Path:
    output_path = vault_root / "outputs" / "health" / "graph-snapshot.json"
    output_path.parent.mkdir(parents=True, exist_ok=True)
    output_path.write_text(json_dump(payload), encoding="utf-8")
    _append_audit_event(
        vault_root,
        "build_graph_snapshot",
        {"output_path": output_path.relative_to(vault_root).as_posix(), "node_count": payload["node_count"], "edge_count": payload["edge_count"]},
    )
    return output_path


def main() -> None:
    parser = argparse.ArgumentParser(description="Build a local JSON graph snapshot from approved live pages.")
    parser.add_argument("vault_root")
    parser.add_argument("--write", action="store_true", help="Write outputs/health/graph-snapshot.json")
    args = parser.parse_args()

    vault_root = Path(args.vault_root).resolve()
    payload = build_graph_snapshot(vault_root)
    if args.write:
        output_path = write_graph_snapshot(vault_root, payload)
        payload["output_path"] = output_path.relative_to(vault_root).as_posix()
    print(json_dump(payload))


if __name__ == "__main__":
    main()
