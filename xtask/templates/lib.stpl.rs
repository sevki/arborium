#![doc = include_str!("../README.md")]

use tree_sitter_patched_arborium::Language;

unsafe extern "C" {
    fn tree_sitter_<%= c_symbol %>() -> Language;
}

/// Returns the <%= grammar_id %> tree-sitter language.
pub fn language() -> Language {
    unsafe { tree_sitter_<%= c_symbol %>() }
}

<% if highlights_exists { %>
/// The highlights query for <%= grammar_id %>.
pub const HIGHLIGHTS_QUERY: &str = include_str!("../queries/highlights.scm");
<% } else { %>
/// The highlights query for <%= grammar_id %> (empty - no highlights available).
pub const HIGHLIGHTS_QUERY: &str = "";
<% } %>

<% if injections_exists { %>
/// The injections query for <%= grammar_id %>.
pub const INJECTIONS_QUERY: &str = include_str!("../queries/injections.scm");
<% } else { %>
/// The injections query for <%= grammar_id %> (empty - no injections available).
pub const INJECTIONS_QUERY: &str = "";
<% } %>

<% if locals_exists { %>
/// The locals query for <%= grammar_id %>.
pub const LOCALS_QUERY: &str = include_str!("../queries/locals.scm");
<% } else { %>
/// The locals query for <%= grammar_id %> (empty - no locals available).
pub const LOCALS_QUERY: &str = "";
<% } %>
<% if !tests_cursed { %>

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grammar() {
        arborium_test_harness::test_grammar(
            language(),
            "<%= grammar_id %>",
            HIGHLIGHTS_QUERY,
            INJECTIONS_QUERY,
            LOCALS_QUERY,
            env!("CARGO_MANIFEST_DIR"),
        );
    }
}
<% } %>
