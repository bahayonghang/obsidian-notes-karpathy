#!/usr/bin/env python3

from __future__ import annotations

import argparse
import json
from datetime import UTC, datetime
from pathlib import Path
import sys

SCRIPT_DIR = Path(__file__).resolve().parent
sys.path.insert(0, str(SCRIPT_DIR))

from _vault_utils import audit_vault_mechanics, json_dump, scan_compile_delta
from build_governance_indices import build_governance_indices, write_governance_indices
from build_graph_snapshot import build_graph_snapshot, write_graph_snapshot
from build_memory_episodes import build_memory_episodes, write_memory_episodes


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


def run_automation(vault_root: Path, mode: str, write: bool) -> dict[str, object]:
    payload: dict[str, object] = {
        "vault_root": str(vault_root),
        "mode": mode,
        "write": write,
    }

    if mode == "scheduled-health":
        lint_payload = audit_vault_mechanics(vault_root)
        governance_payload = build_governance_indices(vault_root)
        graph_payload = build_graph_snapshot(vault_root)
        payload.update(
            {
                "lint": lint_payload,
                "governance": governance_payload,
                "graph": graph_payload,
            }
        )
        if write:
            write_governance_indices(vault_root, governance_payload)
            graph_output = write_graph_snapshot(vault_root, graph_payload)
            payload["graph_output_path"] = graph_output.relative_to(vault_root).as_posix()
    elif mode == "session-end":
        episode_payload = build_memory_episodes(vault_root)
        payload["episodes"] = episode_payload
        if write:
            payload["written_paths"] = write_memory_episodes(vault_root, episode_payload)
    elif mode == "query-archive":
        governance_payload = build_governance_indices(vault_root)
        episode_payload = build_memory_episodes(vault_root)
        payload.update(
            {
                "governance": governance_payload,
                "episodes": episode_payload,
            }
        )
        if write:
            write_governance_indices(vault_root, governance_payload)
            payload["written_paths"] = write_memory_episodes(vault_root, episode_payload)
    elif mode == "new-source":
        compile_payload = scan_compile_delta(vault_root)
        graph_payload = build_graph_snapshot(vault_root)
        payload.update(
            {
                "compile_delta": compile_payload,
                "graph": graph_payload,
            }
        )
        if write:
            graph_output = write_graph_snapshot(vault_root, graph_payload)
            payload["graph_output_path"] = graph_output.relative_to(vault_root).as_posix()
    else:
        raise ValueError(f"Unsupported mode: {mode}")

    if write:
        _append_audit_event(
            vault_root,
            "run_automation",
            {
                "mode": mode,
                "steps": sorted(key for key in payload.keys() if key not in {"vault_root", "mode", "write"}),
            },
        )
    return payload


def main() -> None:
    parser = argparse.ArgumentParser(description="Run deterministic automation workflows for the review-gated vault.")
    parser.add_argument("vault_root")
    parser.add_argument("--mode", required=True, choices=("new-source", "query-archive", "session-end", "scheduled-health"))
    parser.add_argument("--write", action="store_true", help="Write the automation outputs and append audit breadcrumbs")
    args = parser.parse_args()

    vault_root = Path(args.vault_root).resolve()
    payload = run_automation(vault_root, args.mode, args.write)
    print(json_dump(payload))


if __name__ == "__main__":
    main()
