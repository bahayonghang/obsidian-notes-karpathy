# 编译 Wiki

`kb-compile` 是知识库的编译阶段。

## 输入

- `raw/` 下的 markdown 资料
- `raw/papers/` 下的 PDF 论文，处理时优先 `alphaxiv-paper-lookup`，再降级到 `pdf`
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
