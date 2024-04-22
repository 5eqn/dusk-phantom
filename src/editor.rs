use crate::PluginState;
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
    UpdateCode,
}

// Describe how the data is mutated in response to events
impl Model for Data {
    fn event(&mut self, _: &mut EventContext, event: &mut Event) {
        event.map(|app_event, _| match app_event {
            AppEvent::UpdateCode => {
                self.plugin_state.update_code(self.params.global.profile.value());
            }
        });
    }
}

// Makes sense to also define this here, makes it a bit easier to keep track of
pub(crate) fn default_state() -> Arc<ViziaState> {
    ViziaState::new(|| (600, 360))
}

pub(crate) fn create(
    params: Arc<PluginParams>,
    plugin_state: Arc<PluginState>,
    editor_state: Arc<ViziaState>,
) -> Option<Box<dyn Editor>> {
    create_vizia_editor(editor_state, ViziaTheming::Custom, move |cx, _| {
        assets::register_noto_sans_light(cx);
        assets::register_noto_sans_thin(cx);

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
                .top(Stretch(1.0))
                .bottom(Stretch(1.0));

            // Generic params
            GenericUi::new(cx, Data::params.map(|p| p.global.clone()))
            .bottom(Stretch(1.0));

            // Update code button
            Button::new(
                cx, 
                |cx| cx.emit(AppEvent::UpdateCode), 
                |cx| Label::new(cx, "Update Code")
            )
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
