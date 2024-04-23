use constant::*;
use lang::*;
use nih_plug::prelude::*;
use nih_plug_vizia::ViziaState;
use realfft::{num_complex::Complex32, ComplexToReal, RealFftPlanner, RealToComplex};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

mod constant;
mod editor;
pub mod lang;

/// This is mostly identical to the gain example, minus some fluff, and with a GUI.
pub struct DuskPhantom {
    params: Arc<PluginParams>,
    local_state: LocalState,
    plugin_state: Arc<PluginState>,
}

struct LocalState {
    /// The algorithms for the FFT and IFFT operations, for each supported order so we can switch
    /// between them without replanning or allocations. Initialized during `initialize()`.
    plan_for_order: Option<[Plan; MAX_WINDOW_ORDER - MIN_WINDOW_ORDER + 1]>,

    /// The output of our real->complex FFT.
    complex_fft_buffer: Vec<Complex32>,

    /// An adapter that performs most of the overlap-add algorithm for us.
    stft: util::StftHelper,

    /// Contains a Hann window function of the current window length, passed to the overlap-add
    /// helper. Allocated with a `MAX_WINDOW_SIZE` initial capacity.
    window_function: Vec<f32>,
}

/// An FFT plan for a specific window size, all of which will be precomputed during initilaization.
struct Plan {
    /// The algorithm for the FFT operation.
    r2c_plan: Arc<dyn RealToComplex<f32>>,
    /// The algorithm for the IFFT operation.
    c2r_plan: Arc<dyn ComplexToReal<f32>>,
}

struct PluginState {
    debug: Mutex<String>,
    profiler: Mutex<String>,
    message: Mutex<String>,
    code_value: Mutex<Option<Value>>,
}

impl PluginState {
    pub fn init_code(&self, code_cache: Arc<Mutex<String>>) {
        // Read code from cache
        let code_str = code_cache.lock().unwrap().clone();

        // Compile code
        self.compile_code(code_str)
    }

    pub fn update_code(&self, profile: i32, code_cache: Arc<Mutex<String>>) {
        // Get file path
        let path =
            std::env::var("DUSK_PHANTOM_PATH").unwrap_or_else(|_| "/home/seqn/dft".to_string());
        let file_path = PathBuf::from(&path).join(format!("{}.dft", profile));
        let file_str = file_path.to_str().unwrap_or("unknown path").to_string();

        // Read code from file
        let code_str = match std::fs::read_to_string(file_path) {
            Ok(code_str) => code_str,
            Err(err) => {
                return *self.message.lock().unwrap() =
                    format!("Error reading from {}: {}", file_str, err);
            }
        };

        // Store code to cache
        *code_cache.lock().unwrap() = code_str.clone();

        // Compile code
        self.compile_code(code_str)
    }

    pub fn compile_code(&self, code_str: String) {
        // Evaluate and simplify code as a function
        let (msg, code) = match run(&code_str) {
            Ok(val) => (
                format!("Compilation success: {}", val.pretty_term()),
                Some(val),
            ),
            Err(err) => (err, None),
        };

        // Put message and code in memory
        *self.message.lock().unwrap() = msg;
        *self.code_value.lock().unwrap() = code;
    }
}

#[derive(Params)]
struct PluginParams {
    /// The editor state, saved together with the parameter state so the custom scaling can be
    /// restored.
    #[persist = "editor-state"]
    editor_state: Arc<ViziaState>,

    /// Cached code
    #[persist = "code"]
    code: Arc<Mutex<String>>,

    // NOTE: These `Arc`s are only here temporarily to work around Vizia's Lens requirements so we
    // can use the generic UIs
    /// Global parameters. These could just live in this struct but I wanted a separate generic UI
    /// just for these.
    #[nested(group = "global")]
    pub global: Arc<GlobalParams>,
}

#[derive(Params)]
pub struct GlobalParams {
    /// The size of the FFT window as a power of two (to prevent invalid inputs).
    #[id = "stft_window"]
    pub window_size_order: IntParam,

