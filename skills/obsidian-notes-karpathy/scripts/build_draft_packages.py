#!/usr/bin/env python3

from __future__ import annotations

import argparse
import sys
from pathlib import Path

SCRIPT_DIR = Path(__file__).resolve().parent
sys.path.insert(0, str(SCRIPT_DIR))

from _vault_utils import build_draft_packages, json_dump


def main() -> None:
    parser = argparse.ArgumentParser(description="Build deterministic draft packages from tracked raw sources.")
    parser.add_argument("vault_root")
    parser.add_argument("--write", action="store_true", help="Write draft package files into wiki/drafts/**")
    args = parser.parse_args()

    vault_root = Path(args.vault_root).resolve()
    print(json_dump(build_draft_packages(vault_root, write=args.write)))


if __name__ == "__main__":
    main()
