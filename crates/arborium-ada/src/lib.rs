//! ADA grammar for tree-sitter
//!
//! This crate provides the ada language grammar for use with tree-sitter.

use tree_sitter_patched_arborium::Language;

unsafe extern "C" {
    fn tree_sitter_ada() -> Language;
}

/// Returns the ada tree-sitter language.
pub fn language() -> Language {
    unsafe { tree_sitter_ada() }
}

/// The highlight query for ada.
pub const HIGHLIGHTS_QUERY: &str = include_str!("../../../grammars/tree-sitter-ada/queries/highlights.scm");

/// The injections query for ada (empty - no injections available).
pub const INJECTIONS_QUERY: &str = "";

/// The locals query for ada.
pub const LOCALS_QUERY: &str = include_str!("../../../grammars/tree-sitter-ada/queries/locals.scm");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_language() {
        let lang = language();
        assert!(lang.version() > 0);
    }
}
