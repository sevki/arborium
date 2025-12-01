//! HASKELL grammar for tree-sitter
//!
//! This crate provides the haskell language grammar for use with tree-sitter.

use tree_sitter_patched_arborium::Language;

unsafe extern "C" {
    fn tree_sitter_haskell() -> Language;
}

/// Returns the haskell tree-sitter language.
pub fn language() -> Language {
    unsafe { tree_sitter_haskell() }
}

/// The highlight query for haskell.
pub const HIGHLIGHTS_QUERY: &str = include_str!("../../../grammars/tree-sitter-haskell/queries/highlights.scm");

/// The injections query for haskell.
pub const INJECTIONS_QUERY: &str = include_str!("../../../grammars/tree-sitter-haskell/queries/injections.scm");

/// The locals query for haskell.
pub const LOCALS_QUERY: &str = include_str!("../../../grammars/tree-sitter-haskell/queries/locals.scm");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_language() {
        let lang = language();
        assert!(lang.version() > 0);
    }
}
