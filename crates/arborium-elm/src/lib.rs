//! ELM grammar for tree-sitter
//!
//! This crate provides the elm language grammar for use with tree-sitter.

use tree_sitter_patched_arborium::Language;

unsafe extern "C" {
    fn tree_sitter_elm() -> Language;
}

/// Returns the elm tree-sitter language.
pub fn language() -> Language {
    unsafe { tree_sitter_elm() }
}

/// The highlight query for elm.
pub const HIGHLIGHTS_QUERY: &str = include_str!("../../../grammars/tree-sitter-elm/queries/highlights.scm");

/// The injections query for elm.
pub const INJECTIONS_QUERY: &str = include_str!("../../../grammars/tree-sitter-elm/queries/injections.scm");

/// The locals query for elm.
pub const LOCALS_QUERY: &str = include_str!("../../../grammars/tree-sitter-elm/queries/locals.scm");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_language() {
        let lang = language();
        assert!(lang.version() > 0);
    }
}
