//! SSH-CONFIG grammar for tree-sitter
//!
//! This crate provides the ssh-config language grammar for use with tree-sitter.

use tree_sitter_patched_arborium::Language;

unsafe extern "C" {
    fn tree_sitter_ssh_config() -> Language;
}

/// Returns the ssh-config tree-sitter language.
pub fn language() -> Language {
    unsafe { tree_sitter_ssh_config() }
}

/// The highlight query for ssh-config.
pub const HIGHLIGHTS_QUERY: &str = include_str!("../../../grammars/tree-sitter-ssh-config/queries/highlights.scm");

/// The injections query for ssh-config.
pub const INJECTIONS_QUERY: &str = include_str!("../../../grammars/tree-sitter-ssh-config/queries/injections.scm");

/// The locals query for ssh-config (empty - no locals available).
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
