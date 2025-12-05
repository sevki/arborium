// Mock Node.js module system
global.module = \{ exports: \{\} \};
global.exports = global.module.exports;

// Dummy tree-sitter globals
global.grammar = () => (\{\});
global.seq = (...args) => args;
global.choice = (...args) => args;
global.repeat = (rule) => rule;
global.repeat1 = (rule) => rule;
global.optional = (rule) => rule;
global.prec = (n, rule) => rule;
global.prec_left = (n, rule) => rule;
global.prec_right = (n, rule) => rule;
global.prec_dynamic = (n, rule) => rule;
global.token = (rule) => rule;
global.alias = (rule, name) => rule;
global.field = (name, rule) => rule;
global.$ = new Proxy(\{\}, \{ get: () => "rule" \});

// Pattern constants (optional - grammars may define their own)
global.NEWLINE = 'newline';
global.WHITESPACE = 'whitespace';
global.IDENTIFIER = 'identifier';
global.NUMBER = 'number';
global.STRING = 'string';
global.COMMENT = 'comment';

// Try to require the grammar file
require('{grammar_path}');
