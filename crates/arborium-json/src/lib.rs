//! JSON grammar for tree-sitter
//!
//! This crate provides the json language grammar for use with tree-sitter.

use tree_sitter_patched_arborium::Language;

unsafe extern "C" {
    fn tree_sitter_json() -> Language;
}

/// Returns the json tree-sitter language.
pub fn language() -> Language {
    unsafe { tree_sitter_json() }
}

/// The highlight query for json.
pub const HIGHLIGHTS_QUERY: &str = include_str!("../../../grammars/tree-sitter-json/queries/highlights.scm");

/// The injections query for json (empty - no injections available).
pub const INJECTIONS_QUERY: &str = "";

/// The locals query for json (empty - no locals available).
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
