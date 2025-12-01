; Batch/CMD highlights query

; Comments
(comment) @comment

; Strings
(string) @string

; Numbers
(number) @number

; Variables
(variable_reference) @variable
(variable_declaration
  (identifier) @variable)

; Labels (functions)
(function_definition) @function

; Echo off
(echooff) @keyword

; Built-in commands
[
  "ASSOC"
  "ATTRIB"
  "CD"
  "CHKDSK"
  "CLS"
  "COPY"
  "DEL"
  "DIR"
  "ECHO"
  "ERASE"
  "EXIT"
  "FC"
  "FIND"
  "FINDSTR"
  "GOTO"
  "IF"
  "MD"
  "MKDIR"
  "MKLINK"
  "MORE"
  "MOVE"
  "PATH"
  "PAUSE"
  "POPD"
  "PRINT"
  "PUSHD"
  "RD"
  "REM"
  "REN"
  "RENAME"
  "RMDIR"
  "SET"
  "SETLOCAL"
  "ENDLOCAL"
  "SHIFT"
  "SORT"
  "START"
  "TIME"
  "TITLE"
  "TREE"
  "TYPE"
  "VER"
  "VOL"
  "XCOPY"
  "CALL"
  "FOR"
  "DO"
  "IN"
  "NOT"
  "EXIST"
  "ELSE"
  "DEFINED"
  "EQU"
  "NEQ"
  "LSS"
  "LEQ"
  "GTR"
  "GEQ"
  "NUL"
  "CON"
  "PRN"
  "AUX"
  "ERRORLEVEL"
] @keyword

; Operators
[
  "="
  "=="
  ">"
  "<"
  ">>"
  "|"
  "&"
  "&&"
  "||"
] @operator

; Punctuation
[
  "("
  ")"
  "%"
  "@"
  ":"
  "::"
] @punctuation.special
