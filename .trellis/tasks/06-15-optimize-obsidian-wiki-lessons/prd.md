# Optimize KB CLI and skill ecosystem from obsidian-wiki analysis

## Goal

Turn the obsidian-wiki comparison into a prioritized optimization backlog for this repo, with an explicit split between CLI/distribution improvements and skill/ecosystem improvements, while preserving the review-gated Rust core.

## Confirmed Facts

- This repo already has a deterministic Rust CLI and a review-gated contract centered on `raw -> draft -> review -> live`.
- The reference repo is stronger at installation/distribution, multi-agent bootstrap coverage, cross-agent history tooling, and query/export ergonomics.
- The reference repo is weaker on truth-boundary rigor than this repo, so the review gate must stay intact.

## Requirements

- Select the highest-value improvements worth adopting from the reference repo.
- Keep CLI/distribution concerns separate from skill/ecosystem concerns.
- Preserve the approved/live truth boundary and the Rust runtime as the authoritative core.
- Define at least one recommended path and one rejected alternative.
- Produce a phaseable backlog that can later be split into child tasks if implementation is approved.

## Out of Scope

- Replacing the Rust CLI with Python or shell scripts.
- Adopting the reference repo's single-layer wiki semantics.
- Introducing any write path that bypasses `kb-review`.
- Porting every reference-repo skill as-is.

## Acceptance Criteria

- [ ] The task artifacts state one recommended optimization path and why it is preferred.
- [ ] The task artifacts state at least one rejected alternative and why it is not chosen.
- [ ] CLI/distribution and skill/ecosystem changes are separated into distinct phases or child-task candidates.
- [ ] The plan names the impacted surfaces in this repo: CLI, skills, docs, tests, and fixtures.
- [ ] The plan includes validation gates and rollback boundaries.
- [ ] No unresolved template text remains in the task artifacts.

## Notes

- Keep `prd.md` focused on requirements, constraints, and acceptance criteria.
- Lightweight tasks can remain PRD-only.
- For complex tasks, add `design.md` for technical design and `implement.md` for execution planning before `task.py start`.
