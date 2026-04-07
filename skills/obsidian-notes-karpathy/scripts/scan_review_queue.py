#!/usr/bin/env python3

from __future__ import annotations

import sys
from pathlib import Path

SCRIPT_DIR = Path(__file__).resolve().parent
sys.path.insert(0, str(SCRIPT_DIR))

from _vault_utils import json_dump, scan_review_queue


def main() -> None:
    if len(sys.argv) != 2:
        raise SystemExit("Usage: scan_review_queue.py <vault-root>")

    vault_root = Path(sys.argv[1]).resolve()
    print(json_dump(scan_review_queue(vault_root)))


if __name__ == "__main__":
    main()
