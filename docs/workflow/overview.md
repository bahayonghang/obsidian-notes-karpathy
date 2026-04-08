# Workflow Overview

The workflow has one routing stage and five operational stages.

## Enter by symptom

| If the vault or request looks like this | Start here |
| --- | --- |
| The contract is missing, partial, or still in a legacy-layout | `kb-init` |
| New raw captures have not been compiled into drafts yet | `kb-compile` |
| Drafts are waiting for approval or briefings are stale and should be rebuilt in the next gate pass | `kb-review` |
| The user wants an answer, report, article, slides, or a thread | `kb-query` |
| The approved layer needs a maintenance baseline, drift audit, or safe cleanup pass | `kb-health` |
| The correct lifecycle step is unclear | `obsidian-notes-karpathy` |

If some live content exists but `wiki/drafts/`, `wiki/briefings/`, or `outputs/reviews/` is still missing, start at `kb-init` anyway. Structural repair takes priority over normal query work.

```mermaid
graph LR
    A[obsidian-notes-karpathy] --> B[kb-init]
    A --> C[kb-compile]
    A --> D[kb-review]
    A --> E[kb-query]
    A --> F[kb-health]
    B --> C
    C --> D
    D --> E
    D --> F
    E -.->|writeback candidates need approval| C
    F -.->|repair or rebuild| D
```

Use the package entry skill only for ambiguous, workflow-level Obsidian vault requests. If the operation is already clear, go straight to the operation-specific skill.
