//! OBJC grammar for tree-sitter
//!
//! This crate provides the objc language grammar for use with tree-sitter.

use tree_sitter_patched_arborium::Language;

unsafe extern "C" {
    fn tree_sitter_objc() -> Language;
}

/// Returns the objc tree-sitter language.
pub fn language() -> Language {
    unsafe { tree_sitter_objc() }
}

/// The highlight query for objc.
pub const HIGHLIGHTS_QUERY: &str = include_str!("../../../grammars/tree-sitter-objc/queries/highlights.scm");

/// The injections query for objc.
pub const INJECTIONS_QUERY: &str = include_str!("../../../grammars/tree-sitter-objc/queries/injections.scm");

/// The locals query for objc.
pub const LOCALS_QUERY: &str = include_str!("../../../grammars/tree-sitter-objc/queries/locals.scm");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_language() {
        let lang = language();
        assert!(lang.version() > 0);
    }
}
