//! Tree-sitter grammar for {grammar_id}.

use tree_sitter::Language;
use tree_sitter_language::LanguageFn;

extern "C" {
    fn tree_sitter_{c_symbol}() -> Language;
}

/// The tree-sitter [`Language`] for {grammar_id}.
pub const LANGUAGE: LanguageFn = || tree_sitter_{c_symbol}();

/// The highlights query for {grammar_id}.
{highlights_query}

/// The injections query for {grammar_id}.
{injections_query}

#[cfg(test)]
mod tests {
    #[test]
    fn can_load_grammar() {
        let mut parser = tree_sitter::Parser::new();
        parser
            .set_language(&super::LANGUAGE())
            .expect("Error loading {grammar_id} grammar");
    }

    {test_function}
}
