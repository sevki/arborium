//! DEVICETREE grammar for tree-sitter
//!
//! This crate provides the devicetree language grammar for use with tree-sitter.

use tree_sitter_patched_arborium::Language;

unsafe extern "C" {
    fn tree_sitter_devicetree() -> Language;
}

/// Returns the devicetree tree-sitter language.
pub fn language() -> Language {
    unsafe { tree_sitter_devicetree() }
}

/// The highlight query for devicetree.
pub const HIGHLIGHTS_QUERY: &str = include_str!("../../../grammars/tree-sitter-devicetree/queries/highlights.scm");

/// The injections query for devicetree (empty - no injections available).
pub const INJECTIONS_QUERY: &str = "";

/// The locals query for devicetree (empty - no locals available).
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
