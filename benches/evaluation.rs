use core::panic;

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use dusk_phantom::lang::{run, Extern, Value};
use realfft::num_complex::Complex32;

fn eval_benchmark(c: &mut Criterion) {
    let len = 1024;
    let complex_vec: Vec<Complex32> = vec![Complex32::new(1.0, 0.0); len];
    let norms: &[Complex32] = &complex_vec;
    let code = "let lp: Float -> Float -> Float = (l: Float) => (i: Float) => if i < l then 1 else 0 in (f: Float -> (Float, Float)) => (i: Float) => (f(i).norm * lp(800)(i), f(i).angle).polar";
    let code_value = match run(code) {
        Ok(x) => x,
        Err(err) => panic!("failed to run code: {}", err),
    };
    let data = (norms, code_value);

    c.bench_with_input(BenchmarkId::new("mutate", "1024"), &data, |b, (n, c)| {
        b.iter(|| {
            let _ = c.clone().apply(Value::Extern(Extern::ComplexArray(n))).collect(0..len);
        })
    });
}

criterion_group!(benches, eval_benchmark);
criterion_main!(benches);

