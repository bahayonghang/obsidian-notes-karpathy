#!/usr/bin/env python3

from __future__ import annotations

import argparse
import sys
from pathlib import Path

SCRIPT_DIR = Path(__file__).resolve().parent
sys.path.insert(0, str(SCRIPT_DIR))

from _vault_render import render_artifact
from _vault_utils import json_dump


def main() -> None:
    parser = argparse.ArgumentParser(description="Render deterministic slides/charts/canvas/report artifacts from approved knowledge.")
    parser.add_argument("vault_root")
    parser.add_argument("--mode", required=True, choices=("slides", "charts", "canvas", "report", "web"))
    parser.add_argument("--source", action="append", dest="sources", default=[], help="Relative markdown source path. Repeat for multiple inputs.")
    parser.add_argument("--output", help="Relative output path to write.")
    parser.add_argument("--title", help="Optional explicit title.")
    parser.add_argument("--write", action="store_true", help="Write the output artifact.")
    args = parser.parse_args()

    vault_root = Path(args.vault_root).resolve()
    print(
        json_dump(
            render_artifact(
                vault_root,
                mode=args.mode,
                source_paths=args.sources,
                output_path=args.output,
                title=args.title,
                write=args.write,
            )
        )
    )


if __name__ == "__main__":
    main()
