# 归档模型

这个 bundle 里的 `archive` 有两层含义。

## 两类归档面

1. **source retention archive**
2. **artifact archive**

它们都持久存在，但都不等于 approved truth。

## Source retention archive

source retention archive 指的是：

- `raw/**`
- `raw/_manifest.yaml`

意思是原始来源被保留，并且有登记表。

它**不等于**：

- 把来源移到 `raw/09-archive/`
- compile 后删除 source files
- 把 raw captures 当作 query-time truth

这个仓库故意把 raw 留在原位。这里说的 archive，是 retention，不是 relocation。

## Artifact archive

artifact archive 指下游输出层：

- `outputs/qa/**`
- `outputs/content/**`
- `outputs/episodes/**`
- `outputs/reviews/**`
- `outputs/health/**`
- `outputs/web/**`

这些产物之所以要持久保存，是因为工作流要复用、维护、回写，而不是把它们当成一次性聊天残留。

## 真相、复用、维护边界

| Surface | 主要作用 | 是否真相层 | 是否可复用 | 是否会触发维护 |
| --- | --- | --- | --- | --- |
| `raw/**` + `raw/_manifest.yaml` | 保留证据 | 否 | 作为证据可复用 | 会触发 ingest / maintenance |
| `wiki/live/**` | 已批准知识 | 是 | 是，且优先 | 会触发 review / maintenance |
| `outputs/**` | artifact archive | 否 | 是，作为工作面 | 会触发 archive hygiene 与 writeback |

核心规则很简单：

- `wiki/live/**` 仍然是唯一 approved truth layer。
- archived outputs 可以复用。
- archived outputs 可以生成 `writeback_candidates`。
- archived outputs 可以形成 backlog 和 drift 信号。
- archived outputs 不会自动升格成真相层。

## 默认复用顺序

当用户要新答案或新对外内容时，安全的复用顺序是：

1. approved live pages
2. live indices 和 briefings
3. 历史 archived Q&A
4. 已经正确复用批准知识的 archived publish artifacts

这样 archive 才是“有用的工作面”，而不是“比 live 更权威的捷径”。

## 为什么这里不用 `raw/09-archive/`

Karpathy 原文强调的是 immutable raw sources 和持续维护的 wiki，并没有要求把处理后的原始来源物理迁移。

所以这个 bundle 选择：

- 保持 raw retention 稳定
- 让 source registration 显式可见
- 把下游 outputs 和 truth boundary 分开

如果以后真要支持 `raw/09-archive/` 这种物理归档，那应该作为单独的架构变更处理，配套新的迁移和生命周期规则。
