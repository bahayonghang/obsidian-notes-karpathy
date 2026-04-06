# kb-health

对编译后的知识库做深度、报告式的健康检查。

## 审计范围

健康检查主要覆盖：

- `wiki/`
- `outputs/qa/`
- `outputs/health/`

只有在检查溯源或新鲜度时，才回头 spot check `raw/`。

## 它会检查什么

- 定义冲突或已被新证据覆盖的旧结论
- 过时 Q&A、过时概念页、过时实体页
- alias drift 和重复概念或实体
- 稀疏页面与明显缺失的主题
- 孤儿页面与弱连接
- 本地资源缺失
- provenance 不足
- 在 Obsidian 里渲染有问题的表格或 wikilink
- 知识库是否已经需要升级检索层

## 报告输出

报告写入 `outputs/health/health-check-{date}.md`。

报告应包含：

- 100 分制总分
- Completeness、Consistency、Connectivity、Freshness、Provenance 五个子分
- 关键问题
- warnings
- 后续问题或资料缺口建议
- 检索升级建议
- 明确区分可立即自动修复项和需要人工判断的事项

## 和 kb-compile 的关系

`kb-compile` 可以在编译后顺手修掉一些明显的机械问题。

`kb-health` 才负责更深层的维护工作，包括漂移、矛盾、溯源缺口，以及长期结构问题。
