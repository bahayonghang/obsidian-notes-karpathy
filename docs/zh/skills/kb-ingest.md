# kb-ingest

当 review-gated 支撑层已经存在，但 `raw/_manifest.yaml` 缺失或过期时，使用 `kb-ingest`。

## 它做什么

- 扫描 `raw/**`
- 把来源登记到 `raw/_manifest.yaml`
- 明确标出 paper PDF 等 deferred 输入
- 保持 `raw/**` 不可变
