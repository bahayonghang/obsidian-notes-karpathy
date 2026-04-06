# 目录结构

这个 vault 明确分成 immutable sources、compiled knowledge 和 generated outputs 三层。

```text
vault/
├── raw/
│   ├── articles/
│   ├── papers/
│   ├── podcasts/
│   └── assets/
├── wiki/
│   ├── concepts/
│   ├── summaries/
│   ├── indices/
│   ├── index.md
│   └── log.md
├── outputs/
│   ├── qa/
│   ├── health/
│   ├── reports/
│   ├── slides/
│   └── charts/
├── AGENTS.md
└── CLAUDE.md
```

## 含义

- `raw/`：人类整理的不可变原始资料
- `wiki/`：LLM 维护的编译产物
- `outputs/`：生成内容和持久化研究结果

## 重要后果

不要把编译状态写回 raw。应该写到摘要页 frontmatter 或健康报告里。
