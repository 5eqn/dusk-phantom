use dusk_phantom::lang::Value;

#[test]
fn test_lp() {
    let len = 1024;
    let norms_vec: Vec<f32> = vec![1.0; len];
    let norms: &[f32] = &norms_vec;
    let code = "let lp: Float -> Float -> Float = (l: Float) => (i: Float) => if i < l then 1 else 0 in (f: Float -> Float) => (i: Float) => f(i) * lp(800)(i)";
    let Ok(code_value) = dusk_phantom::lang::run(code) else {
        panic!("failed to run code");
    };
    let result = code_value.clone().apply(norms.into()).collect(0..len);
    for res in &result[0..800] {
        let Value::Float(x) = res else {
            panic!("result is not float: {}", res);
        };
        assert_eq!(*x, 1.0);
    }
    for res in &result[800..len] {
        let Value::Float(x) = res else {
            panic!("result is not float: {}", res);
        };
        assert_eq!(*x, 0.0);
    }
}