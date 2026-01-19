; Keywords
["true" "false"] @constant.builtin

; Operators
["||" "&&" "!" "==" "!=" "<" "<=" ">" ">=" "in" "+" "-" "*" "/" "%"] @operator
["?" ":"] @conditional

; Macros
["has" "all" "exists" "exists_one" "filter" "map"] @function.macro

; Functions
(function_call function: (identifier) @function)
(member_call method: (method_name) @function.method)

; Literals
(int_literal) @number
(uint_literal) @number
(double_literal) @number.float
(string_literal) @string
(bytes_literal) @string.special
(bool_literal) @boolean
(null_literal) @constant.builtin

; Identifiers
(identifier) @property
(field_initializer field: (identifier) @property)

; Comments
(comment) @comment
