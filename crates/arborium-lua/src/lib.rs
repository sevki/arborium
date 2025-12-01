//! LUA grammar for tree-sitter
//!
//! This crate provides the lua language grammar for use with tree-sitter.

use tree_sitter_patched_arborium::Language;

unsafe extern "C" {
    fn tree_sitter_lua() -> Language;
}

/// Returns the lua tree-sitter language.
pub fn language() -> Language {
    unsafe { tree_sitter_lua() }
}

/// The highlight query for lua.
pub const HIGHLIGHTS_QUERY: &str = include_str!("../../../grammars/tree-sitter-lua/queries/highlights.scm");

/// The injections query for lua.
pub const INJECTIONS_QUERY: &str = include_str!("../../../grammars/tree-sitter-lua/queries/injections.scm");

/// The locals query for lua.
pub const LOCALS_QUERY: &str = include_str!("../../../grammars/tree-sitter-lua/queries/locals.scm");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_language() {
        let lang = language();
        assert!(lang.version() > 0);
    }
}
