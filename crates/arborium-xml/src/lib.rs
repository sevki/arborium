//! XML grammar for tree-sitter
//!
//! This crate provides the xml language grammar for use with tree-sitter.

use tree_sitter_patched_arborium::Language;

unsafe extern "C" {
    fn tree_sitter_xml() -> Language;
}

/// Returns the xml tree-sitter language.
pub fn language() -> Language {
    unsafe { tree_sitter_xml() }
}

unsafe extern "C" {
    fn tree_sitter_dtd() -> Language;
}

/// Returns the dtd tree-sitter language.
pub fn dtd_language() -> Language {
    unsafe { tree_sitter_dtd() }
}

/// The highlight query for xml.
pub const HIGHLIGHTS_QUERY: &str = include_str!("../../../grammars/tree-sitter-xml/queries/highlights.scm");

/// The injections query for xml (empty - no injections available).
pub const INJECTIONS_QUERY: &str = "";

/// The locals query for xml (empty - no locals available).
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
