//! NGINX grammar for tree-sitter
//!
//! This crate provides the nginx language grammar for use with tree-sitter.

use tree_sitter_patched_arborium::Language;

unsafe extern "C" {
    fn tree_sitter_nginx() -> Language;
}

/// Returns the nginx tree-sitter language.
pub fn language() -> Language {
    unsafe { tree_sitter_nginx() }
}

/// The highlight query for nginx.
pub const HIGHLIGHTS_QUERY: &str = include_str!("../../../grammars/tree-sitter-nginx/queries/highlights.scm");

/// The injections query for nginx.
pub const INJECTIONS_QUERY: &str = include_str!("../../../grammars/tree-sitter-nginx/queries/injections.scm");

/// The locals query for nginx (empty - no locals available).
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
