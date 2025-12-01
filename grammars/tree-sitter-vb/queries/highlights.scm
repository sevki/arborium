; VB.NET highlights query

; Comments
(comment) @comment

; Strings
(string_literal) @string
(interpolated_string) @string

; Numbers
(integer_literal) @number
(real_literal) @number

; Boolean
(boolean_literal) @constant

; Nothing/null
(nothing_literal) @constant

; Keywords
[
  "AddHandler"
  "AddressOf"
  "Alias"
  "And"
  "AndAlso"
  "As"
  "Boolean"
  "ByRef"
  "Byte"
  "ByVal"
  "Call"
  "Case"
  "Catch"
  "CBool"
  "CByte"
  "CChar"
  "CDate"
  "CDbl"
  "CDec"
  "Char"
  "CInt"
  "Class"
  "CLng"
  "CObj"
  "Const"
  "Continue"
  "CSByte"
  "CShort"
  "CSng"
  "CStr"
  "CType"
  "CUInt"
  "CULng"
  "CUShort"
  "Date"
  "Decimal"
  "Declare"
  "Default"
  "Delegate"
  "Dim"
  "DirectCast"
  "Do"
  "Double"
  "Each"
  "Else"
  "ElseIf"
  "End"
  "EndIf"
  "Enum"
  "Erase"
  "Error"
  "Event"
  "Exit"
  "False"
  "Finally"
  "For"
  "Friend"
  "Function"
  "Get"
  "GetType"
  "GetXMLNamespace"
  "Global"
  "GoSub"
  "GoTo"
  "Handles"
  "If"
  "Implements"
  "Imports"
  "In"
  "Inherits"
  "Integer"
  "Interface"
  "Is"
  "IsNot"
  "Let"
  "Lib"
  "Like"
  "Long"
  "Loop"
  "Me"
  "Mod"
  "Module"
  "MustInherit"
  "MustOverride"
  "MyBase"
  "MyClass"
  "Namespace"
  "Narrowing"
  "New"
  "Next"
  "Not"
  "Nothing"
  "NotInheritable"
  "NotOverridable"
  "Object"
  "Of"
  "On"
  "Operator"
  "Option"
  "Optional"
  "Or"
  "OrElse"
  "Out"
  "Overloads"
  "Overridable"
  "Overrides"
  "ParamArray"
  "Partial"
  "Private"
  "Property"
  "Protected"
  "Public"
  "RaiseEvent"
  "ReadOnly"
  "ReDim"
  "REM"
  "RemoveHandler"
  "Resume"
  "Return"
  "SByte"
  "Select"
  "Set"
  "Shadows"
  "Shared"
  "Short"
  "Single"
  "Static"
  "Step"
  "Stop"
  "String"
  "Structure"
  "Sub"
  "SyncLock"
  "Then"
  "Throw"
  "To"
  "True"
  "Try"
  "TryCast"
  "TypeOf"
  "UInteger"
  "ULong"
  "UShort"
  "Using"
  "Variant"
  "Wend"
  "When"
  "While"
  "Widening"
  "With"
  "WithEvents"
  "WriteOnly"
  "Xor"
] @keyword

; Types
(primitive_type) @type.builtin
(type_identifier) @type
(namespace_name) @type

; Functions and methods
(function_statement
  name: (identifier) @function)
(sub_statement
  name: (identifier) @function)
(method_declaration
  name: (identifier) @function)
(invocation_expression
  function: (identifier) @function)
(invocation_expression
  function: (member_access_expression
    name: (identifier) @function))

; Variables and parameters
(identifier) @variable
(parameter
  name: (identifier) @variable.parameter)

; Properties
(property_statement
  name: (identifier) @property)
(member_access_expression
  name: (identifier) @property)

; Attributes
(attribute
  name: (identifier) @attribute)

; Operators
[
  "+"
  "-"
  "*"
  "/"
  "\\"
  "^"
  "&"
  "="
  "<>"
  "<"
  ">"
  "<="
  ">="
  "+="
  "-="
  "*="
  "/="
  "\\="
  "^="
  "&="
] @operator

; Punctuation
[
  "("
  ")"
  "["
  "]"
  "{"
  "}"
] @punctuation.bracket

[
  "."
  ","
  ":"
] @punctuation.delimiter
