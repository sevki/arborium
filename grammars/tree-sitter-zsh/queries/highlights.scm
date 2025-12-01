; Comments
(comment) @comment

; Keywords
(keyword) @keyword

; Strings
(string) @string
(raw_string) @string
(escape_sequence) @string.escape

; Variables
(expansion) @variable
(special_variable) @variable.builtin

; Command substitution
(command_substitution) @embedded

; Numbers
(number) @number

; Operators
(operator) @operator

; Words (commands, arguments, etc) - low priority
(word) @string.special
