//! HTML grammar for tree-sitter
//!
//! This crate provides the html language grammar for use with tree-sitter.

use tree_sitter_patched_arborium::Language;

unsafe extern "C" {
    fn tree_sitter_html() -> Language;
}

/// Returns the html tree-sitter language.
pub fn language() -> Language {
    unsafe { tree_sitter_html() }
}

/// The highlight query for html.
pub const HIGHLIGHTS_QUERY: &str = include_str!("../../../grammars/tree-sitter-html/queries/highlights.scm");

/// The injections query for html.
pub const INJECTIONS_QUERY: &str = include_str!("../../../grammars/tree-sitter-html/queries/injections.scm");

/// The locals query for html (empty - no locals available).
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
