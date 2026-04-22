pub mod cli;
pub mod dev;
pub mod kb;
pub mod payload;
pub mod support;

pub mod audit_log {
    pub use crate::support::audit_log::*;
}

pub mod automation {
    pub use crate::kb::automation::*;
}

pub mod common {
    pub use crate::kb::index::{
        RecordByBasename, RecordByPath, normalize_identity, record_identities,
        registry_for_records, resolve_target, slugify_identity,
    };
    pub use crate::kb::markdown::{
        ALIAS_WIKILINK_RE, FRONTMATTER_RE, MARKDOWN_LINK_RE, MarkdownRecord, TABLE_LINE_RE,
        WIKILINK_RE, classify_markdown_path, extract_markdown_links, extract_wikilinks,
        iter_markdown_records, list_field, load_markdown, parse_frontmatter, parse_scalar,
        parse_simple_yaml, strip_link_alias,
    };
    pub use crate::support::{
        collapse_posix, json_string, normalize_path_string, now_iso, parse_datetime, parse_number,
        relative_posix, write_markdown,
    };
}

pub mod compile {
    pub use crate::kb::compile::*;
}

pub mod config {
    pub use crate::support::config::*;
}

pub mod episodes {
    pub use crate::kb::episodes::*;
}

pub mod governance {
    pub use crate::kb::governance::*;
}

pub mod graph {
    pub use crate::kb::graph::*;
}

pub mod guidance {
    pub use crate::kb::guidance::*;
}

pub mod health {
    pub use crate::kb::health::*;
}

pub mod ingest {
    pub use crate::kb::ingest::*;
}

pub mod init {
    pub use crate::kb::init::*;
}

pub mod layout {
    pub use crate::kb::layout::*;
}

pub mod query {
    pub use crate::kb::query::*;
}

pub mod render {
    pub use crate::kb::render::*;
}

pub mod review {
    pub use crate::kb::review::*;
}

pub mod skills {
    pub use crate::support::skills::*;
}
