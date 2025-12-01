//! POSTSCRIPT grammar for tree-sitter
//!
//! This crate provides the postscript language grammar for use with tree-sitter.

use tree_sitter_patched_arborium::Language;

unsafe extern "C" {
    fn tree_sitter_postscript() -> Language;
}

/// Returns the postscript tree-sitter language.
pub fn language() -> Language {
    unsafe { tree_sitter_postscript() }
}

/// The highlight query for postscript (empty - no highlights available).
pub const HIGHLIGHTS_QUERY: &str = "";

/// The injections query for postscript (empty - no injections available).
pub const INJECTIONS_QUERY: &str = "";

/// The locals query for postscript (empty - no locals available).
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
