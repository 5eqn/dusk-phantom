use atomic_float::AtomicF32;
use constant::{DEFAULT_CODE, PEAK_METER_DECAY_MS};
use lang::*;
use nih_plug::prelude::*;
use nih_plug_vizia::ViziaState;
use std::sync::{atomic::AtomicI32, Arc, Mutex};

mod constant;
mod editor;
mod lang;

/// This is mostly identical to the gain example, minus some fluff, and with a GUI.
pub struct DuskPhantom {
    params: Arc<DuskPhantomParams>,

    /// Needed to normalize the peak meter's response based on the sample rate.
    peak_meter_decay_weight: f32,

    /// The current data for the peak meter. This is stored as an [`Arc`] so we can share it between
    /// the GUI and the audio processing parts. If you have more state to share, then it's a good
    /// idea to put all of that in a struct behind a single `Arc`.
    ///
    /// This is stored as voltage gain.
    peak_meter: Arc<AtomicF32>,

    /// Version of generated code.
    code_version: i32,

    /// Value of generated code.
    code_value: Value,
}

#[derive(Params)]
struct DuskPhantomParams {
    /// The editor state, saved together with the parameter state so the custom scaling can be
    /// restored.
    #[persist = "editor-state"]
    editor_state: Arc<ViziaState>,

    /// The code to compile
    #[persist = "code"]
    code: Arc<Mutex<String>>,

    /// Version of code
    #[persist = "code-version"]
    code_version: Arc<AtomicI32>,
}

impl Default for DuskPhantom {
    fn default() -> Self {
        Self {
            params: Arc::new(DuskPhantomParams::default()),

            peak_meter_decay_weight: 1.0,
            peak_meter: Arc::new(AtomicF32::new(util::MINUS_INFINITY_DB)),

            code_version: 0,
            code_value: Value::Float(1.0),
        }
    }
}

impl Default for DuskPhantomParams {
    fn default() -> Self {
        Self {
            editor_state: editor::default_state(),

            // See the main gain example for more details
            code: Arc::new(Mutex::new(DEFAULT_CODE.into())),
            code_version: Arc::new(AtomicI32::new(0)),
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
            self.peak_meter.clone(),
            self.params.editor_state.clone(),
        )
    }

    fn initialize(
        &mut self,
        _audio_io_layout: &AudioIOLayout,
        buffer_config: &BufferConfig,
        _context: &mut impl InitContext<Self>,
    ) -> bool {
        // After `PEAK_METER_DECAY_MS` milliseconds of pure silence, the peak meter's value should
        // have dropped by 12 dB
        self.peak_meter_decay_weight = 0.25f64
            .powf((buffer_config.sample_rate as f64 * PEAK_METER_DECAY_MS / 1000.0).recip())
            as f32;

        true
    }

    fn process(
        &mut self,
        buffer: &mut Buffer,
        _aux: &mut AuxiliaryBuffers,
        _context: &mut impl ProcessContext<Self>,
    ) -> ProcessStatus {
        // Update code value if code updated
        let new_version = self
            .params
            .code_version
            .load(std::sync::atomic::Ordering::Relaxed);
        if new_version > self.code_version {
            self.code_version = new_version;
            let code = run(&self.params.code.lock().unwrap()).unwrap_or(Value::Float(1.0));
            self.code_value = code;
        }

        // Calculate gain
        let gain: f32 = match self.code_value {
            Value::Float(x) => x,
            _ => 1.0,
        };

        // Iterate all samples
        for channel_samples in buffer.iter_samples() {
            let mut amplitude = 0.0;
            let num_samples = channel_samples.len();
            for sample in channel_samples {
                *sample *= gain;
                amplitude += *sample;
            }

            // To save resources, a plugin can (and probably should!) only perform expensive
            // calculations that are only displayed on the GUI while the GUI is open
            if self.params.editor_state.is_open() {
                amplitude = (amplitude / num_samples as f32).abs();
                let current_peak_meter = self.peak_meter.load(std::sync::atomic::Ordering::Relaxed);
                let new_peak_meter = if amplitude > current_peak_meter {
                    amplitude
                } else {
                    current_peak_meter * self.peak_meter_decay_weight
                        + amplitude * (1.0 - self.peak_meter_decay_weight)
                };

                self.peak_meter
                    .store(new_peak_meter, std::sync::atomic::Ordering::Relaxed)
            }
        }

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
