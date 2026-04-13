# Profile Contract

Use this reference when the workflow needs different operating postures without changing the truth boundary.

## Supported profiles

- `governed-team`
- `standard`
- `fast-personal`

Default: `governed-team`

## Behavior

### governed-team

- strict review gate
- briefings participate in default query scope
- health surfaces the full governance signal set

### standard

- same truth boundary as governed-team
- lighter maintenance noise for non-critical backlog signals
- briefings still participate in default query scope

### fast-personal

- same truth boundary as governed-team
- briefings do not participate in default query scope
- stale briefing and low-severity health noise may be delayed
- batch compile is acceptable, but draft -> review -> live still holds

## Configuration posture

The profile may be stored in `raw/_manifest.yaml` as a top-level `profile` field or in `wiki/index.md` frontmatter as `kb_profile`.
