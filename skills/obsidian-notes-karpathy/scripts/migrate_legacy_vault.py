#!/usr/bin/env python3

from __future__ import annotations

import argparse
import sys
from pathlib import Path

SCRIPT_DIR = Path(__file__).resolve().parent
sys.path.insert(0, str(SCRIPT_DIR))

from _vault_init import migrate_legacy_vault
from _vault_layout import DEFAULT_KB_PROFILE
from _vault_utils import json_dump


def main() -> None:
    parser = argparse.ArgumentParser(description="Migrate a legacy single-layer vault into the review-gated layout.")
    parser.add_argument("vault_root", help="Path to the target vault root.")
    parser.add_argument("--profile", default=DEFAULT_KB_PROFILE, help="Vault operating profile.")
    parser.add_argument("--no-governance", action="store_true", help="Skip governance starter indices.")
    parser.add_argument("--include-full-outputs", action="store_true", help="Create downstream output directories.")
    parser.add_argument("--include-latest-outputs", action="store_true", help="Create episodic and audit scaffolding.")
    args = parser.parse_args()

    payload = migrate_legacy_vault(
        Path(args.vault_root).resolve(),
        profile=args.profile,
        include_governance=not args.no_governance,
        include_full_outputs=args.include_full_outputs,
        include_latest_outputs=args.include_latest_outputs,
    )
    print(json_dump(payload))


if __name__ == "__main__":
    main()
