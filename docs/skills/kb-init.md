# kb-init

Initialize a new Obsidian vault or sub-vault for the Karpathy-style workflow.

## Creates

```text
raw/{articles,papers,podcasts,assets}
wiki/{concepts,summaries,indices}
wiki/index.md
wiki/log.md
outputs/{qa,health,reports,slides,charts,content/{articles,threads,talks}}
AGENTS.md
CLAUDE.md
```

## Important behavior

- creates both `AGENTS.md` and `CLAUDE.md`
- keeps the two schema files aligned
- establishes the rule that `raw/` is immutable
- creates zero-state indices and log files so later skills have a stable starting point
- can provision publish-mode output folders
- seeds an example concept page and a clipper template

## After kb-init

1. add sources into `raw/`
2. run `kb-compile`
3. ask a substantive question with `kb-query`
4. generate one article or thread draft if content output matters
5. run `kb-health` for the first maintenance baseline
