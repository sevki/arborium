//! PHP grammar for tree-sitter
//!
//! This crate provides the php language grammar for use with tree-sitter.

use tree_sitter_patched_arborium::Language;

unsafe extern "C" {
    fn tree_sitter_php() -> Language;
}

/// Returns the php tree-sitter language.
pub fn language() -> Language {
    unsafe { tree_sitter_php() }
}

unsafe extern "C" {
    fn tree_sitter_php_only() -> Language;
}

/// Returns the php_only tree-sitter language.
pub fn php_only_language() -> Language {
    unsafe { tree_sitter_php_only() }
}

/// The highlight query for php.
pub const HIGHLIGHTS_QUERY: &str = include_str!("../../../grammars/tree-sitter-php/queries/highlights.scm");

/// The injections query for php.
pub const INJECTIONS_QUERY: &str = include_str!("../../../grammars/tree-sitter-php/queries/injections.scm");

/// The locals query for php (empty - no locals available).
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
