use aoc24_codspeed::day5::input_generator;
use aoc24_codspeed::day5::{part1, part2};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn criterion_benchmark(c: &mut Criterion) {
    let input = include_str!("../input/2024/day5.txt");
    c.bench_function("Run parts", |b| {
        b.iter(|| {
            let parsed = input_generator(input);
            black_box(part1(&parsed));
            black_box(part2(&parsed));
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
