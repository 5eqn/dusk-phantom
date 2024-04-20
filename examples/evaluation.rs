fn main() {
    let len = 1024;
    let norms_vec: Vec<f32> = (0..len).map(|_| rand::random::<f32>()).collect();
    let norms: &[f32] = &norms_vec;
    let code = "let lp: Float -> Float -> Float = (l: Float) => (i: Float) => if i < l then 1 else 0 in (f: Float -> Float) => (i: Float) => f(i) * lp(24)(i)";
    let Ok(mut code_value) = dusk_phantom::lang::run(code) else {
        panic!("failed to run code");
    };
    for _ in 0..2000 {
        let _ = code_value.ref_apply(norms.into()).collect(0..len);
    }
}