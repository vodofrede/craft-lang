# Syntax

## Literals

**Bools** are the entire set of literals allowed in boolean logic.  
```
bool -> "true" | "false"
```

**Numbers** are integer or floating point literals.  
```
digit -> [0-9]
number -> digit+ ("." digit+)?
```

**Text** is any amount of unicode characters surrounded by unescaped quotation marks. These literals may be prefixed with a single-character text modifier which specifies how the contained text should be interpreted.  
```
text_modifier -> ("f" | "r" | "c" | "b")
text -> text_modifier? '"' (\. | [^\])* '"'
```  

**Unit** values are a special type of literal which only has one value, "unit". It is used to represent the empty value, and is returned by default for most expressions which do not otherwise return a value.
```
unit -> "unit"
```

**Identifiers** define the names of entities. Valid identifiers are defined in terms of the [Unicode XID](https://www.unicode.org/reports/tr31/tr31-39.html#D1) start and continue sets. Identifiers are allowed to start with an underscore, which marks them as ignored for linting purposes.  
```
id -> (xid_start | "_") xid_continue*
```

## Expressions

**Expressions** evaluate to a value and always have a type. An expression can evaluate to the `unit` value.  
```
expr -> literal_expr
    | path_expr
    | infix_expr
    | prefix_expr
    | seq_expr
    | list_expr
    | table_expr
    | block_expr
    | if_expr
    | loop_expr
    | match_expr
    | stmt_expr

literal_expr -> bool | number | text | unit
path_expr -> id ("." id)
infix_expr -> expr infix_op expr
prefix_expr -> prefix_op expr
seq_expr -> "(" expr ( "," expr )* ")"
list_expr -> "[" expr ( "," expr )* "]"
table_expr -> "[" pattern "=" expr ( "," pattern "=" expr )* "]"
block_expr -> "do" expr* "end"
if_expr -> "if" expr "then" expr* ("else" expr*)? "end"
loop_expr -> "loop" expr* "end"
match_expr -> "match" expr "with" (pattern "then" expr*)+
```

**Statement Expressions** are expressions which evaluate to the unit value always.
```
stmt_expr -> bind_stmt
    | function_stmt
    | type_stmt
    | record_stmt
    | trait_stmt
    | trait_impl

bind_stmt -> pattern (":" type)? "=" expr ("else" expr*)?
function_stmt -> "function" id "(" (pattern ":" type ("=" expr*)?)* ")" stmt_expr* "end"
type_stmt -> "type" id "is" type ("," type)* "end"
record_stmt -> "record" id "is" (id ":" type ",")* "end"
trait_stmt -> "trait" id "is" stmt_expr* "end"
trait_impl -> id "has" id stmt_expr* "end"
```

**Operators** apply operations to their left and/or right operands.
```
infix_op -> "+" | "-" | "*" | "/" | "%" | "<" | ">" | "=" | "==" | "<=" | ">=" | "and" | "or" | "xor"
prefix_op -> "-" | "not"
```

## Patterns

**Patterns** are used to destructure and bind values from structures. "Basic" patterns are a subset of expressions.  
```
multi_pattern -> pattern ("or" pattern)*
pattern -> literal_pattern 
    | seq_pattern
    | list_pattern
    | table_pattern
    | type_pattern
    | record_pattern 

literal_pattern -> bool | number | text | unit | id
sequence_pattern -> "(" pattern ("," pattern)* ")"
list_pattern -> "[" pattern ("," pattern)* "]"
table_pattern -> "{" pattern ("," pattern)* "}"
type_pattern -> path "(" pattern ("," pattern)* ")"
record_pattern -> path "{" pattern ("," pattern)* "}"
```

**Types** are a separate namespace to identifiers which are used to verify program correctness before runtime. The language contains a few built-in types which form the basis for all other types.  
```
primitive_type -> "bool" | "number" | "text" | "unit"
type -> primitive_type | id
```

## Associativity & Precedence

Operators/Expressions higher in the table have stronger binding.  
Operators with the same precedence share a row.

| Operator/Expression        | Associativity       | Examples          |
| -------------------------- | ------------------- | ----------------- |
| Accession                  | Left to right       | `a.b`             |
| Grouping                   |                     | `(a + b) * c`     |
| Unary negation/logical not | Right to left       | `-a`, `not a`     |
| Multiplication/division    | Left to right       | `a * b`, `a / b`  |
| Addition/subtraction       | Left to right       | `a + b`, `a - b`  |
| Comparison                 | Left to right       | `a < b`, `a >= b` |
| Equality                   | Require parentheses | `a == b`          |
| Logical and                | Left to right       | `a and b`         |
| Logical exclusive or (xor) | Left to right       | `a xor b`         |
| Logical or                 | Left to right       | `a or b`          |
| Ranges                     | Require parentheses | `a to b`          |
| Assignment                 | Right to left       | `a = b`           |

References:
- [JavaScript operator precedence](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Operators/Operator_precedence)
- [Rust expression precedence](https://doc.rust-lang.org/reference/expressions.html)
