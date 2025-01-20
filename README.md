# Makovi - Yet another language
Makovi is one of my free time black holes.
Here I've defined the major whats and whys of the language.
The current implementation is not complete and might never be.

## Purpose
Learning, curiosity, research and personal use

## Goals
- High performance (JIT / AOT)
- Frictionless (opt in types, consistent/simple syntax, quick to write)
- Sandboxed scripting language

# Language Features

Variable definition and assignment
```c
coins = 0
interests = 0.5 * (x+y)
title = "Makovi 101"
```
The compiler infers the type it based on the value and other factors
to squeeze runtime performance, but it can be set explicitly.
```c
coins: int = 0
average: float = 0.5 * (x+y)
title: string = "Makovi 101"
```

One can even specify more specific types
```c
coins: u8 = 0
average: f64 = 0.5 * (x+y)
title: u8[] = "Makovi 101"
```

Function declaration
```c
add = fn(a, b)
    return a + b

// One liners can also be written like this
add = fn(a, b) a + b
sub = fn(a, b) a - b
mul = fn(a, b) a * b
```

Arrays are growable if the size is omitted
```c
samples = [4, 2, 6, 3, 5] // type is int[]
samples.push(9)

samples: int[5] = [4, 2, 6, 3, 5]
samples.push(9) // this gives ERROR
```

String literals are powerful
- Embed code inside a string with `{code}`
- Alternatively, define the string as raw to not look of `{code}`
- Use 3 or more quotes to write quotes inside the string
- Multiline indented strings
```javascript
text = "1 + 2 = {1 + 2}" // 1 + 2 = 3
text = raw"1 + 2 = {1 + 2}" // 1 + 2 = {1 + 2}

text = """My "quoted" minion's""" //  My "quoted" minions's 

text = """
    Multiline string ignores the nesting indentation.
    You can write """""" here since this multiline string will only close
    with a triple """ in a new line
    """ // Multiline ... indentation.\n\n ..."""""" ... line
```

## Generics / Templates / Polymorphic procedures
`distance` is a generic function that accepts any two inputs
that have x and y components that can be subtracted
```javascript
distance = fn(point_a, point_b)
    x = point_a.x - point_b.x
    y = point_a.y - point_a.y
    return sqrt(x^2 + y^2)

distance({x = 2, y = 4}, {x = 2, y = 6}) // int 2
distance({x = 2., y = 4.}, {x = 2., y = 6.2}) // 2.2
```

## Lambdas
A lambda is not different from a simple function
```c
data = [1, 2, 3, 4, 5].filter(fn(x) x > 2)
```

This reduces friction compared to other languages
where lambda function syntax is different from its class method syntax,
which is itself different from its global function syntax.

## Structs / Classes / Type definitions
`Position` is a generic structure with an `add` method.
```javascript
Position = type { x, y }

Position.add = fn(rhs: Position)
    return Position { x = x + rhs.x, y = y + rhs.y }

a = Position { x = 1, y = 2 }
b = Position { x = 3, y = 4 }
print("{a + b}") // Position { x = 4, y = 6 }

a = { x = 1, y = 2 }
b = { x = 3, y = 4 }
print("{a + b}") // Error since a and b are not Positions
```

More complex types can be defined in multiple line as so
```javascript
Shop = type
    stock: string[]
    clients: string[]
    days_since_last: uint

my_shop_data = Shop
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
```javascript
compute = async(expensive_computation)
sum = async(fn() 3 + 1)
file = async(fn() read_file("b.txt"))
```

Now, everything will be as usual.
The big difference is that instead of waiting for the value to be calculated in the declaration,
this waiting will be done when the variable is used/needed.
```javascript
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
- Garbage collector (Go / JavaScript) - High memory overhead with periodic freezes of ~100 microseconds
- Reference counting (Swift) - High performance overhead due to many atomic increments / decrements
- Life times (Rust) - Restrictive model with high thinking time overhead that adds [color](https://journal.stuffwithstuff.com/2015/02/01/what-color-is-your-function) to functions and types.
- RAII / Free in destructor (C++) - It's not enough/possible in some complex scenarios

