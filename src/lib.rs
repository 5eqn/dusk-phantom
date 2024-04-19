use atomic_float::AtomicF32;
use constant::{DEFAULT_CODE, PEAK_METER_DECAY_MS};
use lang::*;
use nih_plug::prelude::*;
use nih_plug_vizia::ViziaState;
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
}

struct PluginState {
    peak_meter: AtomicF32,
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
        Self {
            params: PluginParams::default().into(),
            local_state: LocalState {
                peak_meter_decay_weight: 1.0,
            },
            plugin_state: PluginState {
                peak_meter: AtomicF32::new(util::MINUS_INFINITY_DB),
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
        _context: &mut impl InitContext<Self>,
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

        // Initialization success
        true
    }

    fn process(
        &mut self,
        buffer: &mut Buffer,
        _aux: &mut AuxiliaryBuffers,
        _context: &mut impl ProcessContext<Self>,
    ) -> ProcessStatus {
        // Bypass if there is no code
        let Some(code_value) = self.plugin_state.code_value.lock().unwrap().clone() else {
            return ProcessStatus::Normal;
        };

        // Calculate array indexer
        let arr = [1.0, -1.0];
        let indexer: Arc<I2F> = Arc::new(move |x| {
            let i = x as usize;
            if i < arr.len() {
                arr[i]
            } else {
                0.0
            }
        });
        let product_array = code_value.apply(indexer.into());

        // Iterate all samples
        for (i, channel_samples) in buffer.iter_samples().enumerate() {
            // Calculate gain
            let Value::Float(gain) = product_array.clone().apply(Value::Int(i as i32)) else {
                panic!("Expected float");
            };

            // Apply gain
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
                let current_peak_meter = self
                    .plugin_state
                    .peak_meter
                    .load(std::sync::atomic::Ordering::Relaxed);
                let new_peak_meter = if amplitude > current_peak_meter {
                    amplitude
                } else {
                    current_peak_meter * self.local_state.peak_meter_decay_weight
                        + amplitude * (1.0 - self.local_state.peak_meter_decay_weight)
                };

                self.plugin_state
                    .peak_meter
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
