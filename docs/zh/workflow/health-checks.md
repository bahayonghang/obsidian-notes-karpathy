# 健康检查

`kb-health` 是编译后 wiki 的维护和体检流程。

## 适用时机

- wiki 感觉越来越散
- 旧结论可能被新资料推翻
- 概念页定义不一致
- 需要每周或每月做一次质量基线

## 报告位置

健康报告写入：

`outputs/health/health-check-{date}.md`

## 评分维度

- Completeness
- Consistency
- Connectivity
- Freshness
- Provenance

## 常见结果

- 修复明显的交叉链接问题
- 标记 alias drift 或重复概念
- 找出缺失概念页
- 提醒哪些旧 Q&A 该重写或补注释
