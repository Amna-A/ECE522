extern crate rand;
use rand::prelude::Rng;
//Question 6: Bench your function from Question 2 using the following code:
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use part1::question2;

fn criterion_benchmark(c: &mut Criterion) {
    let mut rng = rand::thread_rng();
    let mut l: Vec<i64> = (0..10000).map(|_| {rng.gen_range(1, 10000)}).collect();
    c.bench_function("your function: ", |b| b.iter(|| question2::<i64>(black_box(&mut l))));
 
}
criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);


