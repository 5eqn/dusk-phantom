use core::panic;

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use dusk_phantom::lang::{run, Resource};
use realfft::num_complex::Complex32;

fn eval_benchmark(c: &mut Criterion) {
    let len = 1024;
    let complex: Vec<Complex32> = vec![Complex32::new(1.0, 0.0); len];
    let code = "let lp: Float -> Float -> Float = (l: Float) => (i: Float) => if i < l then 1 else 0 in (i: Float) => (fft(i).norm * lp(800)(i), fft(i).angle).polar";
    let code_value = match run(code) {
        Ok(x) => x,
        Err(err) => panic!("failed to run code: {}", err),
    };
    let resource = Resource { 
        fft: &complex,
        beat: 0.0,
        second: 0.0,
    };
    let data = (resource, code_value);

    c.bench_with_input(BenchmarkId::new("mutate", "1024"), &data, |b, (r, c)| {
        b.iter(|| {
            let _ = c.clone().collect(0..len, r);
        })
    });
}

criterion_group!(benches, eval_benchmark);
criterion_main!(benches);

