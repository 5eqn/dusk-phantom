use dusk_phantom::lang::{run, Resource, Value};
use realfft::num_complex::Complex32;

#[test]
fn test_lp() {
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
    let result = code_value.collect(0..len, &resource);
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