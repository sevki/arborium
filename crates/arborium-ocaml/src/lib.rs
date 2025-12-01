//! OCAML grammar for tree-sitter
//!
//! This crate provides the ocaml language grammar for use with tree-sitter.

use tree_sitter_patched_arborium::Language;

unsafe extern "C" {
    fn tree_sitter_ocaml() -> Language;
}

/// Returns the ocaml tree-sitter language.
pub fn language() -> Language {
    unsafe { tree_sitter_ocaml() }
}

/// The highlight query for ocaml (empty - no highlights available).
pub const HIGHLIGHTS_QUERY: &str = "";

/// The injections query for ocaml (empty - no injections available).
pub const INJECTIONS_QUERY: &str = "";

/// The locals query for ocaml (empty - no locals available).
pub const LOCALS_QUERY: &str = "";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grammar() {
        arborium_test_harness::test_grammar(
            language(),
            "ocaml",
            HIGHLIGHTS_QUERY,
            INJECTIONS_QUERY,
            LOCALS_QUERY,
            env!("CARGO_MANIFEST_DIR"),
        );
    }
}
