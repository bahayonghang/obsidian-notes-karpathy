mod audit;
mod context;
mod engine;
mod rules;
mod signals;

pub use audit::audit_vault_mechanics;
pub use engine::{
    briefing_staleness_issues, confidence_decay_due_issues, episodic_backlog_issues,
    graph_gap_issues, memory_knowledge_mix_issues, missing_confidence_metadata_issues,
    procedural_promotion_gap_issues, review_backlog_issues, stale_qa_issues,
    supersession_gap_issues, volatile_page_stale_issues, weak_live_sources_issues,
    writeback_backlog_issues,
};
