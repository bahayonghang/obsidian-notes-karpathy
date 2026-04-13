# 边界

这套 bundle 能否长期稳定，核心不在“工具多不多”，而在 surface 有没有分开。

## 已落地边界

- `raw/` 是证据 intake，后续阶段不应改写。
- `wiki/drafts/` 是可审校知识，不是真相层。
- `wiki/live/` 是批准后的长期知识脑。
- `wiki/live/procedures/` 是批准后的流程记忆。
- `wiki/briefings/` 只能从 live 构建。
- `outputs/reviews/` 是 promotion 决策账本。
- `outputs/episodes/` 是 episodic memory，不是批准真相。
- `outputs/audit/operations.jsonl` 是机器可读审计面。
- `wiki/live/indices/` 下的治理索引属于批准层，必须以 approved live pages 为基础。

## 协作层与知识层

`MEMORY.md` 是协作记忆层，用来放：

- 偏好
- 协作规则
- 编辑优先级
- 当前关注主题

它不是放专题结论的地方。带来源的知识判断应该进入 `wiki/drafts/` 或 `wiki/live/`。

## Archived outputs 与 approved truth

`outputs/qa/` 与 `outputs/content/` 可以保存高价值产物，但它们不会自动变成 approved truth。

它们可以：

- 提供维护信号
- 暴露 `writeback_candidates`
- 记录 `followup_route`
- 携带 `crystallized_from_episode`
- 为 health 与 backlog 视图提供输入

但长期知识仍然必须重新走 draft -> review -> live。

## 为什么这条线重要

如果边界不清，系统会很快出现三个问题：

- 任务状态和知识结论混在一起
- query 时拉到错误上下文
- health 无法区分正常协作痕迹和真正的知识漂移

## Planned / evolving checks

health 会继续增强这几类检查：

- 协作记忆混进 approved knowledge
- approved knowledge 倒灌进协作记忆
- 回写 backlog 长期堆积不处理
- 把 archived outputs 误当成 approved live 治理真相
