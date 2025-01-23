#![allow(unused)]

enum Statement<'a> {
    Assign(&'a str),
    Return(),
    IfElse(),
    While(),
}
