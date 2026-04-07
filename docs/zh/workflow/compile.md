# 编译 Wiki

`kb-compile` 是知识库的编译阶段。

## 输入

- `raw/` 下的 markdown 资料
- `raw/papers/` 下的 PDF 论文，先做确定性 handle 解析，再决定走 `alphaxiv-paper-lookup` 还是 `pdf`
- `AGENTS.md` 与 `CLAUDE.md` 中定义的本地契约

## 输出

- 摘要页
- 概念页
- 重建后的索引
- 更新后的 `wiki/index.md`
- 追加后的 `wiki/log.md`

## 关键规则

编译会读取 raw，但不会修改 raw。

如果两个论文 companion skill 都没安装，只跳过受影响的 PDF，并返回安装提示。

PDF 旁边允许放一个可选的 `paper-name.source.md` sidecar，用来提供 `paper_id` 或 `source` 元数据，但它不算第二份 raw source。
