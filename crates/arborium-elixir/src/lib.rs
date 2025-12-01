//! ELIXIR grammar for tree-sitter
//!
//! This crate provides the elixir language grammar for use with tree-sitter.

use tree_sitter_patched_arborium::Language;

unsafe extern "C" {
    fn tree_sitter_elixir() -> Language;
}

/// Returns the elixir tree-sitter language.
pub fn language() -> Language {
    unsafe { tree_sitter_elixir() }
}

/// The highlight query for elixir.
pub const HIGHLIGHTS_QUERY: &str = include_str!("../../../grammars/tree-sitter-elixir/queries/highlights.scm");

/// The injections query for elixir.
pub const INJECTIONS_QUERY: &str = include_str!("../../../grammars/tree-sitter-elixir/queries/injections.scm");

/// The locals query for elixir (empty - no locals available).
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
