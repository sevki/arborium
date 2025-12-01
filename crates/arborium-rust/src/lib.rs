//! RUST grammar for tree-sitter
//!
//! This crate provides the rust language grammar for use with tree-sitter.

use tree_sitter_patched_arborium::Language;

unsafe extern "C" {
    fn tree_sitter_rust_orchard() -> Language;
}

/// Returns the rust tree-sitter language.
pub fn language() -> Language {
    unsafe { tree_sitter_rust_orchard() }
}

/// The highlight query for rust.
pub const HIGHLIGHTS_QUERY: &str = include_str!("../../../grammars/tree-sitter-rust/queries/highlights.scm");

/// The injections query for rust.
pub const INJECTIONS_QUERY: &str = include_str!("../../../grammars/tree-sitter-rust/queries/injections.scm");

/// The locals query for rust (empty - no locals available).
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
