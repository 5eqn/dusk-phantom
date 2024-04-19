use constant::*;
use lang::*;
use nih_plug::prelude::*;
use nih_plug_vizia::ViziaState;
use realfft::{num_complex::Complex32, ComplexToReal, RealFftPlanner, RealToComplex};
use std::sync::{Arc, Mutex};

mod constant;
mod editor;
mod lang;
mod assets;

/// This is mostly identical to the gain example, minus some fluff, and with a GUI.
pub struct DuskPhantom {
    params: Arc<PluginParams>,
    local_state: LocalState,
    plugin_state: Arc<PluginState>,
}

struct LocalState {
    /// Needed to normalize the peak meter's response based on the sample rate.
    peak_meter_decay_weight: f32,

    /// The algorithm for the FFT operation.
    r2c_plan: Arc<dyn RealToComplex<f32>>,

    /// The algorithm for the IFFT operation.
    c2r_plan: Arc<dyn ComplexToReal<f32>>,

    /// The output of our real->complex FFT.
    complex_fft_buffer: Vec<Complex32>,
    
    /// An adapter that performs most of the overlap-add algorithm for us.
    stft: util::StftHelper,
}

struct PluginState {
    debug: Mutex<String>,
    profiler: Mutex<String>,
    message: Mutex<String>,
    code_value: Mutex<Option<Value>>,
}

#[derive(Params)]
struct PluginParams {
    /// The editor state, saved together with the parameter state so the custom scaling can be
    /// restored.
    #[persist = "editor-state"]
    editor_state: Arc<ViziaState>,

    /// The code to compile
    #[persist = "code"]
    code: Arc<Mutex<String>>,
}

impl Default for DuskPhantom {
    fn default() -> Self {
        let mut planner = RealFftPlanner::new();
        let r2c_plan = planner.plan_fft_forward(FFT_WINDOW_SIZE);
        let c2r_plan = planner.plan_fft_inverse(FFT_WINDOW_SIZE);
        let complex_fft_buffer = r2c_plan.make_output_vec();
        Self {
            params: PluginParams::default().into(),
            local_state: LocalState {
                peak_meter_decay_weight: 1.0,
                stft: util::StftHelper::new(2, WINDOW_SIZE, FFT_WINDOW_SIZE - WINDOW_SIZE),
                r2c_plan,
                c2r_plan,
                complex_fft_buffer,
            },
            plugin_state: PluginState {
                debug: Mutex::new("".into()),
                profiler: Mutex::new("".into()),
                message: Mutex::new("".into()),
                code_value: Mutex::new(None),
            }.into(),
        }
    }
}

impl Default for PluginParams {
    fn default() -> Self {
        Self {
            editor_state: editor::default_state(),
            code: Arc::new(Mutex::new(DEFAULT_CODE.into())),
        }
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
        _audio_io_layout: &AudioIOLayout,
        buffer_config: &BufferConfig,
        context: &mut impl InitContext<Self>,
    ) -> bool {
        // After `PEAK_METER_DECAY_MS` milliseconds of pure silence, the peak meter's value should
        // have dropped by 12 dB
        self.local_state.peak_meter_decay_weight = 0.25f64
            .powf((buffer_config.sample_rate as f64 * PEAK_METER_DECAY_MS / 1000.0).recip())
            as f32;

        // Init code state
        let (msg, code) = match run(&self.params.code.lock().unwrap()) {
            Ok(val) => (format!("Compilation success: {}", val.pretty_term()), Some(val)),
            Err(err) => (err, None),
        };
        *self.plugin_state.message.lock().unwrap() = msg;
        *self.plugin_state.code_value.lock().unwrap() = code;

        // The plugin's latency consists of the block size from the overlap-add procedure and half
        // of the filter kernel's size (since we're using a linear phase/symmetrical convolution
        // kernel)
        context.set_latency_samples(self.local_state.stft.latency_samples() + (FILTER_WINDOW_SIZE as u32 / 2));

        // Initialization success
        true
    }

    fn reset(&mut self) {
        // Normally we'd also initialize the STFT helper for the correct channel count here, but we
        // only do stereo so that's not necessary. Setting the block size also zeroes out the
        // buffers.
        self.local_state.stft.set_block_size(WINDOW_SIZE);
    }

    fn process(
        &mut self,
        buffer: &mut Buffer,
        _aux: &mut AuxiliaryBuffers,
        _context: &mut impl ProcessContext<Self>,
    ) -> ProcessStatus {
        self.local_state.stft
            .process_overlap_add(buffer, 1, |_channel_idx, real_fft_buffer| {
                // Bypass if there is no code
                let Some(code_value) = self.plugin_state.code_value.lock().unwrap().clone() else {
                    return;
                };

                // Forward FFT, `real_fft_buffer` already is already padded with zeroes, and the
                // padding from the last iteration will have already been added back to the start of
                // the buffer
                let profile_1 = std::time::Instant::now();
                self.local_state.r2c_plan
                    .process_with_scratch(real_fft_buffer, &mut self.local_state.complex_fft_buffer, &mut [])
                    .unwrap();

                // Calculate new magnitudes
                let profile_2 = std::time::Instant::now();
                let len = self.local_state.complex_fft_buffer.len();
                let norms: Value = self.local_state.complex_fft_buffer
                    .iter()
                    .map(|c| c.norm())
                    .collect::<Vec<_>>()
                    .into();
                let result = code_value.apply(norms).collect(0..len);

                // Apply new magnitudes
                let profile_3 = std::time::Instant::now();
                for (i, val) in result.enumerate() {
                    let norm = match val {
                        Value::Float(f) => f,
                        _ => 0.0,
                    };
                    let old_norm = self.local_state.complex_fft_buffer[i].norm();
                    if old_norm == 0.0 {
                        self.local_state.complex_fft_buffer[i] = Complex32::new(norm, 0.0);
                    } else {
                        self.local_state.complex_fft_buffer[i] *= norm / old_norm;
                    }
                }

                // Inverse FFT back into the scratch buffer. This will be added to a ring buffer
                // which gets written back to the host at a one block delay.
                let profile_4 = std::time::Instant::now();
                self.local_state.c2r_plan
                    .process_with_scratch(&mut self.local_state.complex_fft_buffer, real_fft_buffer, &mut [])
                    .unwrap();

                // Store profiling result
                let profile = format!(
                    "Profile: {} ns, {} ns, {} ns, {} ns",
                    profile_1.elapsed().as_nanos(),
                    profile_2.elapsed().as_nanos(),
                    profile_3.elapsed().as_nanos(),
                    profile_4.elapsed().as_nanos(),
                );
                *self.plugin_state.profiler.lock().unwrap() = profile;

                // Store debug result
                *self.plugin_state.debug.lock().unwrap() = format!("complex_len = {}", len);
            });
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
