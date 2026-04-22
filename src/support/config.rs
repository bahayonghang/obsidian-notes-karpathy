//! 项目范围的阈值与魔数。
//!
//! 把散落在各模块的裸字面量集中到这里，便于调整与文档化。
//! 任何新的决策阈值、时间窗口或策略常量应在此添加，并在消费点引用。

/// 审核得分 >= 此阈值自动批准。
pub const REVIEW_APPROVE_SCORE: f64 = 0.85;

/// 审核得分 < 此阈值自动驳回。
pub const REVIEW_REJECT_SCORE: f64 = 0.60;

/// 有别名候选时，低于此得分不再自动批准，转人工。
pub const REVIEW_ALIAS_JUDGMENT_SCORE: f64 = 0.90;

/// raw 文件 mtime 距今超过此天数即视为"可能过期"提示。
pub const STALENESS_HINT_DAYS: i64 = 730;

/// 不同波动性档位的审核到期阈值（天）。
///
/// key 来自 frontmatter `domain_volatility` 字段，通常是 `high` / `medium` / `low`。
pub const VOLATILITY_THRESHOLDS: &[(&str, i64)] = &[("high", 90), ("medium", 180), ("low", 365)];
