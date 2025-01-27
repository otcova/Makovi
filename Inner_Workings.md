# Common modules
This are data structures and utilities used across layers

- Ast: Definition of the abstract syntax tree
- Error: Compilation error types

# Compilation
Compilation is the process of transforming source code into an executable.
Here are the steps and transformations done to the source code.
```
Source 1.-> Tokens 2.-> Statements 3.-> AST 4.-> IR 5.-> executable
```
Responsible modules
1. Lexer
1. Parser stage 1
1. Parser stage 2
1. Ir
1. Cranelift crate

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


# Runtime
The runtime is all the extra code that will run with the compiled source.
The built in runtime is the responsible to provide the implementation of the standard library.

The runtime can also be extended with additional customizable modules (see [next section](#runtime-modules)).
Since runtime modules provide extra types and functions,
the compiler needs to know in advance in which runtime the code is supposed to run.

Here is an example of who to compile and run code using the `DesktopRuntime`.

```rust
// Define the runtime
type DesktopRuntime = BaseRuntime<DesktopModule>;

// Compile targeting the runtime
let compiler = Compiler::default();
let executable = compiler.compile::<DesktopRuntime>(source_code);

// Instantiate the runtime and run the code
let runtime = DesktopRuntime::default();
runtime.run(executable);
```

> [!Note]
> The `DesktopModule` is a collection of [runtime modules](#runtime-modules) that gives access to
> the common desktop environments like the file system and networking.


## Runtime modules
A `RuntimeModule` is a group of functions and types that extends the Makovi's built-ins.
They are the way to let the scripting language interact with the environment (desktop / app / game).

The built in runtime has the most basic functionality which is always present in the language.
This includes basic functions and types like rounding and arrays.
While a `RuntimeModule` is an opt in customizable extension.

> [!Note]
> The built in runtime will always be sandboxed.
> Access to files, std::out or networking
> will always need to be enabled explicitly with some `RuntimeModules`
