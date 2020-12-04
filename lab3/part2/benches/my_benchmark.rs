extern crate rand;
use rand::prelude::*;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use part2::question7;


fn criterion_benchmark(c: &mut Criterion) {
    let mut rng = rand::thread_rng();
    let mut l: Vec<i64> = (0..10000).map(|_| {rng.gen_range(1, 10000)}).collect();
    c.bench_function("your function: ", |b| b.iter(|| question7::<i64>(black_box(&mut l)))); 
}
criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

/*
Question 9: Compare between the two performance results obtained from Criterion crate from Q6 and Q8.
    - in part 1 the excution time was fluctuating around 52 ms 
    and in part 3 it's around 53 ms . so the performance was somewhat similar.

Question 10: What are “zero-cost abstractions” in Rust? Explain how this term applies to Q9.
    - zero-cost abstractions: wether the code is functional or not (with loops or with colsures) 
    it will not affect the performance.

    - This apply to question 9 in the sense that the performance is not that different 
    and it result in fairly similar assembly code.
*/