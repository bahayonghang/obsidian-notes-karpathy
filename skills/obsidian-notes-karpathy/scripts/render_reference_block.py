#!/usr/bin/env python3

from __future__ import annotations

import argparse
import sys
from pathlib import Path

SCRIPT_DIR = Path(__file__).resolve().parent
sys.path.insert(0, str(SCRIPT_DIR))

from _skill_reference_blocks import render_shared_reference_block


def main() -> None:
    parser = argparse.ArgumentParser(description="Render the shared reference bullet block for a core skill.")
    parser.add_argument("skill_name", help="Core skill name, such as kb-init or obsidian-notes-karpathy.")
    args = parser.parse_args()
    print(render_shared_reference_block(args.skill_name))


if __name__ == "__main__":
    main()
