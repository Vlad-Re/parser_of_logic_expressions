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

### Rules of procedure

`COMMENT = _{ "//" ~ (!NEWLINE ~ ANY)* }`
A comment starts with `"//"`, followed by any sequence of characters (`ANY`) until the end of the line (`!NEWLINE`).

`WHITESPACE = _{ " " | NEWLINE }`
Denotes spaces (`" "`) or a newline character between elements of an expression, essentially defining the gaps between symbols in an expression or the transition to the next expression.

### Expression Rules:
The order of rules in this category defines the precedence of operations.

`atom = @{ (^"true" | ^"false") | IDENT }`
An atom is the simplest unit of an expression. It can be a boolean literal (`true` or `false` in various cases) or an identifier (`IDENT`), which denotes a variable.

`primary = { atom | "(" ~ expression ~ ")" }`
A primary expression can be either an `atom`, or any `expression` enclosed in parentheses.

`negation = { "!"* ~ primary }`
Negation (`NOT`) consists of zero or more exclamation marks (`"!"`), followed by a `primary` expression.

`and = { negation ~ (("&" | "↑") ~ negation)* }`
Combines conjunction (`AND`) and Peirce's arrow (`NAND`). It consists of a `negation`, followed by zero or more repetitions of the AND operator (`"&"`) or the NAND operator (`"↑"`), each of which must be accompanied by another `negation` (i.e., an expression from higher precedence rules).

`xor = { and ~ ("^" ~ and)* }`
Exclusive OR (`XOR`) consists of `and`, followed by zero or more repetitions of the XOR operator (`"^"`), each of which must be accompanied by another `and`.

`or = { xor ~ ("|" ~ xor)* }`
Disjunction (`OR`) consists of `xor`, followed by zero or more repetitions of the OR operator (`"|"`), each of which must be accompanied by another `xor`.

`implication = { or ~ ("->" ~ implication)* }`
Implication (`Implies`) consists of `or`, followed by zero or more implication operators (`"->"`), each of which is accompanied by another `implication`. This structure typically indicates a right-associative order of execution for the `->` operator.

`expression = { implication ~ ("<->" ~ implication)* }`
The general expression consists of `implication`, followed by zero or more equivalence operators (`Bi-implies`, `<->`), each of which is accompanied by another `implication`.

`file = { SOI ~ (expression)* ~ EOI }`
This rule describes the structure of the entire input file. It starts with `SOI` (Start Of Input), may contain zero or more complete `expression`'s, and ends with `EOI` (End Of Input).
