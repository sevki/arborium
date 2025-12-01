//! TLAPLUS grammar for tree-sitter
//!
//! This crate provides the tlaplus language grammar for use with tree-sitter.

use tree_sitter_patched_arborium::Language;

unsafe extern "C" {
    fn tree_sitter_tlaplus() -> Language;
}

/// Returns the tlaplus tree-sitter language.
pub fn language() -> Language {
    unsafe { tree_sitter_tlaplus() }
}

/// The highlight query for tlaplus.
pub const HIGHLIGHTS_QUERY: &str = include_str!("../../../grammars/tree-sitter-tlaplus/queries/highlights.scm");

/// The injections query for tlaplus (empty - no injections available).
pub const INJECTIONS_QUERY: &str = "";

/// The locals query for tlaplus.
pub const LOCALS_QUERY: &str = include_str!("../../../grammars/tree-sitter-tlaplus/queries/locals.scm");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_language() {
        let lang = language();
        assert!(lang.version() > 0);
    }
}
