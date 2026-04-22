mod identity;
mod link_integrity;

pub use identity::duplicate_identity_issues;
pub use link_integrity::{alias_wikilink_table_issues, broken_wikilink_issues, orphan_page_issues};
