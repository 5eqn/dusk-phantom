use core::panic;

use dusk_phantom::lang::run;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
fn eval_benchmark(c: &mut Criterion) {
    let len = 1024;
    let norms: Vec<f32> = (0..len).map(|_| rand::random::<f32>()).collect();

    c.bench_with_input(BenchmarkId::new("mutate", "1024"), &norms, |b, n| b.iter(|| {
        let code = "let lp: Float -> Float -> Float = (l: Float) => (i: Float) => if i < l then 1 else 0 in (f: Float -> Float) => (i: Float) => f(i) * lp(24)(i)";
        let Ok(code_value) = run(code) else {
            panic!("failed to run code");
        };
        let _ = code_value.apply(n.clone().into()).collect(0..len);
    }));
}

criterion_group!(benches, eval_benchmark);
criterion_main!(benches);