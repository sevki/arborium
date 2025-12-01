//! F# grammar for tree-sitter
//!
//! This crate provides the fsharp language grammar for use with tree-sitter.

use tree_sitter_patched_arborium::Language;

unsafe extern "C" {
    fn tree_sitter_fsharp() -> Language;
}

/// Returns the fsharp tree-sitter language.
pub fn language() -> Language {
    unsafe { tree_sitter_fsharp() }
}

/// The highlight query for fsharp.
pub const HIGHLIGHTS_QUERY: &str = include_str!("../../../grammars/tree-sitter-fsharp/queries/highlights.scm");

/// The injections query for fsharp.
pub const INJECTIONS_QUERY: &str = include_str!("../../../grammars/tree-sitter-fsharp/queries/injections.scm");

/// The locals query for fsharp.
pub const LOCALS_QUERY: &str = include_str!("../../../grammars/tree-sitter-fsharp/queries/locals.scm");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_language() {
        let lang = language();
        assert!(lang.version() > 0);
    }
}
