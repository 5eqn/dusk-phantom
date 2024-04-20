use crate::PluginState;
use crate::lang::*;
use crate::assets::*;
use nih_plug::prelude::Editor;
use nih_plug_vizia::vizia::prelude::*;
use nih_plug_vizia::widgets::*;
use nih_plug_vizia::{assets, create_vizia_editor, ViziaState, ViziaTheming};
use std::sync::Arc;

use crate::PluginParams;

#[derive(Lens)]
struct Data {
    params: Arc<PluginParams>,
    plugin_state: Arc<PluginState>,
}

// Define events to mutate the data
pub enum AppEvent {
    SetCode(String),
}

// Describe how the data is mutated in response to events
impl Model for Data {
    fn event(&mut self, _: &mut EventContext, event: &mut Event) {
        event.map(|app_event, _| match app_event {
            AppEvent::SetCode(code) => {
                *self.params.code.lock().unwrap() = code.clone();

                // TODO remove duplicate code
                let (msg, code) = match run(&self.params.code.lock().unwrap()) {
                    Ok(val) => (format!("Compilation success: {}", val.pretty_term()), Some(val)),
                    Err(err) => (err, None),
                };
                *self.plugin_state.message.lock().unwrap() = msg;
                *self.plugin_state.code_value.lock().unwrap() = code;
            }
        });
    }
}

// Makes sense to also define this here, makes it a bit easier to keep track of
pub(crate) fn default_state() -> Arc<ViziaState> {
    ViziaState::new(|| (800, 600))
}

pub(crate) fn create(
    params: Arc<PluginParams>,
    plugin_state: Arc<PluginState>,
    editor_state: Arc<ViziaState>,
) -> Option<Box<dyn Editor>> {
    create_vizia_editor(editor_state, ViziaTheming::Custom, move |cx, _| {
        assets::register_noto_sans_light(cx);
        assets::register_noto_sans_thin(cx);
        register_jb_mono_regular(cx);

        Data {
            params: params.clone(),
            plugin_state: plugin_state.clone(),
        }
        .build(cx);

        VStack::new(cx, |cx| {
            // Title
            Label::new(cx, "DuskPhantom GUI")
                .font_family(vec![FamilyOwned::Name(String::from(assets::NOTO_SANS))])
                .font_weight(FontWeightKeyword::Thin)
                .font_size(30.0)
                .height(Pixels(50.0))
                .top(Stretch(1.0));
            Label::new(cx, "Type Your EQ Here").bottom(Pixels(10.0));

            // Code area
            Textbox::new_multiline(cx, Data::params.map(|p| p.code.lock().unwrap().to_string()), true)
                .font_family(vec![FamilyOwned::Name(String::from(JB_MONO))])
                .width(Percentage(75.0))
                .height(Pixels(320.0))
                .bottom(Stretch(1.0))
                .on_edit(|cx, code| cx.emit(AppEvent::SetCode(code)));

            // Generic params
            GenericUi::new(cx, Data::params.map(|p| p.global.clone()))
            .bottom(Stretch(1.0));

            // Profiling message
            Label::new(
                cx,
                Data::plugin_state.map(|st| st.profiler.lock().unwrap().to_string()),
            )
            .width(Percentage(75.0))
            .bottom(Stretch(1.0));

            // Debug message
            Label::new(
                cx,
                Data::plugin_state.map(|st| st.debug.lock().unwrap().to_string()),
            )
            .width(Percentage(75.0))
            .bottom(Stretch(1.0));

            // Error message
            Label::new(
                cx,
                Data::plugin_state.map(|st| st.message.lock().unwrap().to_string()),
            )
            .width(Percentage(75.0))
            .height(Pixels(30.0))
            .bottom(Stretch(1.0));
        })
        .row_between(Pixels(0.0))
        .child_left(Stretch(1.0))
        .child_right(Stretch(1.0));

        ResizeHandle::new(cx);
    })
}
