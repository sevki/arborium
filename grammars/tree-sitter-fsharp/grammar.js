/**
 * @file Fsharp grammar for tree-sitter
 * @author Nikolaj Sidorenco
 * @license MIT
 */

/// <reference types="tree-sitter-cli/dsl" />
// @ts-check

module.exports = grammar({
  name: "fsharp",

  rules: {
    // TODO: add the actual grammar rules
    source_file: $ => "hello"
  }
});
