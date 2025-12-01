//! PERL grammar for tree-sitter
//!
//! This crate provides the perl language grammar for use with tree-sitter.

use tree_sitter_patched_arborium::Language;

unsafe extern "C" {
    fn tree_sitter_perl() -> Language;
}

/// Returns the perl tree-sitter language.
pub fn language() -> Language {
    unsafe { tree_sitter_perl() }
}

/// The highlight query for perl.
pub const HIGHLIGHTS_QUERY: &str = include_str!("../../../grammars/tree-sitter-perl/queries/highlights.scm");

/// The injections query for perl.
pub const INJECTIONS_QUERY: &str = include_str!("../../../grammars/tree-sitter-perl/queries/injections.scm");

/// The locals query for perl (empty - no locals available).
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
