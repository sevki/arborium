//! CMAKE grammar for tree-sitter
//!
//! This crate provides the cmake language grammar for use with tree-sitter.

use tree_sitter_patched_arborium::Language;

unsafe extern "C" {
    fn tree_sitter_cmake() -> Language;
}

/// Returns the cmake tree-sitter language.
pub fn language() -> Language {
    unsafe { tree_sitter_cmake() }
}

/// The highlight query for cmake.
pub const HIGHLIGHTS_QUERY: &str = include_str!("../../../grammars/tree-sitter-cmake/queries/highlights.scm");

/// The injections query for cmake.
pub const INJECTIONS_QUERY: &str = include_str!("../../../grammars/tree-sitter-cmake/queries/injections.scm");

/// The locals query for cmake (empty - no locals available).
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
