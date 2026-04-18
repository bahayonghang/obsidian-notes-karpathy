from __future__ import annotations

from collections import Counter, defaultdict
from pathlib import Path
from typing import Any

from _vault_common import list_field, load_markdown, normalize_identity, parse_datetime, slugify_identity
from _vault_ingest import load_source_manifest, manifest_optional_metadata_for_source
from _vault_layout import (
    accepted_raw_sources,
    compute_hash,
    detect_companion_skills,
    detect_layout_family,
    iso_from_timestamp,
    manifest_path,
    pdf_ingest_plan,
    raw_source_metadata,
    source_class_for_raw,
    summary_for_raw,
)


def _staleness_hint(raw_mtime_dt: Any, now_dt: Any) -> tuple[bool, int | None]:
    if raw_mtime_dt is None or now_dt is None:
        return False, None
    age_days = max(0, int((now_dt - raw_mtime_dt).total_seconds() // 86400))
    return age_days > 730, age_days


def _identity_candidates(raw_path: Path, metadata: dict[str, Any]) -> set[str]:
    identities: set[str] = {raw_path.stem, raw_path.stem.replace("-", " ")}
    for key in ("title", "author"):
        value = metadata.get(key)
        if isinstance(value, str) and value.strip():
            identities.add(value.strip())
    return {identity for identity in identities if normalize_identity(identity)}


def _candidate_record(vault_root: Path, raw_path: Path) -> Any | None:
    if raw_path.suffix.lower() == ".md":
        return load_markdown(raw_path, vault_root)
    sidecar_path = raw_path.with_name(f"{raw_path.stem}.source.md")
    if sidecar_path.exists():
        return load_markdown(sidecar_path, vault_root)
    return None


def _clean_candidate_slug(value: str) -> str | None:
    slug = slugify_identity(value)
    if not slug:
        return None
    if slug in {"article", "articles", "note", "notes", "raw", "human", "agent", "agents"}:
        return None
    return slug


def _topic_candidates(vault_root: Path, raw_path: Path, record: Any | None) -> list[str]:
    rel = raw_path.relative_to(vault_root / "raw")
    candidates: set[str] = set()
    for part in rel.parts[:-1]:
        slug = _clean_candidate_slug(part)
        if slug:
            candidates.add(slug)
    if record is not None:
        for tag in list_field(record.frontmatter, "tags"):
            slug = _clean_candidate_slug(tag)
            if slug:
                candidates.add(slug)
    source_class = source_class_for_raw(raw_path)
    if source_class == "image_asset":
        candidates.add("assets")
    if source_class == "data_asset":
        candidates.add("data")
    if not candidates:
        fallback = _clean_candidate_slug(raw_path.stem)
        if fallback:
            candidates.add(fallback)
    return sorted(candidates)


def _concept_candidates(raw_path: Path, record: Any | None) -> list[str]:
    candidates: set[str] = set()
    if record is not None:
        for key in ("concepts", "tags"):
            for value in list_field(record.frontmatter, key):
                slug = _clean_candidate_slug(value)
                if slug:
                    candidates.add(slug)
        title = record.frontmatter.get("title")
        if isinstance(title, str) and title.strip():
            slug = _clean_candidate_slug(title)
            if slug:
                candidates.add(slug)
    if not candidates:
        slug = _clean_candidate_slug(raw_path.stem)
        if slug:
            candidates.add(slug)
    return sorted(candidates)


def _entity_candidates(raw_path: Path, record: Any | None) -> list[str]:
    candidates: set[str] = set()
    if record is not None:
        for key in ("author", "project", "tool", "library", "dataset", "entity"):
            value = record.frontmatter.get(key)
            if isinstance(value, str) and value.strip():
                slug = _clean_candidate_slug(value)
                if slug:
                    candidates.add(slug)
        for value in list_field(record.frontmatter, "authors"):
            slug = _clean_candidate_slug(value)
            if slug:
                candidates.add(slug)
    rel_parts = raw_path.parts
    if "repos" in rel_parts:
        slug = _clean_candidate_slug(raw_path.stem)
        if slug:
            candidates.add(slug)
    return sorted(candidates)


def _relationship_candidates(topic_candidates: list[str], concept_candidates: list[str], entity_candidates: list[str]) -> list[str]:
    relationships: list[str] = []
    relationships.extend(f"belongs_to:[[wiki/live/topics/{slug}]]" for slug in topic_candidates)
    relationships.extend(f"supports:[[wiki/live/concepts/{slug}]]" for slug in concept_candidates)
    relationships.extend(f"mentions:[[wiki/live/entities/{slug}]]" for slug in entity_candidates)
    return relationships


def _review_package_meta_path(vault_root: Path, raw_path: Path) -> Path:
    rel = raw_path.relative_to(vault_root / "raw")
    return vault_root / "wiki" / "drafts" / "indices" / "packages" / rel.with_suffix(".md")


def _source_package(vault_root: Path, raw_path: Path) -> dict[str, Any]:
    record = _candidate_record(vault_root, raw_path)
    topic_candidates = _topic_candidates(vault_root, raw_path, record)
    concept_candidates = _concept_candidates(raw_path, record)
    entity_candidates = _entity_candidates(raw_path, record)
    relationship_candidates = _relationship_candidates(topic_candidates, concept_candidates, entity_candidates)
    review_meta_path = _review_package_meta_path(vault_root, raw_path).relative_to(vault_root).as_posix()
    return {
        "summary": summary_for_raw(vault_root, raw_path).relative_to(vault_root).as_posix(),
        "concept_candidates": concept_candidates,
        "entity_candidates": entity_candidates,
        "relationship_candidates": relationship_candidates,
        "topic_candidates": topic_candidates,
        "review_package_meta": review_meta_path,
    }


def _record_list(record: Any | None, *keys: str) -> list[str]:
    if record is None:
        return []
    values: list[str] = []
    for key in keys:
        values.extend(list_field(record.frontmatter, key))
    return sorted(dict.fromkeys(value.strip() for value in values if value.strip()))


def _compile_method_payload(
    record: Any | None,
    topic_candidates: list[str],
    concept_candidates: list[str],
) -> dict[str, Any]:
    boundary_conditions = _record_list(record, "boundary_conditions", "limits", "boundary_notes")
    assumption_flags = _record_list(record, "assumption_flags", "assumptions")
    transfer_targets = _record_list(record, "transfer_targets", "cross_domain_targets", "transferable_to")
    core_conclusions = _record_list(record, "core_conclusions")
    key_evidence = _record_list(record, "key_evidence")

    if not core_conclusions and record is not None:
        title = record.frontmatter.get("title")
        if isinstance(title, str) and title.strip():
            core_conclusions = [f"{title.strip()} introduces a durable candidate for the review gate."]
    if not key_evidence and record is not None:
        source = record.frontmatter.get("source")
        if isinstance(source, str) and source.strip():
            key_evidence = [source.strip()]
    if not transfer_targets:
        transfer_targets = [f"hub:{topic_candidates[0]}"] if topic_candidates else []
        if not transfer_targets and concept_candidates:
            transfer_targets = [f"concept:{concept_candidates[0]}"]

    promotion_target = "semantic"
    if record is not None:
        explicit = record.frontmatter.get("promotion_target")
        if isinstance(explicit, str) and explicit.strip():
            promotion_target = explicit.strip()

    return {
        "boundary_conditions": boundary_conditions,
        "assumption_flags": assumption_flags,
        "transfer_targets": transfer_targets,
        "core_conclusions": core_conclusions,
        "key_evidence": key_evidence,
        "promotion_target": promotion_target,
    }


def scan_compile_delta(vault_root: Path) -> dict[str, Any]:
    items: list[dict[str, Any]] = []
    counts = Counter()
    ingest_counts = Counter()
    companion_skills = detect_companion_skills()
    companion_status = companion_skills["skills"]
    layout_family = detect_layout_family(vault_root)
    manifest = load_source_manifest(vault_root)
    manifest_by_path = {
        str(item.get("path")): item
        for item in manifest.get("sources", [])
        if isinstance(item, dict) and item.get("path")
    }
    now_dt = parse_datetime(iso_from_timestamp(__import__("time").time()))

    identity_registry: dict[str, set[str]] = defaultdict(set)
    for root in (vault_root / "wiki" / "drafts" / "concepts", vault_root / "wiki" / "live" / "concepts", vault_root / "wiki" / "drafts" / "entities", vault_root / "wiki" / "live" / "entities"):
        if not root.exists():
            continue
        for candidate in sorted(root.rglob("*.md")):
            record = load_markdown(candidate, vault_root)
            basename = candidate.stem
            identity_registry[slugify_identity(basename)].add(record.path)
            for key in ("title", "canonical_name"):
                value = record.frontmatter.get(key)
                if isinstance(value, str) and value.strip():
                    identity_registry[slugify_identity(value)].add(record.path)
            aliases = record.frontmatter.get("aliases", [])
            if isinstance(aliases, list):
                for alias in aliases:
                    if isinstance(alias, str) and alias.strip():
                        identity_registry[slugify_identity(alias)].add(record.path)

    for raw_path in accepted_raw_sources(vault_root):
        rel_path = raw_path.relative_to(vault_root).as_posix()
        summary_path = summary_for_raw(vault_root, raw_path)
        raw_hash = compute_hash(raw_path)
        raw_mtime = iso_from_timestamp(raw_path.stat().st_mtime)
        item: dict[str, Any] = {
            "path": rel_path,
            "summary_path": summary_path.relative_to(vault_root).as_posix(),
            "raw_hash": raw_hash,
            "raw_mtime": raw_mtime,
            "source_class": source_class_for_raw(raw_path),
            "layout_family": layout_family,
            "manifest_path": manifest_path(vault_root).relative_to(vault_root).as_posix(),
        }
        item.update(raw_source_metadata(vault_root, raw_path))
        manifest_entry = manifest_by_path.get(rel_path)
        item["manifest_status"] = "tracked" if manifest_entry else "untracked"
        if manifest_entry:
            item["manifest_source_id"] = manifest_entry.get("source_id")
            item["source_url_or_handle"] = manifest_entry.get("source_url_or_handle") or ""
            item["capture_method"] = manifest_entry.get("capture_method") or ""
            item["linked_assets"] = manifest_entry.get("linked_assets") or []
            item["source_profile"] = manifest_entry.get("source_profile") or ""
        raw_mtime_dt = parse_datetime(raw_mtime)
        stale_hint, age_days = _staleness_hint(raw_mtime_dt, now_dt)
        item["last_verified_at"] = raw_mtime
        item["possibly_outdated"] = stale_hint
        if age_days is not None:
            item["source_age_days"] = age_days

        alias_candidates = sorted({slugify_identity(identity) for identity in _identity_candidates(raw_path, item) if slugify_identity(identity)})
        duplicate_paths = sorted({path for candidate in alias_candidates for path in identity_registry.get(candidate, set())})
        item["alias_candidates"] = alias_candidates
        item["duplicate_candidates"] = duplicate_paths
        if item["source_class"] == "paper_pdf":
            item.update(pdf_ingest_plan(vault_root, raw_path, companion_status))
        else:
            item["ingest_plan"] = "markdown"
            item["ingest_reason"] = "markdown_source"
        if not manifest_entry:
            item.update(
                manifest_optional_metadata_for_source(
                    vault_root,
                    raw_path,
                    item["source_class"],
                    item if item["source_class"] == "paper_pdf" else None,
                )
            )

        item["source_package"] = _source_package(vault_root, raw_path)
        item["source_package"]["capture_method"] = item.get("capture_method") or ""
        item["source_package"]["linked_assets"] = item.get("linked_assets") or []
        if item.get("source_profile"):
            item["source_package"]["source_profile"] = item["source_profile"]

        ingest_counts[item["ingest_plan"]] += 1
        if item["ingest_plan"] == "skip":
            counts["skipped"] += 1

        if not summary_path.exists():
            item["status"] = "new"
            item["reason"] = "missing_summary"
            counts["new"] += 1
            items.append(item)
            continue

        summary_record = load_markdown(summary_path, vault_root)
        summary_fm = summary_record.frontmatter
        summary_hash = summary_fm.get("source_hash")
        summary_mtime = parse_datetime(summary_fm.get("source_mtime"))
        raw_mtime_dt = parse_datetime(raw_mtime)

        if summary_hash:
            item["recorded_source_hash"] = summary_hash
            if summary_hash == raw_hash:
                item["status"] = "unchanged"
                item["reason"] = "source_hash_match"
                counts["unchanged"] += 1
            else:
                item["status"] = "changed"
                item["reason"] = "source_hash_mismatch"
                counts["changed"] += 1
            items.append(item)
            continue

        recorded_source_mtime = summary_fm.get("source_mtime")
        if summary_mtime and raw_mtime_dt and raw_mtime_dt > summary_mtime:
            item["status"] = "changed"
            item["reason"] = "source_mtime_outdated"
            item["recorded_source_mtime"] = recorded_source_mtime or ""
            counts["changed"] += 1
        else:
            item["status"] = "unchanged"
            item["reason"] = "source_mtime_current"
            item["recorded_source_mtime"] = recorded_source_mtime or ""
            counts["unchanged"] += 1

        items.append(item)

    payload_counts = {
        "new": counts["new"],
        "changed": counts["changed"],
        "unchanged": counts["unchanged"],
        "skipped": counts["skipped"],
    }

    return {
        "vault_root": str(vault_root),
        "layout_family": layout_family,
        "counts": payload_counts,
        "ingest_counts": dict(sorted(ingest_counts.items())),
        "companion_skills": companion_skills,
        "items": items,
    }


def _title_for_source(raw_path: Path, record: Any | None) -> str:
    if record is not None:
        title = record.frontmatter.get("title")
        if isinstance(title, str) and title.strip():
            return title.strip()
    return raw_path.stem.replace("-", " ").strip().title()


def _write_markdown(path: Path, lines: list[str]) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text("\n".join(lines) + "\n", encoding="utf-8")


def _merge_topic_source_refs(path: Path, source_ref: str) -> list[str]:
    existing_refs: list[str] = []
    if path.exists():
        record = load_markdown(path)
        existing_refs = list_field(record.frontmatter, "source_refs")
    merged = sorted({*existing_refs, source_ref})
    return merged


def build_draft_packages(vault_root: Path, *, write: bool = False) -> dict[str, Any]:
    delta = scan_compile_delta(vault_root)
    packages: list[dict[str, Any]] = []
    written_paths: list[str] = []

    for item in delta["items"]:
        if item["status"] not in {"new", "changed"}:
            continue
        if item.get("ingest_plan") == "skip":
            continue

        raw_path = vault_root / item["path"]
        record = _candidate_record(vault_root, raw_path)
        topic_candidates = item["source_package"]["topic_candidates"]
        concept_candidates = item["source_package"]["concept_candidates"]
        entity_candidates = item["source_package"]["entity_candidates"]
        relationship_candidates = item["source_package"]["relationship_candidates"]
        compile_method = _compile_method_payload(record, topic_candidates, concept_candidates)
        package = {
            "source_id": item.get("manifest_source_id") or item["path"].replace("/", "--").removesuffix(".md"),
            "source_path": item["path"],
            "title": _title_for_source(raw_path, record),
            "source_package": item["source_package"],
            "capture_source": item["capture_source"],
            "source_class": item["source_class"],
            "capture_method": item.get("capture_method") or "",
            "linked_assets": item.get("linked_assets") or [],
            "source_profile": item.get("source_profile") or "",
            "compile_method": compile_method,
        }
        packages.append(package)

        if not write:
            continue

        summary_path = vault_root / item["source_package"]["summary"]
        review_meta_path = vault_root / item["source_package"]["review_package_meta"]
        source_ref = f"[[{item['path'].removesuffix('.md')}]]" if item["path"].endswith(".md") else item["path"]

        summary_lines = [
            "---",
            f'title: "{package["title"]}"',
            f'source_file: "{source_ref}"',
            f'source_hash: "{item["raw_hash"]}"',
            f'source_mtime: "{item["raw_mtime"]}"',
            f'draft_id: "{package["source_id"]}"',
            f'compiled_from: "{source_ref}"',
            "capture_sources:",
            f'  - "{source_ref}"',
            'review_state: "pending"',
            'review_score: "0.75"',
            "blocking_flags: []",
            f'promotion_target: "{compile_method["promotion_target"]}"',
            "topic_candidates:",
        ]
        summary_lines.extend(f'  - "{slug}"' for slug in topic_candidates) if topic_candidates else summary_lines.append('  - ""')
        summary_lines.append("concept_candidates:")
        summary_lines.extend(f'  - "{slug}"' for slug in concept_candidates) if concept_candidates else summary_lines.append('  - ""')
        summary_lines.append("entity_candidates:")
        summary_lines.extend(f'  - "{slug}"' for slug in entity_candidates) if entity_candidates else summary_lines.append('  - ""')
        summary_lines.append("relationship_candidates:")
        summary_lines.extend(f'  - "{rel}"' for rel in relationship_candidates) if relationship_candidates else summary_lines.append('  - ""')
        summary_lines.append("boundary_conditions:")
        summary_lines.extend(f'  - "{value}"' for value in compile_method["boundary_conditions"]) if compile_method["boundary_conditions"] else summary_lines.append('  - ""')
        summary_lines.append("assumption_flags:")
        summary_lines.extend(f'  - "{value}"' for value in compile_method["assumption_flags"]) if compile_method["assumption_flags"] else summary_lines.append('  - ""')
        summary_lines.append("transfer_targets:")
        summary_lines.extend(f'  - "{value}"' for value in compile_method["transfer_targets"]) if compile_method["transfer_targets"] else summary_lines.append('  - ""')
        summary_lines.extend(
            [
                f'review_package_meta: "[[{item["source_package"]["review_package_meta"].removesuffix(".md")}]]"',
                "---",
                "",
                f"# {package['title']}",
                "",
                "## Source Package",
                "",
                f"- source_path: `{item['path']}`",
                f"- source_class: `{item['source_class']}`",
                f"- capture_source: `{item['capture_source']}`",
                f"- capture_method: `{item.get('capture_method') or 'unknown'}`",
                f"- source_profile: `{item.get('source_profile') or 'none'}`",
                "",
                "## Compression",
                "",
                "### Core Conclusions",
                "",
            ]
        )
        summary_lines.extend(f"- {value}" for value in compile_method["core_conclusions"]) if compile_method["core_conclusions"] else summary_lines.append("- No core conclusions extracted yet.")
        summary_lines.extend(
            [
                "",
                "### Key Evidence",
                "",
            ]
        )
        summary_lines.extend(f"- {value}" for value in compile_method["key_evidence"]) if compile_method["key_evidence"] else summary_lines.append("- No key evidence extracted yet.")
        summary_lines.extend(
            [
                "",
                "## Assumption Checks",
                "",
                "### Assumption Flags",
                "",
            ]
        )
        summary_lines.extend(f"- {value}" for value in compile_method["assumption_flags"]) if compile_method["assumption_flags"] else summary_lines.append("- None surfaced yet.")
        summary_lines.extend(
            [
                "",
                "### Boundary Conditions",
                "",
            ]
        )
        summary_lines.extend(f"- {value}" for value in compile_method["boundary_conditions"]) if compile_method["boundary_conditions"] else summary_lines.append("- None surfaced yet.")
        summary_lines.extend(
            [
                "",
                "## Transfer Targets",
                "",
            ]
        )
        summary_lines.extend(f"- {value}" for value in compile_method["transfer_targets"]) if compile_method["transfer_targets"] else summary_lines.append("- None surfaced yet.")
        if item.get("linked_assets"):
            summary_lines.extend(
                [
                    "",
                    "## Linked Assets",
                    "",
                ]
            )
            summary_lines.extend(f"- `{value}`" for value in item["linked_assets"])
        summary_lines.extend(
            [
                "",
                "## Candidate Topics",
                "",
            ]
        )
        summary_lines.extend(f"- [[wiki/drafts/topics/{slug}]]" for slug in topic_candidates) if topic_candidates else summary_lines.append("- None")
        summary_lines.extend(
            [
                "",
                "## Candidate Concepts",
                "",
            ]
        )
        summary_lines.extend(f"- `{slug}`" for slug in concept_candidates) if concept_candidates else summary_lines.append("- None")
        summary_lines.extend(
            [
                "",
                "## Candidate Entities",
                "",
            ]
        )
        summary_lines.extend(f"- `{slug}`" for slug in entity_candidates) if entity_candidates else summary_lines.append("- None")
        summary_lines.extend(
            [
                "",
                "## Relationship Candidates",
                "",
            ]
        )
        summary_lines.extend(f"- `{rel}`" for rel in relationship_candidates) if relationship_candidates else summary_lines.append("- None")
        _write_markdown(summary_path, summary_lines)
        written_paths.append(summary_path.relative_to(vault_root).as_posix())

        for slug in topic_candidates:
            topic_path = vault_root / "wiki" / "drafts" / "topics" / f"{slug}.md"
            source_refs = _merge_topic_source_refs(topic_path, source_ref)
            topic_lines = [
                "---",
                f'title: "{slug.replace("-", " ").title()}"',
                f'draft_id: "topic-{slug}"',
                'review_state: "pending"',
                'review_score: "0.70"',
                "source_refs:",
            ]
            topic_lines.extend(f'  - "{ref}"' for ref in source_refs)
            topic_lines.extend(
                [
                    "---",
                    "",
                    f"# {slug.replace('-', ' ').title()}",
                    "",
                    "## Why this topic exists",
                    "",
                    "- Candidate browse-layer topic compiled from recent source packages.",
                    "",
                    "## Source Refs",
                    "",
                ]
            )
            topic_lines.extend(f"- {ref}" for ref in source_refs)
            _write_markdown(topic_path, topic_lines)
            topic_rel = topic_path.relative_to(vault_root).as_posix()
            if topic_rel not in written_paths:
                written_paths.append(topic_rel)

        package_lines = [
            "---",
            f'title: "Review Package: {package["source_id"]}"',
            f'source_id: "{package["source_id"]}"',
            'review_state: "pending"',
            f'summary_path: "[[{item["source_package"]["summary"].removesuffix(".md")}]]"',
            f'promotion_target: "{compile_method["promotion_target"]}"',
            "topic_candidates:",
        ]
        package_lines.extend(f'  - "[[wiki/drafts/topics/{slug}]]"' for slug in topic_candidates) if topic_candidates else package_lines.append('  - ""')
        package_lines.extend(
            [
                "relationship_candidates:",
            ]
        )
        package_lines.extend(f'  - "{rel}"' for rel in relationship_candidates) if relationship_candidates else package_lines.append('  - ""')
        package_lines.extend(
            [
                "---",
                "",
                f"# Review Package: {package['source_id']}",
                "",
                f"- summary: [[{item['source_package']['summary'].removesuffix('.md')}]]",
                f"- source: `{item['path']}`",
                f"- capture_method: `{item.get('capture_method') or 'unknown'}`",
                "",
            ]
        )
        _write_markdown(review_meta_path, package_lines)
        written_paths.append(review_meta_path.relative_to(vault_root).as_posix())

    return {
        "vault_root": str(vault_root),
        "package_count": len(packages),
        "packages": packages,
        "written_paths": written_paths,
    }