    /// The amount of overlap to use in the overlap-add algorithm as a power of two (again to
    /// prevent invalid inputs).
    #[id = "stft_overlap"]
    pub overlap_times_order: IntParam,

    /// The profile to use.
    #[id = "profile"]
    pub profile: IntParam,
}

impl Default for DuskPhantom {
    fn default() -> Self {
        Self {
            params: PluginParams::default().into(),
            local_state: LocalState {
                stft: util::StftHelper::new(2, MAX_WINDOW_SIZE, 0),
                plan_for_order: None,
                window_function: Vec::with_capacity(MAX_WINDOW_SIZE),
                complex_fft_buffer: Vec::with_capacity(MAX_WINDOW_SIZE / 2 + 1),
            },
            plugin_state: PluginState {
                debug: Mutex::new("".into()),
                profiler: Mutex::new("".into()),
                message: Mutex::new("".into()),
                code_value: Mutex::new(None),
            }
            .into(),
        }
    }
}

impl Default for PluginParams {
    fn default() -> Self {
        Self {
            editor_state: editor::default_state(),
            code: Arc::new(Mutex::new(DEFAULT_CODE.into())),
            global: Arc::new(GlobalParams::default()),
        }
    }
}

impl Default for GlobalParams {
    fn default() -> Self {
        GlobalParams {
            window_size_order: IntParam::new(
                "Window Size",
                DEFAULT_WINDOW_ORDER as i32,
                IntRange::Linear {
                    min: MIN_WINDOW_ORDER as i32,
                    max: MAX_WINDOW_ORDER as i32,
                },
            )
            .with_value_to_string(formatters::v2s_i32_power_of_two())
            .with_string_to_value(formatters::s2v_i32_power_of_two()),
            overlap_times_order: IntParam::new(
                "Window Overlap",
                DEFAULT_OVERLAP_ORDER as i32,
                IntRange::Linear {
                    min: MIN_OVERLAP_ORDER as i32,
                    max: MAX_OVERLAP_ORDER as i32,
                },
            )
            .with_value_to_string(formatters::v2s_i32_power_of_two())
            .with_string_to_value(formatters::s2v_i32_power_of_two()),
            profile: IntParam::new("Profile", 1, IntRange::Linear { min: 1, max: 16 }),
        }
    }
}

impl DuskPhantom {
    fn window_size(&self) -> usize {
        1 << self.params.global.window_size_order.value() as usize
    }

    fn overlap_times(&self) -> usize {
        1 << self.params.global.overlap_times_order.value() as usize
    }

    /// `window_size` should not exceed `MAX_WINDOW_SIZE` or this will allocate.
    fn resize_for_window(&mut self, window_size: usize) {
        // The FFT algorithms for this window size have already been planned in
        // `self.plan_for_order`, and all of these data structures already have enough capacity, so
        // we just need to change some sizes.
        self.local_state.stft.set_block_size(window_size);
        self.local_state.window_function.resize(window_size, 0.0);
        util::window::hann_in_place(&mut self.local_state.window_function);
        self.local_state
            .complex_fft_buffer
            .resize(window_size / 2 + 1, Complex32::default());
    }
}

impl Plugin for DuskPhantom {
    const NAME: &'static str = "Dusk Phantom";
    const VENDOR: &'static str = "5eqn";
    const URL: &'static str = env!("CARGO_PKG_HOMEPAGE");
    const EMAIL: &'static str = "subcat2077@outlook.com";

    const VERSION: &'static str = env!("CARGO_PKG_VERSION");

