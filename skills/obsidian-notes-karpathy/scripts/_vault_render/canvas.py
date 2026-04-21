from __future__ import annotations

from typing import Any


def canvas_payload(title: str, source_live_pages: list[str]) -> dict[str, Any]:
    return {
        "nodes": [
            {
                "id": "title",
                "type": "text",
                "x": 80,
                "y": 80,
                "width": 320,
                "height": 80,
                "text": title,
            },
            {
                "id": "sources",
                "type": "text",
                "x": 80,
                "y": 200,
                "width": 520,
                "height": 220,
                "text": "\n".join(source_live_pages or ["No approved live pages supplied"]),
            },
        ],
        "edges": [],
    }
