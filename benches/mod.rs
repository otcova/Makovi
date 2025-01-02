#![feature(test)]

extern crate test;

use makovi::parser::{Ast, ParserContext};
use test::Bencher;

#[bench]
fn bench_big_function(b: &mut Bencher) {
    let source = include_str!("../code_samples/smallest_factor.run");
    let ast = Ast::default();
    let parser = ParserContext::default();

    b.iter(|| {
        ast.clear();
        parser.parse(source, &ast).unwrap()
    });
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
