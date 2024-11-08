# Syntax

## Literals

**Bools** are the entire set of literals allowed in boolean logic.  
```shell
bool -> "true" | "false"
```

**Numbers** are integer or floating point literals.  
```shell
digit -> [0-9]
number -> digit+ ("." digit+)?
```

**Text** is any amount of unicode characters surrounded by unescaped quotation marks. These literals may be prefixed with a single-character text modifier which specifies how the contained text should be interpreted.  
```shell
text_modifier -> ("f" | "r" | "c" | "b")
text -> text_modifier? '"' (\. | [^\])* '"'
```  

**Identifiers** define the names of entities. Valid identifiers are defined in terms of the [Unicode XID](https://www.unicode.org/reports/tr31/tr31-39.html#D1) start and continue sets. Identifiers are allowed to start with an underscore, which marks them as ignored for linting purposes.  
```
id -> (xid_start | "_") xid_continue*
```

## Expressions

**Blocks** are a series of expressions. Blocks evaluate to the final expression, or `unit` if the block is empty.
```
block -> expr*
```

**Expressions** evaluate to a value and always have a type. An expression can evaluate to the `unit` value.  
```
expr -> literal_expr
    | path_expr

literal_expr -> bool | number | text
path_expr -> id ("." id)*
```

**Operators** apply operations to their left and/or right operands. If multiple operator characters exist in a row, they are combined into one operator (such as in the case of "=="). Not all combinations of valid operator characters in a row are valid operators.  
```shell
op -> ("+" | "-" | "*" | "/" | "%" | "^" | "<" | ">" | "=" | "." | ":" | "!" | "?")*
```

**Types** are a separate namespace to identifiers which are used to verify program correctness before runtime. The language contains a few built-in types which form the basis for all other types.  
```shell
primitive_type -> "bool" | "number" | "text" | "unit"
type -> primitive_type | id
```

**Patterns** are used to destructure and bind values from structures.  
```
pattern -> basic_pattern ("or" basic_pattern)*
basic_pattern -> 
    literal_pattern 
  | identifier_pattern 
  | tuple_pattern
  | list_pattern
  | table_pattern
  | datatype_pattern
  | record_pattern 

literal_pattern -> bool | number | text | "unit"
identifier_pattern -> id
tuple_pattern -> "(" basic_pattern ("," basic_pattern)* ")"
list_pattern -> "[" basic_pattern ("," basic_pattern)* "]"
tuple_pattern -> "(" basic_pattern ("," basic_pattern)* ")"
table_pattern -> "{" basic_pattern ("," basic_pattern)* "}"
datatype_pattern -> "{" basic_pattern ("," basic_pattern)* "}"
record_pattern -> path "{" basic_pattern ("," basic_pattern)* "}"
```

**Binding** associates a pattern with an expression. Types are inferred, but can be specified.  
```
binding -> pattern (":" type)? "=" expr ("else" expr*)?
```

**Comments** are series of characters which are ignored by the compiler, delimited by the pound symbol and ended by a newline.  
```shell
comment -> "#" .* \n
```

**Spaces** are any characters which fall under general category of 'whitespace'. These are unilaterally ignored by the compiler, except during formatting.
```shell
space -> " " | "\r" | "\n" | "\t" 
```

**Lists** are growable homogenous ordered arrays of elements. Lists may only contain elements of the same type.   
```shell
list -> "[" expr ("," expr)* "]"
```

**Sequences** are finite heterogenous ordered lists. Sequences may contain elements of differing types, but may not dynamically grow in size.  
```shell
sequence -> "(" expr ("," expr)* ")"
```

**Tables** are growable, heterogenous associative arrays over expressions. Tables may contain elements of differing types.  
```shell
table_element -> expr = expr
table -> "[" table_element ("," table_element)* "]"
```

**If** executes one of two expressions based on some condition.  
```
if -> "if" expr "then" expr* ("else" expr*)? "end"
```

**Loop** continually executes some expressions.  
```
loop -> "loop" expr* "end"
```

**While** executes some expressions while the condition is true.  
```
while -> "while" expr "do" expr* "end"
```

**For** executes some expressions for each element in some list.  
```
for -> "for" pattern "in" expr "do" expr* "end"
```

**Datatypes** are tagged unions (sum types).  
```shell
datatype -> "type" id type (, type)* "end"
```

**Records** are a heterogenous collection of fields.  
```shell
field -> id ":" type
record -> "record" id field ("," field)* "end"
```

**Traits** define the functionality of a particular type.  
```shell
trait -> "trait" id "is" expr* "end"
trait_impl -> ("record" | "type") "has" id expr* "end"
```

## Associativity & Precedence

Operators/Expressions higher in the table have stronger binding.  
Operators with the same precedence share a row.

| Operator/Expression        | Associativity       | Examples         |
| -------------------------- | ------------------- | ---------------- |
| Grouping                   |                     | `(a + b) * c`    |
| Access                     | Left to right       | `a.b`            |
| Unary negation/logical not | Right to left       | `-a`, `not a`    |
| Multiplication/division    | Left to right       | `a * b`, `a / b` |
| Addition/subtraction       | Left to right       | `a + b`, `a - b` |
| Equality                   | Require parentheses | `a == b`         |
| Logical and                | Left to right       | `a and b`        |
| Logical exclusive or (xor) | Left to right       | `a xor b`        |
| Logical or                 | Left to right       | `a or b`         |
| Ranges                     | Require parentheses | `a to b`         |
| Assignment                 | Right to left       | `a = b`          |
| Concat                     | Left to right       | `a, b`           |

References:
- [JavaScript operator precedence](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Operators/Operator_precedence)
- [Rust expression precedence](https://doc.rust-lang.org/reference/expressions.html)
