//! TYPST grammar for tree-sitter
//!
//! This crate provides the typst language grammar for use with tree-sitter.

use tree_sitter_patched_arborium::Language;

unsafe extern "C" {
    fn tree_sitter_typst() -> Language;
}

/// Returns the typst tree-sitter language.
pub fn language() -> Language {
    unsafe { tree_sitter_typst() }
}

/// The highlight query for typst.
pub const HIGHLIGHTS_QUERY: &str = include_str!("../../../grammars/tree-sitter-typst/queries/typst/highlights.scm");

/// The injections query for typst.
pub const INJECTIONS_QUERY: &str = include_str!("../../../grammars/tree-sitter-typst/queries/typst/injections.scm");

/// The locals query for typst (empty - no locals available).
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
