# kb-compile

把 immutable raw 编译成可维护的 wiki。

## 它做什么

- 发现新增或变更的 raw 资料
- 生成或更新摘要页
- 创建或更新概念页
- 在需要时扩展 aliases 和冲突标记
- 重建索引和 log 入口
- 做轻量级 post-compile 检查

## 它不会做什么

- 不回写 raw 源文件
- 不承担深度健康检查

## 增量模型

`kb-compile` 通过摘要页里保存的 `source_hash` 和 `source_mtime` 来判断某个 raw 是否需要重新编译。
