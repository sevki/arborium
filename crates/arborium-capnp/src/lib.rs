//! CAPNP grammar for tree-sitter
//!
//! This crate provides the capnp language grammar for use with tree-sitter.

use tree_sitter_patched_arborium::Language;

unsafe extern "C" {
    fn tree_sitter_capnp() -> Language;
}

/// Returns the capnp tree-sitter language.
pub fn language() -> Language {
    unsafe { tree_sitter_capnp() }
}

/// The highlight query for capnp.
pub const HIGHLIGHTS_QUERY: &str = include_str!("../../../grammars/tree-sitter-capnp/queries/highlights.scm");

/// The injections query for capnp.
pub const INJECTIONS_QUERY: &str = include_str!("../../../grammars/tree-sitter-capnp/queries/injections.scm");

/// The locals query for capnp.
pub const LOCALS_QUERY: &str = include_str!("../../../grammars/tree-sitter-capnp/queries/locals.scm");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_language() {
        let lang = language();
        assert!(lang.version() > 0);
    }
}
