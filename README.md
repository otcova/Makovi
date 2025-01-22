# Makovi - Yet another language
Makovi is one of my free time black holes.
Here I've defined the major whats and whys of the language.
The current implementation is not complete and might never be.

## Purpose
Learning, curiosity, research and personal use

## Goals
- High performance (JIT / AOT)
- Frictionless (consistent/simple/minimal syntax, quick/easy to read/think)
- Sandboxed scripting language

# Language Features

Variable definition and assignment
```rust
let coins = 0
let average = (x + y) / 2.0
let title = "Makovi 101"
```
The compiler infers the type it based on the value and other factors
to squeeze runtime performance, but it can be set explicitly.
```rust
let coins: int = 0
let average: float = (x + y) / 2.0
let title: string = "Makovi 101"
```

One can even specify more specific types
```rust
let coins: i64 = 0
let average: f64 = (x + y) / 2.0
let title: u8[] = "Makovi 101"
```

Arrays are growable if the size is omitted
```rust
let samples = [4, 2, 6, 3, 5] // type is int[]
samples.push(9)

let samples: int[5] = [4, 2, 6, 3, 5]
samples.push(9) // this gives ERROR
```

String literals are powerful:
- Embed code inside a string with `{code}`
- Alternatively, define the string as raw to interpret `{` as a normal character.
- Use 3 or more quotes to write quotes inside the string
- Multiline indented strings
```rust
let text = "1 + 2 = {1 + 2}" // 1 + 2 = 3
let text = raw"1 + 2 = {1 + 2}" // 1 + 2 = {1 + 2}

let text = """My "quoted" minion's""" //  My "quoted" minions's 

let text = """
    Multiline string ignores the nesting indentation.
    You can write """""" here since this multiline string will only close
    with a triple """ in a new line
    """ // Multiline ... indentation.\n\n ..."""""" ... line
```

## Mutability
All variables have the same strict mutability.
```rust
let counter = 123
counter = 456   // Valid
counter = "789" // Error
```

A useful to overcome this strict mutability is to redifine the same variable.
```rust
let counter = 123
let counter = "789" // Valid
```

## Functions & Lambdas
Syntax Friction happens when a language interferes with the programmer’s workflow.
For example, C++ lambda function syntax is different from its class method syntax,
which is itself different from its global function syntax.


The function syntax in Makovi is consistent, and always has the form of
```rust
fn (a, b)
    return a + b

// One liners can also be written like this
fn (a, b) a + b
```

A function behaves as any other value.
```rust
let greet = fn (maybe_human)
    print("Hello {maybe_human}!")

[1, 2, 3].map(fn(x) x + 1) // -> [2, 3, 4]
```

This way lambdas and functions are alike in shape and form.


## Generics / Templates / Polymorphic procedures
`distance` is a generic function that accepts any two inputs
that have x and y components that can be subtracted
```rust
let distance = fn(point_a, point_b)
    let x = point_a.x - point_b.x
    let y = point_a.y - point_a.y
    return sqrt(x^2 + y^2)

distance({x = 2, y = 4}, {x = 2, y = 6}) // int 2
distance({x = 2., y = 4.}, {x = 2., y = 6.2}) // 2.2
```

## Lambdas
A lambda is not different from a simple function

data = [1, 2, 3, 4, 5].filter(fn(x) x > 2)
```

This reduces friction compared to other languages
where lambda function syntax is different from its class method syntax,
which is itself different from its global function syntax.

## Structs / Classes / Type definitions
`Position` is a generic structure with an `add` method.
```rust
let Position = type { x, y }

Position.add = fn(rhs: Position)
    return Position { x = x + rhs.x, y = y + rhs.y }

let a = Position { x = 1, y = 2 }
let b = Position { x = 3, y = 4 }
print("{a + b}") // Position { x = 4, y = 6 }

a = { x = 1, y = 2 }
b = { x = 3, y = 4 }
print("{a + b}") // Error since a and b are not Positions
```

More complex types can be defined in multiple line as so
```rust
let Shop = type
    stock: string[]
    clients: string[]
    days_since_last: uint

let my_shop_data = Shop
    stock = ["Pizza"]
    clients = ["Alice", "Bob"]
    days_since_last = 0
```

## Multithreading / Async / Futures / Promise / Tasks

Tasks / Futures / Promises are present in almost every modern programming language.
They allow to:
- Continue the computation while doing expensive system calls (read file / networking)
- Simplify the multithreading model

The drawbacks are that to use them, usually one needs to rewrite a lot of the code
(partially due to [red/blue functions](https://journal.stuffwithstuff.com/2015/02/01/what-color-is-your-function/))

To reduce friction the future/promise syntax does not need extra syntax.

To create an asynchronous task, simply pass the desired function to run to the `async` function.
```rust
let compute = async(expensive_computation)
let sum = async(fn() 3 + 1)
let file = async(fn() read_file("b.txt"))
```

Now, everything will be as usual.
The big difference is that instead of waiting for the value to be calculated in the declaration,
this waiting will be done when the variable is used/needed.
```rust
print("compute: {compute} sum: {sum}") // Here execution will pause till compute and sum are done
print("compute: {compute} sum: {sum}") // Now it's instantaneous

// Here the system could still be reading the file given that we sill haven't use
// it, there's no need to pause the program.
```

Synchronization mechanisms are to be specified yet. But the main idea is:
- Multithreading is not foolproof. It's impossible to solve all multithreading problems (race conditions, deadlocks...)
- The programmer must be able to opt out of multithreading concurrency, but continue using async (like JavaScript promise).

## Memory model
Here are the memory models of popular languages with the drawbacks.

- Garbage collector (Go / JavaScript) - High memory overhead with periodic freezes of ~100 microseconds long

- Reference counting (Swift) - High performance overhead due to many atomic increments / decrements

- Life times (Rust) - Restrictive model with high thinking time overhead that adds [color](https://journal.stuffwithstuff.com/2015/02/01/what-color-is-your-function) to functions and types.

- Ownership / RAII / Free in destructor (C++) - It's not enough/possible in some complex scenarios.

- None (C) - This implies the risk accessing uninitialized memory, which isn't very viable on a sandboxed language.
Also, this would bring a lot of friction due to usually having to think about memory managment.


By default, all arguments are passed by copy. This prevents the programmer from worrying if
the function will change something. Also, passing data as a copy is usually very efficient since
a COW mechanism is in place.
```rust
let increment = fn(x)
    x += 1
    print("x = {x}")

let value = 0

increment(value)
print("value = {value}")

// This prints:
// x = 1
// value = 0
```

When a reference is needed one can do it explicitly this way
```rust
increment(&value)
print("Value = {value}")

// This prints:
// x = 1
// value = 0
```
