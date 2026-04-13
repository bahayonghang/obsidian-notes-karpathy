#!/usr/bin/env python3

from __future__ import annotations

import sys
from pathlib import Path

SCRIPT_DIR = Path(__file__).resolve().parent
sys.path.insert(0, str(SCRIPT_DIR))

from _vault_ingest import sync_source_manifest
from _vault_utils import json_dump


def main() -> None:
    if len(sys.argv) != 2:
        raise SystemExit("Usage: sync_source_manifest.py <vault-root>")

    vault_root = Path(sys.argv[1]).resolve()
    print(json_dump(sync_source_manifest(vault_root)))


if __name__ == "__main__":
    main()
