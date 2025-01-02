use criterion::{criterion_group, criterion_main, Criterion};
use phlex_emmet_ls::parser::parse;
use phlex_emmet_ls::rendering::*;
use std::hint::black_box;

fn parsing(c: &mut Criterion) {
    let mut group = c.benchmark_group("Parsing");
    let abbr = "div+div>p>span+em";
    let with_classes = "div.flex.flex-row.container";
    group.bench_function(abbr, |b| b.iter(|| parse(abbr).unwrap()));
    group.bench_function(with_classes, |b| b.iter(|| parse(with_classes).unwrap()));
}

fn render(c: &mut Criterion) {
    let mut group = c.benchmark_group("Render");
    let abbr = "div+div>p>span+em";
    let ast = parse(abbr).unwrap();
    group.bench_function("Render", |b| b.iter(|| Renderer::render(black_box(&ast))));
    group.finish();
}
criterion_group!(benches, parsing, render);
criterion_main!(benches);
