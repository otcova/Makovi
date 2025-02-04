# Compilation
Compilation is the process of transforming source code into an executable. Here are the steps and transformations done to the source code.

The compilations is the execution of the following stages:
1. Lexer
1. Parser
1. Type Inference
1. IrGeneration

> [!Note]
> For optimization reasons, the stages might not be computed in a sequential order. For example, the lexer transforms the (`soruce_code: &str` into a `tokens: Iter<Token>` which is lazy. Meaning that each token will be computed when the parser requests it.

Let's look an example with a simple top level function and a function call.
```rust
fn greet_pair(x)
    if x % 2 == 0
        print("Hello {x}")

greet_pair(4)
```

## Stage 1 - Lexer
The source code is converted into a stream of known tokens like so:
`KeywordFn`, `Identifier`, `BracketOpen`, `Identifier`, `BracketClose`, `NewLine` ...

The lexer also gives additional metadata to each token:
- **Line & column** - needed when reporting compilation errors
- **String slice** - needed to read the value of an Identifier, or a literal.
- **Nesting** - needed to nest code

It is mainly implemented with the crate [logos](https://github.com/maciejhirsz/logos) which provides optimizations like jump tables that are set up at compile time.

## Stage 2 - Parser
All statements are transformed into simple instructions. This step will convert the token stream into nested simple instructions. To divide the expressions like `x % 2 == 0` into multiple instructions, temporal variables are assigned.

The possible instructions are the following:
- **Function definition:** `fn "function_name" <number of parameters>`
- **Function call:** `vX = function(arg0, arg1, ...)`, `vX = vY.method(arg0, arg1, ...)`
- **Assignation:** `vX = value`
- **Control flow:**
`if vX`, `else if vX`, `else`, `while vX`, `for vX in vY`, `return vX`
```rust
fn "greet_pair" (v0)
    v1 = %(v0, 2)
    v2 = ==(v1, 0)

    if v2
        v3 = "Hello "
        v3.push(v0)
        print(v3)

greet_pair(4)
```

## Stage 3 - Type inference
The Type inference stage will assign types to all the variables and will also instantiate all the generic functions.

The type inference consists in iterating the instructions inside a function/module scope from top to bottom, and do the following.

- **Case function definition:**
The instruction is temporarily skipped. This is necessary because in the case of a generic function, we will need to know which types are being passed.

- **Case assignation:**
This one is straight forward. The variable is assigned the types of the value.

- **Case function call:**
A new type inference instance is started with the context of the called function and the types of the arguments. Once the type inference is done, we will know the return type of the function and therefore, assign it to the return value (if present).

- **Case control flow:**
Here we only need to validate that the provided variable has the correct type. In case of the `if vX` and `while vX`, we ensure that vX is a boolean.

In our example, the first analyzed instruction is `greet_pair(4)`. From there, we analyze `greet_pair<int>` and obtain the following result:

```rust
fn "greet_pair" (v0: int) -> null
    v1: int = %(v0, 2)
    v2: bool = ==(v1, 0)

    if v2
        v3: string = "Hello "
        v3.push(v0)
        print(v3)

greet_pair(4)
```
> [!Note]
> if we had a second call to the function with a different type like `greet_pair(3.4)`,
> we would have ended up with two instances of the `greet_pair` function.
> The `greet_pair<int>` and the `greet_pair<float>`.


## Stage 4 - IrGeneration
Now that we have simple typed instructions, the code is transformed into an intermediate representation (IR). The IR is a generic assembly. This assembly will be later on passed to the back-end to do the final optimizations and translation to the specific instruction set of the targeted machine.

# Runtime
The runtime is all the extra code that will run with the compiled source. The built in runtime is the responsible to provide the implementation of the standard library.

The runtime can also be extended with additional customizable modules (see [next section](#runtime-modules)). Since runtime modules provide extra types and functions, the compiler needs to know in advance in which runtime the code is supposed to run.

Here is an example of who to compile and run code using the `DesktopModule`.

```rust
// Initialize the runtime module
let desktop_module = DesktopModule::default();

// Instantiate the runtime and run the code
let compiler = Compiler::new(desktop_module);

let executable = compiler.compile(source_code);
compiler.run(executable);
```

> [!Note]
> The `DesktopModule` is a collection of [runtime modules](#runtime-modules) that gives access to the common desktop environments like the file system and networking.


## Runtime modules
A `RuntimeModule` is a group of functions and types that extends the Makovi's built-ins. They are the way to let the scripting language interact with the environment (desktop / app / game).

The built in runtime has the most basic functionality which is always present in the language. This includes basic functions and types like rounding and arrays. While a `RuntimeModule` is an opt in customizable extension.

> [!Note]
> The built in runtime will always be sandboxed. Access to files, std::out or networking will always need to be enabled explicitly with some `RuntimeModules`
