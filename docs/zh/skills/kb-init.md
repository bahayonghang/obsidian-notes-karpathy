# kb-init

创建或修复 Karpathy 式 Vault 契约。

## 会创建或修复什么

```text
raw/{articles,papers,podcasts,assets}
wiki/{concepts,summaries,indices}
wiki/index.md
wiki/log.md
outputs/{qa,health,reports,slides,charts,content/{articles,threads,talks}}
AGENTS.md
CLAUDE.md
```

可选扩展：

- `raw/repos/`
- `wiki/entities/`
- `wiki/indices/ENTITIES.md`

## 它会先决定什么

- 目标 Vault 根目录在哪里
- 当前是全新初始化还是修复
- 是否现在就启用 publish-mode 目录
- 是否现在就启用 repo 摄入层或实体层

## 关键保证

- 明确 `raw/` 是对编译器不可变的输入层
- `AGENTS.md` 被当作必需的本地契约，`CLAUDE.md` 被当作生成出来的 companion
- `AGENTS.md` 和 `CLAUDE.md` 在同时存在时对同一个文件模型保持一致
- 在其他技能运行前先补齐零状态索引页和日志页
- 预置可执行的概念页样例和 clipper 模板
- 缺失支撑文件会被视为修复任务，而不是硬失败

## 它会写入什么

- 本地 guidance 契约
- `INDEX.md`、`CONCEPTS.md`、`SOURCES.md`、`RECENT.md` 等起始索引页
- `wiki/index.md` 和 `wiki/log.md`
- 概念页样例，以及在启用实体层时写入实体页样例
- raw clipper 模板

## 推荐交接

1. 在 `raw/` 下放入 5 到 10 条真实资料
2. 运行 `kb-compile`
3. 用 `kb-query` 提一个实质性问题
4. 如果 Vault 需要对外输出，就先产出一个交付物
5. 跑一次 `kb-health` 建立维护基线
