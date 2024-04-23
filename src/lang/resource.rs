use realfft::num_complex::Complex32;

pub struct Resource<'a> {
    pub fft: &'a Vec<Complex32>,
}