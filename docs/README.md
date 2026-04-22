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
├── architecture/          # Design rationale and planned evolution
├── zh/                    # Chinese mirror of the public docs
└── .vitepress/            # Site configuration and theme
```

## Editing rules

1. Update the docs when a `SKILL.md` contract changes in a reader-visible way.
2. Keep English and Chinese pages aligned in scope, even if the wording differs.
3. Prefer summaries, routing matrices, and lifecycle explanations over duplicating full skill prose.
4. `guide/`, `skills/`, and `workflow/` document shipped behavior only.
5. `architecture/` may document design rationale and planned work, but must clearly label what is not yet shipped.
6. Do not document future behavior in contract pages.
7. When a page starts drifting from the skill contract, fix the page or delete the duplicated claim.

## Local development

From the repo root, prefer the `just` shortcuts:

```bash
just install
just docs
just docs-build
```

If you are already inside `docs/`, the equivalent npm commands are:

```bash
npm install
npm run dev
npm run build
npm run preview
```

The build output is written to `.vitepress/dist/`.

## Dependency hygiene

- `docs/package.json` pins `vitepress` and uses npm `overrides` to force `vite` onto a patched release line.
- Keep `docs/package-lock.json` committed whenever the docs toolchain changes.
- After bumping docs dependencies, run:

```bash
npm audit
npm run build
```

- Do not treat a green install as sufficient. The expected verification is `0 vulnerabilities` plus a successful VitePress build.
