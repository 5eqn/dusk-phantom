use dusk_phantom::lang::{run, Extern, Value};
use realfft::num_complex::Complex32;

#[test]
fn test_lp() {
    let len = 1024;
    let complex_vec: Vec<Complex32> = vec![Complex32::new(1.0, 0.0); len];
    let norms: &[Complex32] = &complex_vec;
    let code = "let lp: Float -> Float -> Float = (l: Float) => (i: Float) => if i < l then 1 else 0 in (f: Float -> (Float, Float)) => (i: Float) => (f(i).norm * lp(800)(i), f(i).angle).polar";
    let code_value = match run(code) {
        Ok(x) => x,
        Err(err) => panic!("failed to run code: {}", err),
    };
    let result = code_value.clone().apply(Value::Extern(Extern::ComplexArray(norms))).collect(0..len);
    for res in &result[0..800] {
        let Value::Tuple(xs) = res else {
            panic!("result is not complex: {}", res);
        };
        let Value::Float(re) = xs[0] else {
            panic!("real part is not float: {}", xs[0]);
        };
        let Value::Float(im) = xs[1] else {
            panic!("imaginary part is not float: {}", xs[1]);
        };
        assert_eq!(re, 1.0);
        assert_eq!(im, 0.0);
    }
    for res in &result[800..len] {
        let Value::Tuple(xs) = res else {
            panic!("result is not complex: {}", res);
        };
        let Value::Float(re) = xs[0] else {
            panic!("real part is not float: {}", xs[0]);
        };
        let Value::Float(im) = xs[1] else {
            panic!("imaginary part is not float: {}", xs[1]);
        };
        assert_eq!(re, 0.0);
        assert_eq!(im, 0.0);
    }
}