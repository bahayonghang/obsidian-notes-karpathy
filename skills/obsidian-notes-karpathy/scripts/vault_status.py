#!/usr/bin/env python3

from __future__ import annotations

import argparse
import sys
from pathlib import Path

SCRIPT_DIR = Path(__file__).resolve().parent
sys.path.insert(0, str(SCRIPT_DIR))

from _vault_init import describe_vault_status
from _vault_utils import json_dump


def main() -> None:
    parser = argparse.ArgumentParser(description="Print the current lifecycle stage and high-level counts for a vault.")
    parser.add_argument("vault_root", help="Path to the target vault root.")
    parser.add_argument("--format", choices=("summary", "json"), default="summary", help="Output mode.")
    args = parser.parse_args()

    payload = describe_vault_status(Path(args.vault_root).resolve())
    if args.format == "json":
        print(json_dump(payload))
    else:
        print(payload["summary"])


if __name__ == "__main__":
    main()
