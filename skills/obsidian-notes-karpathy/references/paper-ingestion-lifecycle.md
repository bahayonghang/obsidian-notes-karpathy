# Paper Ingestion Lifecycle

Use this reference when the vault includes literature or paper captures under `raw/**/papers/` and the workflow needs to explain how those sources integrate without bypassing the review gate.

## Core rule

Paper PDFs are valid evidence inputs, but they do not follow the normal markdown-only compile lane directly.

`raw/**/papers/*.pdf` should still route through `paper-workbench` or the dedicated paper-processing workflow before the resulting knowledge can enter normal draft compilation.

## Intake posture

Papers may appear under:

- `raw/human/papers/`
- legacy paper subtrees inside raw during migration

The intake layer should preserve:

- source identity
- title when known
- author or venue metadata when available
- publication year / date when available
- paper handle such as DOI, arXiv ID, or other stable locator when available
- `last_verified_at` and `possibly_outdated` when useful

## Normalized downstream path

1. ingest paper artifact into `raw/**/papers/`
2. route through paper-processing workflow
3. produce markdown evidence notes or structured extraction outputs
4. compile those normalized outputs into `wiki/drafts/`
5. review through `kb-review`
6. promote approved synthesis into `wiki/live/`

## Provenance expectations

Paper-derived summaries, concepts, and entities should preserve enough provenance to answer:

- which paper artifact or extraction they came from
- which stable paper locator identifies the source
- when the source was last checked
- where uncertainty or interpretation remains

Where practical, preserve section-, page-, or excerpt-level evidence cues instead of only citing the paper at a coarse level.

## Review posture

Paper-derived knowledge is often interpretation-heavy.

Review should be especially careful about:

- separating source claims from synthesis
- preserving uncertainty
- avoiding overconfident concept promotion from a single paper
- distinguishing evidence from speculation or future work sections

## Query posture

Once approved, literature-derived live pages are queried the same way as other approved knowledge.

But paper artifacts or raw PDFs do not become retrieval truth directly unless a human is explicitly inspecting source evidence.
