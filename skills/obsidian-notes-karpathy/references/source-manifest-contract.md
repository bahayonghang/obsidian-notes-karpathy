# Source Manifest Contract

Use this reference whenever the workflow needs a canonical raw-source registry.

## Canonical path

- `raw/_manifest.yaml`

This file is the user-visible source registry for the vault. It does not replace `raw/`; it indexes `raw/`.

## Required fields per source entry

- `source_id`
- `path`
- `source_type`
- `capture_origin`
- `source_url_or_handle`
- `content_hash`
- `first_seen_at`
- `last_seen_at`
- `ingest_status`
- `normalized_outputs`

Optional fields may include:

- `deferred_to`
- `metadata_path`
- `capture_method`
- `linked_assets`
- `source_profile`

## Status posture

- `ready-for-compile` means the source is tracked and can enter the compile lane.
- `deferred` means the source is tracked but must be handled by a companion workflow first.
- `deferred-missing-skill` means the source is valid but blocked on a missing companion skill.

## Boundary rules

- Updating the manifest must not rewrite `raw/**` source files.
- The manifest can point at candidate downstream outputs, but it does not widen the truth boundary.
- Paper PDFs under `raw/**/papers/*.pdf` must be recorded in the manifest even when compile cannot process them yet.
- `capture_method` should help distinguish Web Clipper, browser/CDP capture, manual markdown, agent capture, or file-drop intake without changing the truth boundary.
- `linked_assets` should list local image or attachment paths when the markdown source depends on them.
- `source_profile` can record the originating creator/account/profile context when the vault needs downstream editorial consistency checks.
