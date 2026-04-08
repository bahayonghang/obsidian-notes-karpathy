# obsidian-notes-karpathy

Use the package entry skill when the user describes the Obsidian vault workflow as a whole or asks what lifecycle step should run next.

If the operation is already clear, prefer the operation-specific skill instead of the router.

## Main routing outcomes

- `kb-init` for fresh, partial, or legacy-layout vaults
- `kb-compile` when raw captures are ahead of drafts, including bootstrap `raw/*.md` sources
- `kb-review` when drafts are pending or briefings are stale
- `kb-query` when the live layer is ready for retrieval
- `kb-health` when approved knowledge needs maintenance, writeback backlog is piling up, or collaboration memory and approved knowledge appear to be mixing
