use core::panic;

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use dusk_phantom::lang::run;

fn eval_benchmark(c: &mut Criterion) {
    let len = 1024;
    let norms_vec: Vec<f32> = (0..len).map(|_| rand::random::<f32>()).collect();
    let norms: &[f32] = &norms_vec;
    let code = "let lp: Float -> Float -> Float = (l: Float) => (i: Float) => if i < l then 1 else 0 in (f: Float -> Float) => (i: Float) => f(i) * lp(24)(i)";
    let Ok(code_value) = run(code) else {
        panic!("failed to run code");
    };
    let data = (norms, code_value);

    c.bench_with_input(BenchmarkId::new("mutate", "1024"), &data, |b, (n, c)| {
        b.iter(|| {
            let _ = c.clone().apply((*n).into()).collect(0..len);
        })
    });
}

criterion_group!(benches, eval_benchmark);
criterion_main!(benches);

