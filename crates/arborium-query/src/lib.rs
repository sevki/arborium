//! QUERY grammar for tree-sitter
//!
//! This crate provides the query language grammar for use with tree-sitter.

use tree_sitter_patched_arborium::Language;

unsafe extern "C" {
    fn tree_sitter_query() -> Language;
}

/// Returns the query tree-sitter language.
pub fn language() -> Language {
    unsafe { tree_sitter_query() }
}

/// The highlight query for query.
pub const HIGHLIGHTS_QUERY: &str = include_str!("../../../grammars/tree-sitter-query/queries/query/highlights.scm");

/// The injections query for query.
pub const INJECTIONS_QUERY: &str = include_str!("../../../grammars/tree-sitter-query/queries/query/injections.scm");

/// The locals query for query (empty - no locals available).
pub const LOCALS_QUERY: &str = "";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_language() {
        let lang = language();
        assert!(lang.version() > 0);
    }
}
