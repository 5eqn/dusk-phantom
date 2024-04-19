/// The time it takes for the peak meter to decay by 12 dB after switching to complete silence.
pub const PEAK_METER_DECAY_MS: f64 = 150.0;

/// Default code.
pub const DEFAULT_CODE: &str = "1";

/// The size of the windows we'll process at a time.
pub const WINDOW_SIZE: usize = 64;

/// The length of the filter's impulse response.
pub const FILTER_WINDOW_SIZE: usize = 33;

/// The length of the FFT window we will use to perform FFT convolution. This includes padding to
/// prevent time domain aliasing as a result of cyclic convolution.
pub const FFT_WINDOW_SIZE: usize = WINDOW_SIZE + FILTER_WINDOW_SIZE - 1;