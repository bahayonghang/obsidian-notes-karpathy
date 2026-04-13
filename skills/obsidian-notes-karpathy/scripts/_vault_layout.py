from __future__ import annotations

import hashlib
import os
import re
from datetime import UTC, datetime
from pathlib import Path
from typing import Any

from _vault_common import iter_markdown_records, load_markdown


ARXIV_ID_RE = re.compile(r"(?<!\d)(\d{4}\.\d{4,5}(?:v\d+)?)(?!\d)")
ARXIV_URL_RE = re.compile(
    r"https?://(?:www\.)?(?:arxiv\.org/(?:abs|pdf)|alphaxiv\.org/(?:overview|abs))/([^?#/]+)"
)
PDF_SIDECAR_SUFFIX = ".source.md"
PDF_COMPANION_SKILLS = ("paper-workbench", "pdf")


def is_pdf_sidecar(path: Path) -> bool:
    return path.name.endswith(PDF_SIDECAR_SUFFIX)


def sidecar_for_pdf(raw_path: Path) -> Path:
    return raw_path.with_name(f"{raw_path.stem}{PDF_SIDECAR_SUFFIX}")


def detect_layout_family(vault_root: Path) -> str:
    has_review_gated = any(
        (vault_root / rel_path).exists()
        for rel_path in (
            "wiki/drafts",
            "wiki/live",
            "wiki/briefings",
            "outputs/reviews",
            "raw/human",
            "raw/agents",
        )
    )
    if has_review_gated:
        return "review-gated"

    has_legacy_layout = any(
        (vault_root / rel_path).exists()
        for rel_path in (
            "wiki/summaries",
            "wiki/concepts",
            "wiki/entities",
            "raw/articles",
            "raw/papers",
            "raw/podcasts",
            "raw/repos",
        )
    )
    if has_legacy_layout:
        return "legacy-layout"

    return "uninitialized"


def raw_source_metadata(vault_root: Path, raw_path: Path) -> dict[str, Any]:
    rel_parts = raw_path.relative_to(vault_root).parts
    metadata = {
        "capture_source": "legacy",
        "capture_trust": "legacy",
        "agent_role": None,
        "legacy_layout": True,
    }

    if len(rel_parts) >= 3 and rel_parts[1] == "human":
        metadata.update(
            {
                "capture_source": "human",
                "capture_trust": "curated",
                "legacy_layout": False,
            }
        )
    elif len(rel_parts) >= 4 and rel_parts[1] == "agents":
        metadata.update(
            {
                "capture_source": "agent",
                "capture_trust": "untrusted",
                "agent_role": rel_parts[2],
                "legacy_layout": False,
            }
        )

    return metadata


def source_class_for_raw(raw_path: Path) -> str:
    return "paper_pdf" if raw_path.suffix.lower() == ".pdf" else "markdown"


def configured_skill_roots() -> list[Path]:
    override = os.environ.get("KB_COMPANION_SKILL_PATHS")
    candidates: list[Path] = []

    if override is not None:
        candidates.extend(
            Path(part).expanduser()
            for part in override.split(os.pathsep)
            if part.strip()
        )
    else:
        codex_home = os.environ.get("CODEX_HOME")
        if codex_home:
            candidates.append(Path(codex_home).expanduser() / "skills")

        home = Path.home()
        candidates.extend(
            [
                home / ".codex" / "skills",
                home / ".claude" / "skills",
                home / ".agents" / "skills",
            ]
        )

    deduped: list[Path] = []
    seen: set[str] = set()
    for candidate in candidates:
        key = str(candidate)
        if key in seen:
            continue
        seen.add(key)
        deduped.append(candidate)
    return deduped


def detect_companion_skills(skill_roots: list[Path] | None = None) -> dict[str, Any]:
    roots = skill_roots if skill_roots is not None else configured_skill_roots()
    availability = {
        skill_name: any((root / skill_name / "SKILL.md").exists() for root in roots)
        for skill_name in PDF_COMPANION_SKILLS
    }
    return {
        "search_roots": [str(root) for root in roots],
        "skills": availability,
    }


def extract_paper_handle(candidate: Any) -> str | None:
    if not isinstance(candidate, str):
        return None

    text = candidate.strip()
    if not text:
        return None

    url_match = ARXIV_URL_RE.search(text)
    if url_match:
        url_tail = url_match.group(1).removesuffix(".pdf")
        handle_match = ARXIV_ID_RE.search(url_tail)
        if handle_match:
            return handle_match.group(1)

    handle_match = ARXIV_ID_RE.search(text)
    if handle_match:
        return handle_match.group(1)

    return None


