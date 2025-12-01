//! NIX grammar for tree-sitter
//!
//! This crate provides the nix language grammar for use with tree-sitter.

use tree_sitter_patched_arborium::Language;

unsafe extern "C" {
    fn tree_sitter_nix() -> Language;
}

/// Returns the nix tree-sitter language.
pub fn language() -> Language {
    unsafe { tree_sitter_nix() }
}

/// The highlight query for nix.
pub const HIGHLIGHTS_QUERY: &str = include_str!("../../../grammars/tree-sitter-nix/queries/highlights.scm");

/// The injections query for nix.
pub const INJECTIONS_QUERY: &str = include_str!("../../../grammars/tree-sitter-nix/queries/injections.scm");

/// The locals query for nix.
pub const LOCALS_QUERY: &str = include_str!("../../../grammars/tree-sitter-nix/queries/locals.scm");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_language() {
        let lang = language();
        assert!(lang.version() > 0);
    }
}
