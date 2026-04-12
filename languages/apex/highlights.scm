; Adapted from tree-sitter-sfapex for Zed editor
; https://github.com/aheber/tree-sitter-sfapex

; Punctuation
[
  "["
  "]"
  "{"
  "}"
  "?"
  ";"
] @punctuation.bracket

["." "," ":"] @punctuation.delimiter

(type_arguments "<" @punctuation.bracket)
(type_arguments ">" @punctuation.bracket)

; Methods

(method_declaration
  name: (identifier) @function.method)
(method_declaration
  type: (type_identifier) @type)

(method_invocation
  name: (identifier) @function.method)
(argument_list
  (identifier) @variable)
(super) @function.builtin

(explicit_constructor_invocation
  arguments: (argument_list
    (identifier) @variable))

; Annotations

(annotation
  name: (identifier) @attribute)

"@" @operator

(annotation_key_value
  (identifier) @variable)

; Types

; Convention: SCREAMING_SNAKE_CASE identifiers are constants
((identifier) @constant
  (#match? @constant "^_*[A-Z][A-Z\\d_]+$"))

(interface_declaration
  name: (identifier) @type)
(class_declaration
  name: (identifier) @type)
(class_declaration
  (superclass) @type)
(enum_declaration
  name: (identifier) @type)
(enum_constant
  name: (identifier) @variant)

(interfaces
  (type_list
    (type_identifier) @type))

(local_variable_declaration
  (type_identifier) @type)

(expression_statement (_ (identifier)) @variable)

(generic_type
  (type_identifier) @type)
(type_arguments (type_identifier) @type)

(field_access
  field: (identifier) @property)

((field_access
  object: (identifier) @type))

((scoped_identifier
  scope: (identifier) @type)
 (#match? @type "^[A-Z]"))
((method_invocation
  object: (identifier) @type)
 (#match? @type "^[A-Z]"))

(field_declaration
  type: (type_identifier) @type)

(formal_parameter
  type: (type_identifier) @type
  (identifier) @variable)

(method_declaration
  (formal_parameters
    (formal_parameter
      name: (identifier) @variable)))

(enhanced_for_statement
  type: (type_identifier) @type
  name: (identifier) @variable)

(enhanced_for_statement
  value: (identifier) @variable)

(enhanced_for_statement
  name: (identifier) @variable)

(object_creation_expression
  type: (type_identifier) @type)

(array_creation_expression
  type: (type_identifier) @type)

(array_type
  element: (type_identifier) @type)

(return_statement
  (identifier) @variable)

(local_variable_declaration
  (variable_declarator
    name: (identifier) @variable))

(for_statement
  condition: (binary_expression
    (identifier) @variable))

(for_statement
  update: (update_expression
    (identifier) @variable))

(constructor_declaration
  name: (identifier) @type)

(dml_type) @function.builtin

(bound_apex_expression
  (identifier) @variable)

(instanceof_expression
  left: (identifier) @variable
  right: (type_identifier) @type)

(cast_expression
  type: (type_identifier) @type
  value: (identifier) @variable)

(switch_expression
  condition: (identifier) @variable)

(switch_rule
  (switch_label
    (identifier) @variant))

(when_sobject_type
  (type_identifier) @type
  (identifier) @variable)

(trigger_declaration
  name: (identifier) @type
  object: (identifier) @type
  (trigger_event) @keyword
  ("," (trigger_event) @keyword)*)

; Operators

(binary_expression
  operator: [
    ">"
    "<"
    ">="
    "<="
    "=="
    "==="
    "!="
    "!=="
    "&&"
    "||"
    "+"
    "-"
    "*"
    "/"
    "&"
    "|"
    "^"
    "%"
    "<<"
    ">>"
    ">>>"] @operator)

(binary_expression
  (identifier) @variable)

(unary_expression
  operator: [
    "+"
    "-"
    "!"
    "~"
  ]) @operator

("=>" @operator)

(assignment_operator) @operator
(update_operator) @operator

; Built-in types

[
  (boolean_type)
  (void_type)
] @type.builtin

; Variables

(field_declaration (variable_declarator
  (identifier) @property))

(field_declaration
  (modifiers (modifier [(final) (static)])(modifier [(final) (static)]))
  (variable_declarator
    name: (identifier) @constant))

(this) @variable.special

(assignment_expression
  left: (identifier) @variable)

; Literals

(int) @number
(decimal_floating_point_literal) @number

(string_literal) @string

(boolean) @boolean

(null_literal) @constant

; Comments

(line_comment) @comment
(block_comment) @comment

; Keywords

[
  (abstract)
  (all_rows_clause)
  "break"
  "catch"
  "class"
  "continue"
  "do"
  "else"
  "enum"
  "extends"
  (final)
  "finally"
  "for"
  "get"
  (global)
  "if"
  "implements"
  "instanceof"
  "interface"
  "new"
  "on"
  (override)
  (private)
  (protected)
  (public)
  "return"
  "set"
  (static)
  "switch"
  (testMethod)
  (webservice)
  "throw"
  (transient)
  "try"
  "trigger"
  (virtual)
  "when"
  "while"
  (with_sharing)
  (without_sharing)
  (inherited_sharing)
] @keyword

; Special methods

"System.runAs" @function.builtin

(scoped_type_identifier
  (type_identifier) @type)
