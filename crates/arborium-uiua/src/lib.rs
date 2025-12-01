//! UIUA grammar for tree-sitter
//!
//! This crate provides the uiua language grammar for use with tree-sitter.

use tree_sitter_patched_arborium::Language;

unsafe extern "C" {
    fn tree_sitter_uiua() -> Language;
}

/// Returns the uiua tree-sitter language.
pub fn language() -> Language {
    unsafe { tree_sitter_uiua() }
}

/// The highlight query for uiua.
pub const HIGHLIGHTS_QUERY: &str = include_str!("../../../grammars/tree-sitter-uiua/queries/highlights.scm");

/// The injections query for uiua.
pub const INJECTIONS_QUERY: &str = include_str!("../../../grammars/tree-sitter-uiua/queries/injections.scm");

/// The locals query for uiua (empty - no locals available).
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
