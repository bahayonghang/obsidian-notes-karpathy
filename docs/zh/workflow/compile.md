# 编译 Wiki

`kb-compile` 是知识库的编译阶段。

## 输入

- `raw/human/**` 下的 markdown 资料
- `raw/agents/{role}/**` 下的 markdown 资料
- bootstrap 阶段的 `raw/*.md`
- `raw/**/papers/*.pdf` 下的论文 PDF，但它们是 `paper-workbench` 路由例外，不属于普通编译输入
- `AGENTS.md` 与 `CLAUDE.md` 中定义的本地契约

## 输出

- `wiki/drafts/summaries/**`
- `wiki/drafts/concepts/**`
- `wiki/drafts/entities/**`
- `wiki/drafts/indices/**`
- 追加后的 `wiki/log.md`

## 关键规则

编译会读取 raw，但不会修改 raw，也不能直接提升到 `wiki/live/`。

编译阶段还应该显式产出 alias / duplicate 候选，以及 `source_hash`、`last_verified_at`、`possibly_outdated` 这类证据元数据，供 `kb-review` 判断。

如果 `paper-workbench` 没安装，只跳过受影响的 `raw/**/papers/*.pdf`，并返回安装提示。

PDF 旁边允许放一个可选的 `paper-name.source.md` sidecar，用来提供 `paper_id` 或 `source` 元数据，但它不算第二份 raw source，也不会改变路由规则。
