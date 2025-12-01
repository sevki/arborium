//! THRIFT grammar for tree-sitter
//!
//! This crate provides the thrift language grammar for use with tree-sitter.

use tree_sitter_patched_arborium::Language;

unsafe extern "C" {
    fn tree_sitter_thrift() -> Language;
}

/// Returns the thrift tree-sitter language.
pub fn language() -> Language {
    unsafe { tree_sitter_thrift() }
}

/// The highlight query for thrift.
pub const HIGHLIGHTS_QUERY: &str = include_str!("../../../grammars/tree-sitter-thrift/queries/highlights.scm");

/// The injections query for thrift.
pub const INJECTIONS_QUERY: &str = include_str!("../../../grammars/tree-sitter-thrift/queries/injections.scm");

/// The locals query for thrift.
pub const LOCALS_QUERY: &str = include_str!("../../../grammars/tree-sitter-thrift/queries/locals.scm");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_language() {
        let lang = language();
        assert!(lang.version() > 0);
    }
}
