# kb-compile

把不可变 raw captures 编译成可审校的 draft knowledge。

## 责任边界

`kb-compile` 会读取：

- `raw/human/**`
- `raw/agents/{role}/**`
- bootstrap 阶段的 `raw/*.md`
- 迁移期间的 legacy raw 路径

它只能写：

- `wiki/drafts/summaries/**`
- `wiki/drafts/concepts/**`
- `wiki/drafts/entities/**`
- `wiki/drafts/indices/**`
- `wiki/log.md`

不能直接写 `wiki/live/`。

## 论文与证据规则

`raw/**/papers/*.pdf` 下的论文 PDF 是 `paper-workbench` 的路由例外，不属于普通编译输入。`kb-compile` 只能把它们标记出来、延后处理，或在 companion skill 缺失时返回跳过信息。

在生成 drafts 之前，编译器还应该整理 `source_hash`、`source_mtime`、`last_verified_at`、`possibly_outdated` 这类证据元数据。

## 主要输出

- 带 `compiled_from`、`capture_sources`、`review_state`、`review_score` 的 summary drafts
- 带明确证据链的 concept / entity drafts
- 草稿索引
- `alias_candidates` 和 `duplicate_candidates`
- `wiki/log.md` 里的 `ingest` 记录

drafts 的目标是方便 review 判断，而不是提前做最终润色。
