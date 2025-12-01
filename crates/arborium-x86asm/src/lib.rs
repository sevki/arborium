//! X86ASM grammar for tree-sitter
//!
//! This crate provides the x86asm language grammar for use with tree-sitter.

use tree_sitter_patched_arborium::Language;

unsafe extern "C" {
    fn tree_sitter_x86asm() -> Language;
}

/// Returns the x86asm tree-sitter language.
pub fn language() -> Language {
    unsafe { tree_sitter_x86asm() }
}

/// The highlight query for x86asm.
pub const HIGHLIGHTS_QUERY: &str = include_str!("../../../grammars/tree-sitter-x86asm/queries/highlights.scm");

/// The injections query for x86asm (empty - no injections available).
pub const INJECTIONS_QUERY: &str = "";

/// The locals query for x86asm (empty - no locals available).
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
