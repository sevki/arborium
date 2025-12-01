//! GLEAM grammar for tree-sitter
//!
//! This crate provides the gleam language grammar for use with tree-sitter.

use tree_sitter_patched_arborium::Language;

unsafe extern "C" {
    fn tree_sitter_gleam() -> Language;
}

/// Returns the gleam tree-sitter language.
pub fn language() -> Language {
    unsafe { tree_sitter_gleam() }
}

/// The highlight query for gleam.
pub const HIGHLIGHTS_QUERY: &str = include_str!("../../../grammars/tree-sitter-gleam/queries/highlights.scm");

/// The injections query for gleam.
pub const INJECTIONS_QUERY: &str = include_str!("../../../grammars/tree-sitter-gleam/queries/injections.scm");

/// The locals query for gleam.
pub const LOCALS_QUERY: &str = include_str!("../../../grammars/tree-sitter-gleam/queries/locals.scm");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_language() {
        let lang = language();
        assert!(lang.version() > 0);
    }
}
