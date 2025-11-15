# parser_of_logic_expressions

Simple parser for propositional logic expressions.

This crate provides a parser for propositional (boolean) logic expressions such as `a & (b -> c)`. The grammar recognizes literals (atoms), grouping with parentheses or brackets, negation, and binary operators with precedence: AND, XOR, OR, IMPLICATION, EQUIVALENCE.

Grammar rules:

- `atom`:
	- Description: identifiers (letters, digits and underscores) and boolean literals `true` / `false`.
	- Examples: `A`, `var1`, `TRUE`, `false`.

- `primary`:
	- Description: either an `atom` or a grouped expression using parentheses/brackets: `(...)`, `[...]`, `{...}`.
	- Purpose: grouping forces the enclosed expression to be parsed first regardless of operator precedence.
	- Example: `(A | B) & C` — the grouping forces `A | B` to parse before `&`.

- `negation`:
	- Description: one or more logical negations `!` applied to a `primary`.
	- Precedence: highest; negation is applied before binary operators.
	- Associativity: negation nests right-to-left (e.g. `!!A` = `!(!A)`).
	- Examples: `!A`, `!!(B & C)`.

- `and` (conjunction):
	- Description: sequences of `negation` joined by `&` or Sheffer stroke `↑`.
	- Precedence: lower than `negation`, higher than `xor`.
	- Associativity: parsed left-associative in the grammar (e.g. `A & B & C` → `((A & B) & C)`).
	- Examples: `A & B`, `!X ↑ Y`.

- `xor` (exclusive disjunction):
	- Description: sequences of `and` separated by `^`.
	- Precedence: lower than `and`, higher than `or`.
	- Associativity: implemented as left-associative chaining (`A ^ B ^ C` → `((A ^ B) ^ C)`).
	- Examples: `A ^ B`, `(A & B) ^ C`.

- `or` (disjunction):
	- Description: sequences of `xor` separated by `|`.
	- Precedence: lower than `xor`, higher than `implication`.
	- Associativity: left-associative chaining (`A | B | C` → `((A | B) | C)`).
	- Examples: `A | B`, `A ^ B | C`.

- `implication` (implication):
	- Description: `or` expressions connected by the arrow `->`.
	- Precedence: lower than `or`, higher than equivalence `<->`.
	- Associativity: right-associative — `A -> B -> C` parses as `A -> (B -> C)`.
	- Examples: `A -> B`, `A | B -> C`.

- `expression` (equivalence / top-level):
	- Description: top-level expressions that connect `implication` terms with the `<->` operator (logical equivalence).
	- Associativity: parsed as a left-associative chain in the grammar (`implication ~ ("<->" ~ implication)*`), though equivalence is semantically symmetric.
	- Examples: `A <-> B`, `(A -> B) <-> (C | D)`.

- `file`:
	- Description: multiple `expression` entries, typically one per line

Precedence summary:

- `atom` / `primary`
- `negation` (`!`)
- `and` (`&`, `↑`)
- `xor` (`^`)
- `or` (`|`)
- `implication` (`->`, right-associative)
- `expression` / equivalence (`<->`)

# Usage

- `LogicParser` is the `pest` parser generated from `src/grammar.pest`.
- `make_parantecies(pair: Pair<Rule>) -> String` is a helper that builds a simple left-associative parenthesized representation from a parsed `expression` pair (used by the CLI).

# Error handling

- CLI functions use `anyhow` for context-rich errors.
- Library errors can be represented with `thiserror` types (see `LogicError`).

# Examples

Input (one expression per line):

```text
var1 ↑ !var2 & var3
[A ^ B]  <-> (TRUE & C_1)
```

A simple parenthesized representation (readable form):

```text
((var1 ↑ !var2) & var3)
(([A ^ B]) <-> (TRUE & C_1))
```
