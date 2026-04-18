# Karpathy 工作流

这个工作流把知识库看成有源文件、编译产物和维护流程的代码库。

## 心智模型

| 软件工程 | 知识库 |
|----------|--------|
| source code | `raw/` |
| 来源登记表 | `raw/_manifest.yaml` |
| compiler | `kb-compile` |
| build artifacts | `wiki/` |
| logs and reports | `outputs/` |
| lint / maintenance | `kb-review`（`maintenance` mode） |

## 直接后果

- 人负责策展资料
- 工作流会先把来源登记到 manifest，再进入 compile
- LLM 负责维护结构和交叉链接
- 有价值的 Q&A 会落盘
- 高价值输出会生成 writeback 工作，重新进入 draft / review 闭环
- 可以系统性检查漂移、冲突和连接质量
- creator-facing prose 可以建立在批准知识层之上，而不需要打破真相边界

## Compile 姿态

这个 bundle 的默认 compile 不是“写一个泛泛摘要”。

而是：

`浓缩 -> 质疑 -> 对标`

- `浓缩`：只保留核心结论和关键证据
- `质疑`：显式记录前提、边界条件和失败场景
- `对标`：寻找跨域迁移价值，以及 procedure / hub 候选

## 默认检索方式

先走 markdown-first：

1. 读 `wiki/index.md`
2. 读派生索引
3. 把 topic pages 当默认 浏览层
4. 用 Backlinks、unlinked mentions 和 Properties 搜索
5. 跟 wikilink
6. 必要时做文件搜索

只有当 vault 规模明显超出这个模型时，再考虑更重的搜索基础设施。下一步通常应先是 DuckDB markdown 解析和全文检索，而不是立刻上向量检索。


