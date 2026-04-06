# Docs Maintenance

This folder contains the bilingual VitePress site for Obsidian Notes Karpathy.

## Source of truth

Treat the skill contracts as authoritative:

- `skills/*/SKILL.md` defines trigger surface, lifecycle intent, required reads, writes, and output guarantees.
- `skills/obsidian-notes-karpathy/references/*` defines the shared file model, lifecycle matrix, templates, health rubric, and search posture.
- `skills/obsidian-notes-karpathy/scripts/*` defines deterministic helper behavior that docs may describe, but not reinterpret.

The site should explain those contracts for humans. It should not invent parallel behavior.

## Documentation shape

```text
docs/
├── index.md               # English landing page
├── guide/                 # Mental model, quick start, installation, structure
├── skills/                # Skill-by-skill routing and behavior
├── workflow/              # End-to-end lifecycle flow
├── zh/                    # Chinese mirror of the public docs
└── .vitepress/            # Site configuration and theme
```

## Editing rules

1. Update the docs when a `SKILL.md` contract changes in a reader-visible way.
2. Keep English and Chinese pages aligned in scope, even if the wording differs.
3. Prefer summaries, routing matrices, and lifecycle explanations over duplicating full skill prose.
4. Do not document behavior that is not grounded in the current skill bundle.
5. When a page starts drifting from the skill contract, fix the page or delete the duplicated claim.

## Local development

Install dependencies:

```bash
npm install
```

Start the dev server:

```bash
npm run dev
```

Build the static site:

```bash
npm run build
```

Preview the production build:

```bash
npm run preview
```

The build output is written to `.vitepress/dist/`.
