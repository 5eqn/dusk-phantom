/// The time it takes for the peak meter to decay by 12 dB after switching to complete silence.
pub const PEAK_METER_DECAY_MS: f64 = 150.0;

/// Default code.
pub const DEFAULT_CODE: &str = "1";

/// The order of FFT precision.
pub const FFT_ORDER: usize = 11;

/// The size of the windows we'll process at a time.
pub const WINDOW_SIZE: usize = 1 << FFT_ORDER;

/// The length of the filter's impulse response.
pub const FILTER_WINDOW_SIZE: usize = (1 << (FFT_ORDER - 1)) + 1;

/// The length of the FFT window we will use to perform FFT convolution. This includes padding to
/// prevent time domain aliasing as a result of cyclic convolution.
pub const FFT_WINDOW_SIZE: usize = WINDOW_SIZE + FILTER_WINDOW_SIZE - 1;

/// The gain compensation we need to apply for the STFT process.
pub const GAIN_COMPENSATION: f32 = 1.0 / FFT_WINDOW_SIZE as f32;