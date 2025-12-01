//! TEXTPROTO grammar for tree-sitter
//!
//! This crate provides the textproto language grammar for use with tree-sitter.

use tree_sitter_patched_arborium::Language;

unsafe extern "C" {
    fn tree_sitter_textproto() -> Language;
}

/// Returns the textproto tree-sitter language.
pub fn language() -> Language {
    unsafe { tree_sitter_textproto() }
}

/// The highlight query for textproto.
pub const HIGHLIGHTS_QUERY: &str = include_str!("../../../grammars/tree-sitter-textproto/queries/highlights.scm");

/// The injections query for textproto (empty - no injections available).
pub const INJECTIONS_QUERY: &str = "";

/// The locals query for textproto (empty - no locals available).
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
