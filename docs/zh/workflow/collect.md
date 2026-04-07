# 收集资料

收集阶段是人类主导的部分。

## 目标

把有价值的来源资料以 markdown 形式放进 `raw/`，并保留足够元数据供后续编译。

## 推荐方式

### Obsidian Web Clipper

适合文章和网页。

建议保留：

- `title`
- `source`
- `author`
- `date`
- `type`
- `tags`
- `clipped_at`

### 手动创建 markdown

保持同样的元数据结构，但不要在 raw 中加入编译状态字段。

### PDF 论文

如果原始资料本身就是论文 PDF，就把它放到 `raw/papers/`。

- 如果你已经知道 paper handle，可以在旁边加一个 `paper-name.source.md` sidecar，写 `paper_id` 或 `source`
- 只有在能确定解析出 paper handle 时，编译才优先用 `alphaxiv-paper-lookup`
- 否则降级到 `pdf`
- 如果两个 companion skill 都没装，就明确提示安装，而不是假装编译成功

## Raw 层规则

`raw/` 对编译器来说是 immutable 层。

这意味着：

- 来源元数据放在 raw
- 编译元数据放在摘要页
- 让 `kb-compile` 维护 wiki，而不是回写源文件
