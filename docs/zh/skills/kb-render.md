# kb-render

`kb-render` 现在是已批准知识的确定性派生产物入口。

如果用户说的是 `output/reports`，先判断他要的是治理报告还是确定性渲染报告。前者仍归 `kb-review`，后者才归这里。

## 支持模式

- `slides`
- `charts`
- `canvas`
- `report`

## 规则

渲染产物属于下游派生物，不会自动变成已批准知识层。Markdown 产物应保留 `source_live_pages` 与 `followup_route`。静态 web 导出属于 `kb-query`，不属于 `kb-render`。

