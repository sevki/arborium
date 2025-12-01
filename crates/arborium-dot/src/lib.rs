//! DOT grammar for tree-sitter
//!
//! This crate provides the dot language grammar for use with tree-sitter.

use tree_sitter_patched_arborium::Language;

unsafe extern "C" {
    fn tree_sitter_dot() -> Language;
}

/// Returns the dot tree-sitter language.
pub fn language() -> Language {
    unsafe { tree_sitter_dot() }
}

/// The highlight query for dot.
pub const HIGHLIGHTS_QUERY: &str = include_str!("../../../grammars/tree-sitter-dot/queries/highlights.scm");

/// The injections query for dot.
pub const INJECTIONS_QUERY: &str = include_str!("../../../grammars/tree-sitter-dot/queries/injections.scm");

/// The locals query for dot (empty - no locals available).
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
