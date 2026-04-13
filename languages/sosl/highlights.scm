; Adapted from tree-sitter-sfapex for Zed editor
; https://github.com/aheber/tree-sitter-sfapex

(find_clause
  (term) @string)

(sobject_return
  (identifier) @type)

(with_type (_ "=" @operator))

[
  "ALL"
  "DIVISION"
  "EMAIL"
  "FIND"
  "HIGHLIGHT"
  "IN"
  "ListView"
  "METADATA"
  "NAME"
  "NETWORK"
  "PHONE"
  "PricebookId"
  "RETURNING"
  "SIDEBAR"
  "SNIPPET"
  "SPELL_CORRECTION"
  "target_length"
  "USING"
] @keyword
