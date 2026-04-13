#!/usr/bin/env python3

from __future__ import annotations

import argparse
import re
import sys
from pathlib import Path

SCRIPT_DIR = Path(__file__).resolve().parent
sys.path.insert(0, str(SCRIPT_DIR))

from _vault_utils import collect_markdown_records, json_dump, query_scope


TOKEN_RE = re.compile(r"[A-Za-z0-9_-]+")


def _tokenize(text: str) -> set[str]:
    return {token.lower() for token in TOKEN_RE.findall(text)}


def rank_query_candidates(vault_root: Path, query: str) -> dict[str, object]:
    scope = query_scope(vault_root)
    records = {record.path: record for record in collect_markdown_records(vault_root)}
    query_tokens = _tokenize(query)
    ranked: list[dict[str, object]] = []

    for path in [*scope["included_paths"], *scope["candidate_paths"]]:
        candidate_kind = "included" if path in scope["included_paths"] else "candidate"
        truth_boundary = "approved-live" if candidate_kind == "included" else scope["candidate_policy"]

        if path.endswith("graph-snapshot.json"):
            graph_path = vault_root / path
            title = graph_path.name
            graph_text = graph_path.read_text(encoding="utf-8") if graph_path.exists() else ""
            lexical_overlap = len(query_tokens & _tokenize(title + " " + graph_text))
            metadata_score = 1
            graph_score = 2
            score = lexical_overlap * 10 + metadata_score * 2 + graph_score
            ranked.append(
                {
                    "path": path,
                    "kind": "graph_snapshot",
                    "candidate_kind": candidate_kind,
                    "truth_boundary": truth_boundary,
                    "score": score,
                    "lexical_overlap": lexical_overlap,
                    "metadata_score": metadata_score,
                    "graph_score": graph_score,
                    "title": title,
                }
            )
            continue

        record = records.get(path)
        if record is None:
            continue

        title = str(record.frontmatter.get("title") or record.basename)
        text_tokens = _tokenize(title)
        text_tokens.update(_tokenize(record.body))
        lexical_overlap = len(query_tokens & text_tokens)

        metadata_score = 0
        if record.frontmatter.get("topic_hub"):
            metadata_score += 1
        if record.frontmatter.get("visibility_scope") == "shared":
            metadata_score += 1
        if record.kind in {"concept", "entity", "procedure"}:
            metadata_score += 1
        if record.kind == "briefing":
            metadata_score += 1
        if record.kind == "qa":
            metadata_score += 1

        graph_score = 0
        related = record.frontmatter.get("related")
        if isinstance(related, list):
            graph_score += len([item for item in related if isinstance(item, str) and item.strip()])
        elif isinstance(related, str) and related.strip():
            graph_score += 1
        if record.frontmatter.get("topic_hub"):
            graph_score += 1
        if record.frontmatter.get("question_links"):
            graph_score += 1
        if record.frontmatter.get("crystallized_from_episode"):
            graph_score += 1

        score = lexical_overlap * 10 + metadata_score * 2 + graph_score
        ranked.append(
            {
                "path": path,
                "kind": record.kind,
                "candidate_kind": candidate_kind,
                "truth_boundary": truth_boundary,
                "score": score,
                "lexical_overlap": lexical_overlap,
                "metadata_score": metadata_score,
                "graph_score": graph_score,
                "title": title,
            }
        )

    def _sort_key(item: dict[str, object]) -> tuple[float, bool, str]:
        raw_score = item.get("score")
        score = float(raw_score) if isinstance(raw_score, int | float) else 0.0
        candidate_kind = item.get("candidate_kind")
        path = item.get("path")
        return (-score, candidate_kind != "included", str(path))

    ranked.sort(key=_sort_key)
    return {
        "vault_root": str(vault_root),
        "query": query,
        "candidate_policy": scope["candidate_policy"],
        "included_count": len(scope["included_paths"]),
        "candidate_count": len(scope["candidate_paths"]),
        "ranked_paths": ranked,
    }


def main() -> None:
    parser = argparse.ArgumentParser(description="Rank local query candidates without widening the approved truth boundary.")
    parser.add_argument("vault_root")
    parser.add_argument("query")
    args = parser.parse_args()

    vault_root = Path(args.vault_root).resolve()
    print(json_dump(rank_query_candidates(vault_root, args.query)))


if __name__ == "__main__":
    main()
