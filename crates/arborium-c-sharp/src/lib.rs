//! C-SHARP grammar for tree-sitter
//!
//! This crate provides the c-sharp language grammar for use with tree-sitter.

use tree_sitter_patched_arborium::Language;

unsafe extern "C" {
    fn tree_sitter_c_sharp() -> Language;
}

/// Returns the c-sharp tree-sitter language.
pub fn language() -> Language {
    unsafe { tree_sitter_c_sharp() }
}

/// The highlight query for c-sharp.
pub const HIGHLIGHTS_QUERY: &str = include_str!("../../../grammars/tree-sitter-c-sharp/queries/highlights.scm");

/// The injections query for c-sharp (empty - no injections available).
pub const INJECTIONS_QUERY: &str = "";

/// The locals query for c-sharp (empty - no locals available).
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
