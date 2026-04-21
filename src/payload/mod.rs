//! Typed payload 定义。
//!
//! 阶段 2 逐步用 struct/enum 替换业务函数直接返回的 `serde_json::Value`，
//! 同时保持与现有 JSON 输出字节级/语义等价（由 `tests/payload_compat.rs` 护栏）。
//!
//! 每个 struct 必须 `#[serde(rename_all = "snake_case")]`，匹配现有字段命名。

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// `onkb doctor` 的返回结构。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DoctorPayload {
    pub version: String,
    pub binary_path: String,
    pub embedded_assets_detected: bool,
    pub skill_bundle_available: bool,
    pub runtime_mode: String,
    pub needs_python: bool,
    pub python_detected: Option<Value>,
    pub missing_steps: Vec<String>,
}

/// `onkb skill install` 的单个目标结果。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillInstallTarget {
    pub target_dir: String,
    pub installed: Vec<String>,
    pub skipped: Vec<String>,
}

/// `onkb skill install` 的完整返回。
///
/// 字段随 `--claude` / `--codex` 开关决定是否出现，用 `skip_serializing_if` 处理。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SkillInstallPayload {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub claude: Option<SkillInstallTarget>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub codex: Option<SkillInstallTarget>,
}

/// 通用 counts 映射（`new`/`changed`/`unchanged`/`pending` 等）。
pub type Counts = BTreeMap<String, i64>;

/// `onkb review graph` 单个节点。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphNode {
    pub id: String,
    pub path: String,
    pub kind: String,
    pub title: String,
    pub visibility_scope: String,
    pub topic_hub: Value,
    pub source_count: usize,
    pub related_count: usize,
    pub question_link_count: usize,
    pub has_relationship_notes: bool,
    pub graph_required: bool,
    pub confidence_score: Value,
    pub confidence_band: Value,
    pub approved_at: Value,
    pub last_confirmed_at: Value,
}

/// `onkb review graph` 单条边。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphEdge {
    pub source: String,
    pub relation: String,
    pub target: String,
    pub source_kind: String,
    pub source_visibility_scope: String,
}

/// `onkb review graph` 的返回结构。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphSnapshot {
    pub vault_root: String,
    pub generated_at: String,
    pub candidate_policy: String,
    pub node_count: usize,
    pub edge_count: usize,
    pub edge_type_counts: Counts,
    pub nodes: Vec<GraphNode>,
    pub edges: Vec<GraphEdge>,
}

/// `onkb query scope` 里的隐私越界记录。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScopeLeak {
    pub path: String,
    pub visibility_scope: String,
    pub reason: String,
}

/// `onkb query scope` 里的敏感等级候选。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SensitivityCandidate {
    pub path: String,
    pub sensitivity_level: String,
    pub candidate_kind: String,
}

/// `onkb query scope` 的返回结构。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryScope {
    pub vault_root: String,
    pub layout_family: String,
    pub profile: String,
    pub included_paths: Vec<String>,
    pub candidate_paths: Vec<String>,
    pub candidate_policy: String,
    pub excluded_paths: Vec<String>,
    pub excluded_prefixes: Vec<String>,
    pub scope_leaks: Vec<ScopeLeak>,
    pub sensitivity_candidates: Vec<SensitivityCandidate>,
}

/// `onkb query rank` 单条结果。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RankedEntry {
    pub path: String,
    pub kind: String,
    pub candidate_kind: String,
    pub truth_boundary: String,
    pub score: i64,
    pub lexical_overlap: i64,
    pub metadata_score: i64,
    pub graph_score: i64,
    pub title: String,
}

/// `onkb query rank` 的返回结构。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryRanked {
    pub vault_root: String,
    pub query: String,
    pub candidate_policy: String,
    pub included_count: usize,
    pub candidate_count: usize,
    pub ranked_paths: Vec<RankedEntry>,
}

/// `onkb render` 的返回结构。
///
/// `package_root` 仅在 `web` 模式且 `--write` 开启时填充。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RenderResult {
    pub vault_root: String,
    pub mode: String,
    pub requested_source_paths: Vec<String>,
    pub source_paths: Vec<String>,
    pub rejected_source_paths: Vec<String>,
    pub source_live_pages: Vec<String>,
    pub output_path: String,
    pub title: String,
    pub followup_route: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_root: Option<String>,
}

/// `onkb review queue` 单条 item。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReviewItem {
    pub path: String,
    pub kind: String,
    pub review_state: String,
    pub review_score: Option<f64>,
    pub blocking_flags: Vec<String>,
    pub alias_candidates: Vec<String>,
    pub duplicate_candidates: Vec<String>,
    pub decision: String,
    pub reason: String,
    pub pending: bool,
    pub live_path: String,
    pub live_exists: bool,
    pub review_record_path: String,
}

/// `onkb review queue` 的决策计数。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ReviewCounts {
    pub pending: i64,
    pub approve: i64,
    pub reject: i64,
    #[serde(rename = "needs-human")]
    pub needs_human: i64,
}

/// `onkb review queue` 的返回结构。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReviewQueue {
    pub vault_root: String,
    pub counts: ReviewCounts,
    pub state_counts: Counts,
    pub items: Vec<ReviewItem>,
}

/// `onkb review lint` 的返回结构。
///
/// `issues` 暂时保留 `Vec<Value>`，每条 issue 由对应规则构造；
/// 阶段 4 拆 `rules/` 时再换成 `HealthIssue` internally-tagged enum。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthAudit {
    pub vault_root: String,
    pub layout_family: String,
    pub issue_counts: Counts,
    pub issues: Vec<Value>,
}

/// `onkb ingest scan`/`sync` 的返回结构。
///
/// `items` 保留 `Vec<Value>`（字段极多且随 source_class 变），阶段 5 拆 `ingest/` 时再拆出 `IngestItem`。
/// `written_manifest` 仅在 `sync --write` 时填充。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IngestDelta {
    pub vault_root: String,
    pub profile: String,
    pub manifest_path: String,
    pub manifest_present: bool,
    pub manifest_status: String,
    pub bootstrap_manifest_required: bool,
    pub needs_ingest: bool,
    pub counts: Counts,
    pub items: Vec<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub written_manifest: Option<String>,
}

/// `onkb compile scan` 的返回结构。
///
/// `items` 与 `companion_skills` 保留 `Value`，阶段 5 拆 `compile/` 时再细化。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompileDelta {
    pub vault_root: String,
    pub layout_family: String,
    pub counts: Counts,
    pub ingest_counts: Counts,
    pub companion_skills: Value,
    pub items: Vec<Value>,
}

/// `onkb compile build --write` 的返回结构。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompileBuildResult {
    pub vault_root: String,
    pub package_count: usize,
    pub packages: Vec<Value>,
    pub written_paths: Vec<String>,
}
