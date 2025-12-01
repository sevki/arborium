//! POWERSHELL grammar for tree-sitter
//!
//! This crate provides the powershell language grammar for use with tree-sitter.

use tree_sitter_patched_arborium::Language;

unsafe extern "C" {
    fn tree_sitter_powershell() -> Language;
}

/// Returns the powershell tree-sitter language.
pub fn language() -> Language {
    unsafe { tree_sitter_powershell() }
}

/// The highlight query for powershell.
pub const HIGHLIGHTS_QUERY: &str = include_str!("../../../grammars/tree-sitter-powershell/queries/highlights.scm");

/// The injections query for powershell (empty - no injections available).
pub const INJECTIONS_QUERY: &str = "";

/// The locals query for powershell (empty - no locals available).
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
