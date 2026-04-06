# 技能概览

这个包由一个包级入口技能和四个操作型技能组成。

## 为什么这样拆分

工作流天然分成几步：

- 识别当前处于哪个阶段
- 初始化 vault 契约
- 把 immutable raw 编译成 wiki
- 查询 wiki 并沉淀结果
- 做深度健康检查

拆分后更容易触发，也更容易维护。

## 技能列表

| 技能 | 角色 | 适用场景 |
|------|------|---------|
| `obsidian-notes-karpathy` | 包级入口与路由 | 用户讨论整个工作流 |
| `kb-init` | 一次性初始化 | 标准目录和 schema 还不存在 |
| `kb-compile` | 编译与增量更新 | raw 中有新资料或变更资料 |
| `kb-query` | 搜索、问答、输出 | 用户要从 wiki 提取结论或生成交付物 |
| `kb-health` | 深度体检与维护 | 用户要做周期性健康检查 |

## 共享资源

包内还包含：

- `references/`：schema、summary、concept、Q&A、publish、health report 模板
- `evals/evals.json`：包级回归提示词
- `evals/fixtures/`：fresh / compiled / drift 三类 fixture vault

## 设计契约

- `raw/` 是 immutable 层
- `wiki/` 是编译产物
- `outputs/qa/` 默认沉淀高价值问答
- `outputs/content/` 保存对外内容草稿
- `wiki/index.md` 是内容入口
- `wiki/log.md` 是时间线
