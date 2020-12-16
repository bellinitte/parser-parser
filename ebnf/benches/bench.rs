use criterion::{black_box, criterion_group, criterion_main, Criterion};
use ebnf::parse;

const GRAMMAR: &str = "
expression = term, { ('+' | '-'), term };
term       = factor, { ('*' | '/'), factor };
factor     = constant | variable | '(', expression, ')';
variable   = 'x' | 'y' | 'z';
constant   = digit, { digit };
digit      = '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9';
";

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("parse", |b| b.iter(|| parse(black_box(GRAMMAR))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
