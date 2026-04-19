# 健康检查

`kb-review` 的 `维护模式` 是围绕已批准知识层及其审校生态的长期维护流程，默认先出报告。

适用场景不是“立刻给当前草稿过审”或“顺手重建这一次过期的角色简报”，而是已批准知识层已经出现持续的维护信号。

## 适用时机

- wiki 感觉越来越散
- 旧结论可能被新资料推翻
- 概念页定义不一致
- 需要每周或每月做一次质量基线
- `QUESTIONS.md`、`GAPS.md`、`ALIASES.md` 这类治理视图需要刷新
- 归档 Q&A 或发布产物可能已经积累 writeback backlog
- archived outputs 可能已经出现 reuse drift、过时结论或 scope leak
- 需要检查协作记忆是否渗入已批准知识
- 需要检查 creator-facing 指导面是否开始漂移

## 报告位置

健康报告写入：

`outputs/health/health-check-{date}.md`

## 评分维度

- 完整性（Completeness）
- 一致性（Consistency）
- 连通性（Connectivity）
- 时效性（Freshness）
- 溯源性（Provenance）

## 常见结果

- 修复明显的交叉链接问题
- 标记 alias drift 或重复概念
- 标记过期的归档 Q&A 或 publish 产物
- 标记 `CLAUDE.md`、`MEMORY.md`、`_style-guide.md` 与 briefing 之间的 creator consistency drift
- 标记 archived creator outputs 的 reuse gap
- 标记长期没有进入下游输出面的 underused approved sources
- 找出缺失概念页
- 找出 writeback backlog
- 找出 archive hygiene 问题，但不把 archive 当成真相层
- 找出协作记忆渗入已批准知识的地方
- 在目标明确时刷新治理视图
- 提醒哪些旧 Q&A 该重写或补注释


