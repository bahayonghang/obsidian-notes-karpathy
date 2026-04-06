# 编译 Wiki

`kb-compile` 是知识库的编译阶段。

## 输入

- `raw/` 下的 markdown 资料
- `AGENTS.md` 与 `CLAUDE.md` 中定义的本地契约

## 输出

- 摘要页
- 概念页
- 重建后的索引
- 更新后的 `wiki/index.md`
- 追加后的 `wiki/log.md`

## 关键规则

编译会读取 raw，但不会修改 raw。
