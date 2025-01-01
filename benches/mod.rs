#![feature(test)]

extern crate test;

use makovi::{
    ast::{ExprArena, FunctionAst},
    parser::ParserContext,
};
use test::Bencher;

#[bench]
fn bench_big_function(b: &mut Bencher) {
    let source = include_str!("../code_samples/smallest_factor.run");
    let arena = ExprArena::default();
    let parser = ParserContext::default();

    b.iter(|| parser.parse(source, &arena).unwrap());
}

/*

#[bench]
fn bench_big_function_multiple_times(b: &mut Bencher) {
    let source = code_samples::load_function_source("");
    b.iter(|| {
        for _ in 0..4 {
            parser::function(source);
        }
    });
}*/
