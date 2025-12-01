//! VB grammar for tree-sitter
//!
//! This crate provides the vb language grammar for use with tree-sitter.

use tree_sitter_patched_arborium::Language;

unsafe extern "C" {
    fn tree_sitter_vb_dotnet() -> Language;
}

/// Returns the vb tree-sitter language.
pub fn language() -> Language {
    unsafe { tree_sitter_vb_dotnet() }
}

/// The highlight query for vb (empty - no highlights available).
pub const HIGHLIGHTS_QUERY: &str = "";

/// The injections query for vb (empty - no injections available).
pub const INJECTIONS_QUERY: &str = "";

/// The locals query for vb (empty - no locals available).
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
