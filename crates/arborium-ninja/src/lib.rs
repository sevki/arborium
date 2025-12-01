//! NINJA grammar for tree-sitter
//!
//! This crate provides the ninja language grammar for use with tree-sitter.

use tree_sitter_patched_arborium::Language;

unsafe extern "C" {
    fn tree_sitter_ninja() -> Language;
}

/// Returns the ninja tree-sitter language.
pub fn language() -> Language {
    unsafe { tree_sitter_ninja() }
}

/// The highlight query for ninja.
pub const HIGHLIGHTS_QUERY: &str = include_str!("../../../grammars/tree-sitter-ninja/queries/highlights.scm");

/// The injections query for ninja (empty - no injections available).
pub const INJECTIONS_QUERY: &str = "";

/// The locals query for ninja (empty - no locals available).
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
