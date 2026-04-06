# kb-init

Initialize or repair a Karpathy-style vault contract.

## What it creates or repairs

```text
raw/{articles,papers,podcasts,assets}
wiki/{concepts,summaries,indices}
wiki/index.md
wiki/log.md
outputs/{qa,health,reports,slides,charts,content/{articles,threads,talks}}
AGENTS.md
CLAUDE.md
```

Optional expansions:

- `raw/repos/`
- `wiki/entities/`
- `wiki/indices/ENTITIES.md`

## What it decides up front

- the target vault root
- fresh setup versus repair
- whether publish-mode folders are needed now
- whether repo ingestion or entity coverage should be enabled now

## Key guarantees

- `raw/` is marked as immutable from the compiler's point of view
- `AGENTS.md` and `CLAUDE.md` stay aligned on the file model
- zero-state index pages and log files exist before other skills run
- the vault starts with a concrete concept example and clipper template
- missing support files are treated as repair work, not as a hard stop

## What it writes

- the local guidance contract
- starter indices such as `INDEX.md`, `CONCEPTS.md`, `SOURCES.md`, and `RECENT.md`
- `wiki/index.md` and `wiki/log.md`
- example concept and optional entity pages
- the raw clipper template

## Recommended hand-off

1. add 5 to 10 real sources under `raw/`
2. run `kb-compile`
3. ask one substantive question with `kb-query`
4. publish one artifact if the vault is meant to produce outward-facing content
5. run `kb-health` for the first maintenance baseline
