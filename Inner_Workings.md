
# Execution flow
| Module   | Generates              |
|--------|------------------------|
| Lexer  | `Iter<Token>`          |
| Parser | `Ast`                  |
| Ir     | `Ir`                   |
| Jit    | `Executable`           |

## Lexer
A one token lookahead mainly implemented with crate::logos which
does all of that heavy lifting at compile time
like lookup tables or jump tables optimizations.

## Parser
It's the big module that understands the code.
The main purpose is generating a correct AST.

Internally, the parser is divided into two stages.
1. Statement Parsing
2. Block Parsing

### Statement Parsing
The statement stage converts the Iter<Token> into a Iter<Statement>.
The purpose is to have a simple list of statements to be able to analyze without any context.
This way, the compiler is able to recover easily when there are syntax errors.

The statement stage will do all checks that do not need any kind of scope context.
Specifically, it mostly does Syntax Analysis.

A statement is considered a line of code. But sometimes this gets more complicated.
Each code block represents a single statement
```rust
let value = 1 + 2 +
    3 +  my_function()
```
```rust
let my_function = fn(a, b)
```
```rust
// Note that 'a + b' is considered an
// expression, not an statement.
let my_function = fn(a, b) a + b
```
```rust
if a == b
```
```rust
else
```

> [!Note]
> The statements inside the code block of `if`
> are not part of the `if` statement.
> And the same happens with `while`, `fn`, ...

### Block Parsing
The block parsing stage transforms the Iter<Statement> into an AST.
To do so, it has to:

- Check for correct nesting
- Check variables used inside it's scopes
- Track variables lifetimes (Used to optimize. Preventing inneceray copies or references)
- Type Inference

