//! AWK grammar for tree-sitter
//!
//! This crate provides the awk language grammar for use with tree-sitter.

use tree_sitter_patched_arborium::Language;

unsafe extern "C" {
    fn tree_sitter_awk() -> Language;
}

/// Returns the awk tree-sitter language.
pub fn language() -> Language {
    unsafe { tree_sitter_awk() }
}

/// The highlight query for awk.
pub const HIGHLIGHTS_QUERY: &str = include_str!("../../../grammars/tree-sitter-awk/queries/highlights.scm");

/// The injections query for awk (empty - no injections available).
pub const INJECTIONS_QUERY: &str = "";

/// The locals query for awk (empty - no locals available).
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
