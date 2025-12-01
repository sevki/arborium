//! R grammar for tree-sitter
//!
//! This crate provides the r language grammar for use with tree-sitter.

use tree_sitter_patched_arborium::Language;

unsafe extern "C" {
    fn tree_sitter_r() -> Language;
}

/// Returns the r tree-sitter language.
pub fn language() -> Language {
    unsafe { tree_sitter_r() }
}

/// The highlight query for r.
pub const HIGHLIGHTS_QUERY: &str = include_str!("../../../grammars/tree-sitter-r/queries/highlights.scm");

/// The injections query for r (empty - no injections available).
pub const INJECTIONS_QUERY: &str = "";

/// The locals query for r.
pub const LOCALS_QUERY: &str = include_str!("../../../grammars/tree-sitter-r/queries/locals.scm");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_language() {
        let lang = language();
        assert!(lang.version() > 0);
    }
}