    const AUDIO_IO_LAYOUTS: &'static [AudioIOLayout] = &[
        AudioIOLayout {
            main_input_channels: NonZeroU32::new(2),
            main_output_channels: NonZeroU32::new(2),
            ..AudioIOLayout::const_default()
        },
        AudioIOLayout {
            main_input_channels: NonZeroU32::new(1),
            main_output_channels: NonZeroU32::new(1),
            ..AudioIOLayout::const_default()
        },
    ];

    const SAMPLE_ACCURATE_AUTOMATION: bool = true;

    type SysExMessage = ();
    type BackgroundTask = ();

    fn params(&self) -> Arc<dyn Params> {
        self.params.clone()
    }

    fn editor(&mut self, _async_executor: AsyncExecutor<Self>) -> Option<Box<dyn Editor>> {
        editor::create(
            self.params.clone(),
            self.plugin_state.clone(),
            self.params.editor_state.clone(),
        )
    }

    fn initialize(
        &mut self,
        audio_io_layout: &AudioIOLayout,
        _buffer_config: &BufferConfig,
        context: &mut impl InitContext<Self>,
    ) -> bool {
        // Initialize code
        self.plugin_state.init_code(self.params.code.clone());

        // This plugin can accept a variable number of audio channels, so we need to resize
        // channel-dependent data structures accordingly
        let num_output_channels = audio_io_layout
            .main_output_channels
            .expect("Plugin does not have a main output")
            .get() as usize;
        if self.local_state.stft.num_channels() != num_output_channels {
            self.local_state.stft =
                util::StftHelper::new(self.local_state.stft.num_channels(), MAX_WINDOW_SIZE, 0);
        }

        // Planning with RustFFT is very fast, but it will still allocate we we'll plan all of the
        // FFTs we might need in advance
        if self.local_state.plan_for_order.is_none() {
            let mut planner = RealFftPlanner::new();
            let plan_for_order: Vec<Plan> = (MIN_WINDOW_ORDER..=MAX_WINDOW_ORDER)
                .map(|order| Plan {
                    r2c_plan: planner.plan_fft_forward(1 << order),
                    c2r_plan: planner.plan_fft_inverse(1 << order),
                })
                .collect();
            self.local_state.plan_for_order = Some(
                plan_for_order
                    .try_into()
                    .unwrap_or_else(|_| panic!("Mismatched plan orders")),
            );
        }

        // Resize the window function and the FFT buffer to the new window size
        let window_size = self.window_size();
        self.resize_for_window(window_size);

        // Set the latency to the STFT latency
        context.set_latency_samples(self.local_state.stft.latency_samples());

        // Initialization success
        true
    }

    fn process(
        &mut self,
        buffer: &mut Buffer,
        _aux: &mut AuxiliaryBuffers,
        context: &mut impl ProcessContext<Self>,
    ) -> ProcessStatus {
        // Bypass if there is no code
        if self.plugin_state.code_value.lock().unwrap().is_none() {
            return ProcessStatus::Normal;
        };

        // If the window size has changed since the last process call, reset the buffers and chance
        // our latency. All of these buffers already have enough capacity so this won't allocate.
        let window_size = self.window_size();
        let overlap_times = self.overlap_times();
        if self.local_state.window_function.len() != window_size {
            self.resize_for_window(window_size);
            context.set_latency_samples(self.local_state.stft.latency_samples());
        }

        // These plans have already been made during initialization we can switch between versions
        // without reallocating
        let fft_plan = &mut self.local_state.plan_for_order.as_mut().unwrap()
            [self.params.global.window_size_order.value() as usize - MIN_WINDOW_ORDER];

        // The overlap gain compensation is based on a squared Hann window, which will sum perfectly
        // at four times overlap or higher. We'll apply a regular Hann window before the analysis
        // and after the synthesis.
        let gain_compensation: f32 =
            ((overlap_times as f32 / 4.0) * 1.5).recip() / window_size as f32;

        // We'll apply the square root of the total gain compensation at the DFT and the IDFT
        // stages. That way the compressor threshold values make much more sense.
        let input_gain = gain_compensation.sqrt();
        let output_gain = gain_compensation.sqrt();

        self.local_state.stft.process_overlap_add(
            buffer,
            overlap_times,
            |_channel_idx, real_fft_buffer| {
                // Get the code value again in case it changed during the last process call
                let profile_0 = std::time::Instant::now();
                let Some(code_value) = self.plugin_state.code_value.lock().unwrap().clone() else {
                    return;
                };

                // We'll window the input with a Hann function to avoid spectral leakage. The input gain
                // here also contains a compensation factor for the forward FFT to make the compressor
                // thresholds make more sense.
                for (sample, window_sample) in real_fft_buffer
                    .iter_mut()
                    .zip(self.local_state.window_function.iter())
                {
                    *sample *= window_sample * input_gain;
                }

                // Forward FFT, `real_fft_buffer` already is already padded with zeroes, and the
                // padding from the last iteration will have already been added back to the start of
                // the buffer
                let profile_1 = std::time::Instant::now();
                fft_plan
                    .r2c_plan
                    .process_with_scratch(
                        real_fft_buffer,
                        &mut self.local_state.complex_fft_buffer,
                        &mut [],
                    )
                    .unwrap();

                // Collect result as array
                let profile_4 = std::time::Instant::now();
                let len = self.local_state.complex_fft_buffer.len();
                let res = Resource {
                    fft: &self.local_state.complex_fft_buffer,
                };
                let result = code_value.collect(0..len, &res);

                // Apply new magnitudes
                let profile_5 = std::time::Instant::now();
                for (val, complex) in result
                    .into_iter()
                    .zip(&mut self.local_state.complex_fft_buffer) {
                    *complex = val.into();
                }

                // Remove extreme value
                self.local_state.complex_fft_buffer[0] = Complex32::default();
                self.local_state.complex_fft_buffer[len - 1] = Complex32::default();

                // Inverse FFT back into the scratch buffer. This will be added to a ring buffer
                // which gets written back to the host at a one block delay.
                let profile_6 = std::time::Instant::now();
                fft_plan
                    .c2r_plan
                    .process_with_scratch(
                        &mut self.local_state.complex_fft_buffer,
                        real_fft_buffer,
                        &mut [],
                    )
                    .unwrap();

                // Apply the window function once more to reduce time domain aliasing. The gain
                // compensation compensates for the squared Hann window that would be applied if we
                // didn't do any processing at all as well as the FFT+IFFT itself.
                let profile_7 = std::time::Instant::now();
                for (sample, window_sample) in real_fft_buffer
                    .iter_mut()
                    .zip(self.local_state.window_function.iter())
                {
                    *sample *= window_sample * output_gain;
                }

                // Store profiling result
                let profile = format!(
                    "Profile: {} us, {} us, {} us, {} us, {} us, {} us",
                    profile_0.elapsed().as_micros(),
                    profile_1.elapsed().as_micros(),
                    profile_4.elapsed().as_micros(),
                    profile_5.elapsed().as_micros(),
                    profile_6.elapsed().as_micros(),
                    profile_7.elapsed().as_micros(),
                );
                *self.plugin_state.profiler.lock().unwrap() = profile;

                // Store debug result
                *self.plugin_state.debug.lock().unwrap() = format!("complex_len = {}", len);
            },
        );
        ProcessStatus::Normal
    }
}

impl ClapPlugin for DuskPhantom {
    const CLAP_ID: &'static str = "com.your-domain.dusk-phantom";
    const CLAP_DESCRIPTION: Option<&'static str> = Some("Make unique sounds.");
    const CLAP_MANUAL_URL: Option<&'static str> = Some(Self::URL);
    const CLAP_SUPPORT_URL: Option<&'static str> = None;
    const CLAP_FEATURES: &'static [ClapFeature] = &[
        ClapFeature::AudioEffect,
        ClapFeature::Stereo,
        ClapFeature::Mono,
        ClapFeature::Utility,
    ];
}

impl Vst3Plugin for DuskPhantom {
    const VST3_CLASS_ID: [u8; 16] = *b"DuskPhantom55555";
    const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] =
        &[Vst3SubCategory::Fx, Vst3SubCategory::Tools];
}

nih_export_clap!(DuskPhantom);
nih_export_vst3!(DuskPhantom);
