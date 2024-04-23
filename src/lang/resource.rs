use realfft::num_complex::Complex32;

pub struct Resource<'a> {
    pub fft: &'a Vec<Complex32>,
    pub modulation: &'a Vec<f32>,
    pub beat: f64,
    pub second: f64,
}