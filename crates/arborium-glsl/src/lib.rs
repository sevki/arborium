//! GLSL grammar for tree-sitter
//!
//! This crate provides the glsl language grammar for use with tree-sitter.

use tree_sitter_patched_arborium::Language;

unsafe extern "C" {
    fn tree_sitter_glsl() -> Language;
}

/// Returns the glsl tree-sitter language.
pub fn language() -> Language {
    unsafe { tree_sitter_glsl() }
}

/// The highlight query for glsl.
pub const HIGHLIGHTS_QUERY: &str = include_str!("../../../grammars/tree-sitter-glsl/queries/highlights.scm");

/// The injections query for glsl (empty - no injections available).
pub const INJECTIONS_QUERY: &str = "";

/// The locals query for glsl (empty - no locals available).
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
