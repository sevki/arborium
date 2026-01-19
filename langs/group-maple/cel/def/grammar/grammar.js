module.exports = grammar({
  name: 'cel',

  extras: ($) => [/\s/, $.comment],

  conflicts: ($) => [[$._primary_expression, $.qualified_type]],

  word: ($) => $.identifier,

  rules: {
    source_file: ($) => optional($._expression),

    _expression: ($) =>
      choice($.conditional_expression, $.logical_or_expression),

    // Conditional (ternary) operator: e ? e1 : e2
    conditional_expression: ($) =>
      prec.right(
        1,
        seq(
          field('condition', $.logical_or_expression),
          '?',
          field('consequence', $._expression),
          ':',
          field('alternative', $._expression),
        ),
      ),

    // Logical operators
    logical_or_expression: ($) =>
      prec.left(
        2,
        choice(
          $._logical_and_expression,
          seq(
            field('left', $.logical_or_expression),
            field('operator', '||'),
            field('right', $._logical_and_expression),
          ),
        ),
      ),

    _logical_and_expression: ($) =>
      choice($.binary_expression, $.logical_and_expression),

    logical_and_expression: ($) =>
      prec.left(
        3,
        seq(
          field('left', $._logical_and_expression),
          field('operator', '&&'),
          field('right', $._logical_and_expression),
        ),
      ),

    // Binary operators (relational and arithmetic)
    binary_expression: ($) =>
      choice(
        $.relation_expression,
        $.addition_expression,
        $.multiplication_expression,
        $._unary_expression,
      ),

    relation_expression: ($) =>
      prec.left(
        4,
        seq(
          field('left', $.binary_expression),
          field('operator', choice('<', '<=', '>=', '>', '==', '!=', 'in')),
          field('right', $.binary_expression),
        ),
      ),

    addition_expression: ($) =>
      prec.left(
        5,
        seq(
          field('left', $.binary_expression),
          field('operator', choice('+', '-')),
          field('right', $.binary_expression),
        ),
      ),

    multiplication_expression: ($) =>
      prec.left(
        6,
        seq(
          field('left', $.binary_expression),
          field('operator', choice('*', '/', '%')),
          field('right', $.binary_expression),
        ),
      ),

    // Unary operators
    _unary_expression: ($) => choice($._member_expression, $.unary_expression),

    unary_expression: ($) =>
      prec(
        7,
        seq(
          field('operator', choice('!', '-')),
          field('operand', $._unary_expression),
        ),
      ),

    // Member access and function calls
    _member_expression: ($) =>
      choice(
        $._primary_expression,
        seq(
          field('object', $._member_expression),
          '.',
          field('field', $.identifier),
        ),
        $.member_call,
        $.index,
        $.macro_call,
      ),

    member_call: ($) =>
      prec.left(
        10,
        seq(
          field('function', $._member_expression),
          '.',
          field('method', alias($.identifier, $.method_name)),
          field('arguments', $.argument_list),
        ),
      ),

    index: ($) =>
      prec.left(
        10,
        seq(
          field('object', $._member_expression),
          '[',
          field('index', $._expression),
          ']',
        ),
      ),

    // Macros (has, all, exists, exists_one, filter, map)
    macro_call: ($) =>
      choice(
        $.has_macro,
        $.all_macro,
        $.exists_macro,
        $.exists_one_macro,
        $.filter_macro,
        $.map_macro,
      ),

    has_macro: ($) =>
      prec(11, seq('has', '(', field('field', $._member_expression), ')')),

    all_macro: ($) =>
      prec.left(
        10,
        seq(
          field('target', $._member_expression),
          '.',
          'all',
          '(',
          field('variable', $.identifier),
          ',',
          field('predicate', $._expression),
          ')',
        ),
      ),

    exists_macro: ($) =>
      prec.left(
        10,
        seq(
          field('target', $._member_expression),
          '.',
          'exists',
          '(',
          field('variable', $.identifier),
          ',',
          field('predicate', $._expression),
          ')',
        ),
      ),

    exists_one_macro: ($) =>
      prec.left(
        10,
        seq(
          field('target', $._member_expression),
          '.',
          'exists_one',
          '(',
          field('variable', $.identifier),
          ',',
          field('predicate', $._expression),
          ')',
        ),
      ),

    filter_macro: ($) =>
      prec.left(
        10,
        seq(
          field('target', $._member_expression),
          '.',
          'filter',
          '(',
          field('variable', $.identifier),
          ',',
          field('predicate', $._expression),
          ')',
        ),
      ),

    map_macro: ($) =>
      prec.left(
        10,
        seq(
          field('target', $._member_expression),
          '.',
          'map',
          '(',
          field('variable', $.identifier),
          ',',
          field('transform', $._expression),
          ')',
        ),
      ),

    // Primary expressions
    _primary_expression: ($) =>
      choice(
        $.identifier,
        $._literal,
        $.list_literal,
        $.map_literal,
        seq('(', $._expression, ')'),
        $.function_call,
        $.message_literal,
      ),

    function_call: ($) =>
      seq(field('function', $.identifier), field('arguments', $.argument_list)),

    argument_list: ($) =>
      seq(
        '(',
        optional(seq($._expression, repeat(seq(',', $._expression)))),
        optional(','),
        ')',
      ),

    // Literals
    _literal: ($) =>
      choice(
        $.int_literal,
        $.uint_literal,
        $.double_literal,
        $.string_literal,
        $.bytes_literal,
        $.bool_literal,
        $.null_literal,
      ),

    int_literal: ($) =>
      token(seq(optional('-'), choice(/[0-9]+/, /0[xX][0-9a-fA-F]+/))),

    uint_literal: ($) =>
      token(seq(choice(/[0-9]+/, /0[xX][0-9a-fA-F]+/), choice('u', 'U'))),

    double_literal: ($) =>
      token(
        choice(
          seq(
            optional('-'),
            /[0-9]+/,
            '.',
            /[0-9]*/,
            optional(seq(/[eE]/, optional(choice('+', '-')), /[0-9]+/)),
          ),
          seq(
            optional('-'),
            /[0-9]+/,
            /[eE]/,
            optional(choice('+', '-')),
            /[0-9]+/,
          ),
          seq(
            optional('-'),
            '.',
            /[0-9]+/,
            optional(seq(/[eE]/, optional(choice('+', '-')), /[0-9]+/)),
          ),
        ),
      ),

    string_literal: ($) =>
      choice(
        seq('"', repeat(choice($.escape_sequence, /[^"\\\n]/)), '"'),
        seq("'", repeat(choice($.escape_sequence, /[^'\\\n]/)), "'"),
        $.raw_string_literal,
        $.multiline_string_literal,
      ),

    raw_string_literal: ($) =>
      choice(
        seq('r"', repeat(/[^"]/), '"'),
        seq("r'", repeat(/[^']/), "'"),
        seq('R"', repeat(/[^"]/), '"'),
        seq("R'", repeat(/[^']/), "'"),
      ),

    multiline_string_literal: ($) =>
      choice(seq('"""', repeat(/./), '"""'), seq("'''", repeat(/./), "'''")),

    bytes_literal: ($) =>
      choice(
        seq('b"', repeat(choice($.escape_sequence, /[^"\\\n]/)), '"'),
        seq("b'", repeat(choice($.escape_sequence, /[^'\\\n]/)), "'"),
        seq('B"', repeat(choice($.escape_sequence, /[^"\\\n]/)), '"'),
        seq("B'", repeat(choice($.escape_sequence, /[^'\\\n]/)), "'"),
      ),

    escape_sequence: ($) =>
      token(
        seq(
          '\\',
          choice(
            /[abfnrtv\\'"\?]/,
            /[0-7]{1,3}/,
            /x[0-9a-fA-F]{2}/,
            /u[0-9a-fA-F]{4}/,
            /U[0-9a-fA-F]{8}/,
          ),
        ),
      ),

    bool_literal: ($) => choice('true', 'false'),

    null_literal: ($) => 'null',

    // List literal
    list_literal: ($) =>
      seq(
        '[',
        optional(
          seq($._expression, repeat(seq(',', $._expression)), optional(',')),
        ),
        ']',
      ),

    // Map literal
    map_literal: ($) =>
      seq(
        '{',
        optional(
          seq($.map_entry, repeat(seq(',', $.map_entry)), optional(',')),
        ),
        '}',
      ),

    map_entry: ($) =>
      seq(field('key', $._expression), ':', field('value', $._expression)),

    // Message literal (protocol buffer message construction)
    // Type.Name{field: value} or pkg.Type.Name{field: value}
    message_literal: ($) =>
      seq(
        field('type', $.qualified_type),
        '{',
        optional(
          seq(
            $.field_initializer,
            repeat(seq(',', $.field_initializer)),
            optional(','),
          ),
        ),
        '}',
      ),

    qualified_type: ($) => seq($.identifier, repeat1(seq('.', $.identifier))),

    field_initializer: ($) =>
      seq(field('field', $.identifier), ':', field('value', $._expression)),

    // Identifiers
    identifier: ($) => /[_a-zA-Z][_a-zA-Z0-9]*/,

    method_name: ($) => /[_a-zA-Z][_a-zA-Z0-9]*/,

    // Comments
    comment: ($) =>
      token(
        choice(seq('//', /.*/), seq('/*', /[^*]*\*+([^/*][^*]*\*+)*/, '/')),
      ),
  },
});