def pdf_ingest_plan(
    vault_root: Path,
    raw_path: Path,
    companion_status: dict[str, bool],
) -> dict[str, Any]:
    plan: dict[str, Any] = {
        "source_class": "paper_pdf",
        "paper_handle": None,
        "paper_handle_source": None,
        "ingest_plan": None,
        "ingest_reason": None,
        "companion_status": dict(companion_status),
    }

    sidecar_path = sidecar_for_pdf(raw_path)
    if sidecar_path.exists():
        sidecar_record = load_markdown(sidecar_path, vault_root)
        plan["metadata_path"] = sidecar_record.path

        title = sidecar_record.frontmatter.get("title")
        if isinstance(title, str) and title.strip():
            plan["paper_title"] = title.strip()

        source_url = sidecar_record.frontmatter.get("source")
        if isinstance(source_url, str) and source_url.strip():
            plan["source_url"] = source_url.strip()

        for source_name, candidate in (
            ("paper_id", sidecar_record.frontmatter.get("paper_id")),
            ("source", sidecar_record.frontmatter.get("source")),
        ):
            handle = extract_paper_handle(candidate)
            if handle:
                plan["paper_handle"] = handle
                plan["paper_handle_source"] = source_name
                break

    if not plan["paper_handle"]:
        handle = extract_paper_handle(raw_path.stem)
        if handle:
            plan["paper_handle"] = handle
            plan["paper_handle_source"] = "filename"

    if companion_status["paper-workbench"]:
        plan["ingest_plan"] = "paper-workbench"
        plan["ingest_reason"] = "paper_workbench_directory_policy"
        return plan

    plan["ingest_plan"] = "skip"
    plan["ingest_reason"] = "paper_workbench_required_for_raw_papers"
    return plan


def accepted_raw_sources(vault_root: Path) -> list[Path]:
    raw_root = vault_root / "raw"
    if not raw_root.exists():
        return []

    sources: list[Path] = []
    for path in sorted(raw_root.rglob("*")):
        if not path.is_file():
            continue

        rel = path.relative_to(vault_root)
        if any(part.startswith(".") for part in rel.parts):
            continue
        if path.name.startswith("_"):
            continue
        if is_pdf_sidecar(path):
            continue
        if "assets" in rel.parts:
            continue

        suffix = path.suffix.lower()
        if suffix not in {".md", ".pdf"}:
            continue

        if suffix == ".pdf" and "papers" not in rel.parts:
            continue

        sources.append(path)

    return sources


def draft_summary_for_raw(vault_root: Path, raw_path: Path) -> Path:
    rel = raw_path.relative_to(vault_root / "raw")
    return vault_root / "wiki" / "drafts" / "summaries" / rel.with_suffix(".md")


def legacy_summary_for_raw(vault_root: Path, raw_path: Path) -> Path:
    summary_name = raw_path.name if raw_path.suffix.lower() == ".md" else f"{raw_path.stem}.md"
    return vault_root / "wiki" / "summaries" / summary_name


def summary_for_raw(vault_root: Path, raw_path: Path) -> Path:
    layout_family = detect_layout_family(vault_root)
    if layout_family == "legacy-layout":
        return legacy_summary_for_raw(vault_root, raw_path)
    return draft_summary_for_raw(vault_root, raw_path)


def iso_from_timestamp(timestamp: float) -> str:
    return datetime.fromtimestamp(timestamp, UTC).replace(microsecond=0).isoformat().replace("+00:00", "Z")


def compute_hash(path: Path) -> str:
    digest = hashlib.sha256()
    digest.update(path.read_bytes())
    return digest.hexdigest()


def collect_markdown_records(vault_root: Path):
    layout_family = detect_layout_family(vault_root)
    if layout_family == "review-gated":
        records = iter_markdown_records(
            vault_root,
            ("raw", "wiki/live", "wiki/briefings", "outputs/qa", "outputs/content", "outputs/episodes", "outputs/reviews"),
        )
        memory_path = vault_root / "MEMORY.md"
        if memory_path.exists():
            records.append(load_markdown(memory_path, vault_root))
        return records
    return iter_markdown_records(vault_root, ("raw", "wiki", "outputs/qa"))
