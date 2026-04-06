# kb-health

对编译后的知识库做深度、报告式的健康检查。

## 检查内容

- 定义冲突
- 过时结论和过时 Q&A
- alias drift 和重复概念
- 稀疏页面
- 孤儿页面
- 弱连接
- 本地资源引用损坏
- provenance 不足

## 输出位置

写入：

`outputs/health/health-check-{date}.md`

## 报告内容

- 总分
- Completeness、Consistency、Connectivity、Freshness、Provenance 五个子分
- 关键问题
- warnings
- 可立即修复项与需人工判断项

## 和 kb-compile 的关系

`kb-compile` 只做轻量修补。

`kb-health` 才是专门的维护和体检流程。
