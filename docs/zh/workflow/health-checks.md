# 健康检查

维护知识库 wiki 的质量和完整性。

## 概览

健康检查（也称为"lint"）是自动质量评估，识别编译 wiki 中的问题。它们在每次编译后自动运行，或可以独立触发。

## 何时运行

- **自动**：每次 `kb-compile` 运行后
- **按需**："health check"、"lint"、"检查知识库"
- **定期**：每周或每月捕获漂移

## 检查什么

### 1. 一致性检查

**目的**：确保 wiki 不包含矛盾或过时的信息。

**检查**：
- **矛盾陈述**：概念 A 说 X 为真，概念 B 说 X 为假
- **过时信息**：早于阈值（如 2 年）的资料
- **断链 wikilinks**：指向不存在文件的链接

**问题示例**：

```
> [!danger] 检测到矛盾
> [[wiki/concepts/batch-normalization]] 声称批归一化对训练至关重要
> [[wiki/concepts/group-normalization]] 声称组归一化优于批归一化
> 两者引用不同来源 —— 考虑调和

> [!warning] 过时资料
> [[wiki/concepts/rnn]] 主要引用 2015-2017 年的资料
> 考虑添加近期发展（2020+）

> [!danger] 断链
> [[wiki/concepts/attention]] 链接到 [[wiki/concepts/multi-head-attention]]
> 但目标文件不存在
```

### 2. 缺失数据检测

**目的**：识别不完整或稀疏的内容。

**检查**：
- **稀疏概念**：少于 100 字的文章
- **不完整 frontmatter**：缺少标签或元数据的原始资料
- **缺失关键概念**：没有 `key_concepts` 字段的摘要

**问题示例**：

```
> [!warning] 稀疏概念
> [[wiki/concepts/layer-normalization]] 只有 67 字
> 考虑用更多资料扩展

> [!warning] 缺失标签
> [[raw/2026-04-03-some-article]] 没有标签
> 添加 2-5 个标签以提高可搜索性

> [!warning] 缺失关键概念
> [[wiki/summaries/some-source]] 的 key_concepts 为空
> 更新摘要以识别关键概念
```

### 3. 连接发现

**目的**：找到改进 wiki 互联性的机会。

**检查**：
- **未建立连接**：共享主题但未链接的概念
- **缺失概念文章**：在文本中提到但没有自己文章的概念
- **建议 wikilinks**：应添加 `[[wikilinks]]` 的地方

**问题示例**：

```
> [!tip] 建议连接
> [[wiki/concepts/attention]] 和 [[wiki/concepts/self-attention]] 
> 有 80% 关键词重叠但未链接
> 考虑添加 wikilink

> [!tip] 缺失概念文章
> "Positional encoding"在 5 篇文章中提到但没有概念页
> 考虑创建 [[wiki/concepts/positional-encoding]]
```

### 4. 孤立检测

**目的**：找到未连接到 wiki 其余部分的内容。

**检查**：
- **孤立资料**：没有对应摘要的原始资料
- **孤立概念**：没有传入链接的概念文章
- **断开摘要**：未链接到任何概念的摘要

**问题示例**：

```
> [!warning] 孤立资料
> [[raw/2026-04-01-older-article]] 没有摘要
> 运行 kb-compile 生成

> [!warning] 孤立概念
> [[wiki/concepts/obscure-technique]] 没有来自其他文章的传入链接
> 要么删除它，要么从相关概念添加连接

> [!warning] 断开摘要
> [[wiki/summaries/some-source]] 未链接到任何概念
> 摘要可能太通用 —— 识别关键概念
```

## 健康报告

检查完成后，报告写入 `outputs/reports/health-check-{date}.md`：

```markdown
---
title: "健康检查报告"
date: 2026-04-03T15:30:00
---

# 知识库健康检查

## 摘要
- 整体健康状态：良好
- 资料：45（3 份新，0 份孤立）
- 概念：67（2 份稀疏，54 份良好连接）
- 断链：1

## 发现的问题

### 关键
> [!danger] 断链
> [[wiki/concepts/attention]] 链接到 [[wiki/concepts/multi-head-attention]]
> 目标文件不存在

### 警告
> [!warning] 稀疏概念
> [[wiki/concepts/layer-normalization]] 只有 67 字

### 建议
> [!tip] 建议连接
> [[wiki/concepts/attention]] 和 [[wiki/concepts/self-attention]] 共享关键词

## 建议操作
1. 创建 [[wiki/concepts/multi-head-attention]] 或修复 [[wiki/concepts/attention]] 中的链接
2. 用更多资料扩展 [[wiki/concepts/layer-normalization]]
3. 在注意力和自注意力文章之间添加 wikilink
```

## 健康评级

### 良好

- 大多数概念良好连接（>80% 有传入链接）
- 无断链 wikilinks
- 很少或没有稀疏概念
- 所有资料都有摘要

### 需要注意

- 一些孤立内容
- 几个断链
- 若干稀疏概念
- 缺失一些连接

### 较差

- 许多断链
- 大量孤立内容
- 矛盾陈述
- 过时信息未标记

## 修复问题

### 关键问题（立即修复）

- **断链**：创建缺失文件或更新链接
- **矛盾**：调和冲突陈述，引用资料

### 警告（尽快处理）

- **稀疏概念**：添加更多资料，重新编译
- **孤立资料**：运行编译生成摘要
- **缺失 frontmatter**：手动或通过重新编译补充资料文件

### 建议（考虑未来改进）

- **建议连接**：在适当位置添加 wikilinks
- **缺失概念文章**：为频繁提到的概念创建文章
- **标签改进**：为资料添加缺失标签

## 自动 vs 手动修复

### LLM 可以自动修复

- 创建缺失概念文章（如果有足够资料）
- 在提到概念的地方添加 wikilinks
- 更新索引和交叉引用
- 补充不完整资料的 frontmatter

### 你应该手动修复

- 矛盾陈述（需要判断）
- 过时信息（需要领域知识）
- 结构重组（需要规划）

## 最佳实践

### 1. 定期查看报告

不要忽略健康报告。每周检查它们并及时处理关键问题。

### 2. 添加资料后重新编译

许多问题（稀疏概念、孤立）在添加更多资料并重新编译后自动解决。

### 3. 跟踪趋势

比较健康报告随时间变化：
- wiki 是否更连接？
- 断链是否减少？
- 概念质量是否提高？

### 4. 设置质量阈值

定义你自己的标准：
- 最小概念长度（如 100 字）
- 标记为过时之前的最大年限
- 每个概念的最小传入链接数

## 下一步

- [**编译 Wiki**](/zh/workflow/compile) — 运行编译自动修复问题
- [**查询与输出**](/zh/workflow/query) — 从健康 wiki 中提取价值
- [**kb-compile 技能**](/zh/skills/kb-compile) — 详细编译参考
