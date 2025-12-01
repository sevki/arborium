//! SVELTE grammar for tree-sitter
//!
//! This crate provides the svelte language grammar for use with tree-sitter.

use tree_sitter_patched_arborium::Language;

unsafe extern "C" {
    fn tree_sitter_svelte() -> Language;
}

/// Returns the svelte tree-sitter language.
pub fn language() -> Language {
    unsafe { tree_sitter_svelte() }
}

/// The highlight query for svelte (includes HTML highlights as base).
pub const HIGHLIGHTS_QUERY: &str = concat!(
    include_str!("../../../grammars/tree-sitter-html/queries/highlights.scm"),
    "\n",
    include_str!("../../../grammars/tree-sitter-svelte/queries/highlights.scm"),
);

/// The injections query for svelte.
pub const INJECTIONS_QUERY: &str = include_str!("../../../grammars/tree-sitter-svelte/queries/injections.scm");

/// The locals query for svelte.
pub const LOCALS_QUERY: &str = include_str!("../../../grammars/tree-sitter-svelte/queries/locals.scm");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_language() {
        let lang = language();
        assert!(lang.version() > 0);
    }
}
