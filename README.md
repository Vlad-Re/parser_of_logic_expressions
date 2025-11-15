# parser_of_logic_expressions
parser_of_logic_expressions — a simple parser for propositional logic expressions.

The project reads lines with expressions (one expression per line), builds a syntactic tree, and can output a simplified, bracketed form of the expression.

## Technical description of parsing
Input: one or more text lines with logical expressions. Supported are atoms (names, true/false), parentheses `()`, `[]`, `{}`, unary negation `!`, binary operators `&`, `|`, `^`, `↑`, `->`, `<->` with defined priorities.
The parser is implemented using `pest`. The grammar rules are described in `src/grammar.pest` and include the following rules: `atom`, `primary`, `negation`, `and`, `xor`, `or`, `implication`, `expression`, `file`.

After parsing, the code can output a parse tree or perform a simple transformation — construct a left-sided parenthesized form of the expression for reading.

## How to use
The command `parse <FILE_PATH>` outputs a parse tree for each expression in the file.

The command `make_parantecies <IN_PATH> <OUT_PATH>` reads the input file and writes to the output file the strings with the parenthesized form of each expression.

## Grammar

```pest
COMMENT = _{ "//" ~ (!NEWLINE ~ ANY)* }
WHITESPACE = _{ " " | NEWLINE }

atom = @{ (^"true" | ^"false") | (('a'..'z' | 'A'..'Z') ~ (('a'..'z' | 'A'..'Z') | '0'..'9' | "_")*) }
primary = { atom | "(" ~ expression ~ ")" | "[" ~ expression ~ "]" | "{" ~ expression ~ "}" }
negation = { "!"* ~ primary }
and = { negation ~ (("&" | "↑") ~ negation)* }
xor = { and ~ ("^" ~ and)* }
or  = { xor ~ ("|" ~ xor)* }
implication = { or ~ ("->" ~ implication)? }

expression = { implication ~ ("<->" ~ implication)* }
file = { SOI ~ (expression)* ~ EOI }
```
