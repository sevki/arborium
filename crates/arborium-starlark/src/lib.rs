//! STARLARK grammar for tree-sitter
//!
//! This crate provides the starlark language grammar for use with tree-sitter.

use tree_sitter_patched_arborium::Language;

unsafe extern "C" {
    fn tree_sitter_starlark() -> Language;
}

/// Returns the starlark tree-sitter language.
pub fn language() -> Language {
    unsafe { tree_sitter_starlark() }
}

/// The highlight query for starlark.
pub const HIGHLIGHTS_QUERY: &str = include_str!("../../../grammars/tree-sitter-starlark/queries/highlights.scm");

/// The injections query for starlark.
pub const INJECTIONS_QUERY: &str = include_str!("../../../grammars/tree-sitter-starlark/queries/injections.scm");

/// The locals query for starlark.
pub const LOCALS_QUERY: &str = include_str!("../../../grammars/tree-sitter-starlark/queries/locals.scm");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_language() {
        let lang = language();
        assert!(lang.version() > 0);
    }
}
