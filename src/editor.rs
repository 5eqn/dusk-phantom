use crate::DuskPhantomState;
use nih_plug::prelude::{util, Editor};
use nih_plug_vizia::vizia::prelude::*;
use nih_plug_vizia::widgets::*;
use nih_plug_vizia::{assets, create_vizia_editor, ViziaState, ViziaTheming};
use std::sync::atomic::Ordering;
use std::sync::Arc;
use std::time::Duration;

use crate::DuskPhantomParams;

#[derive(Lens)]
struct Data {
    params: Arc<DuskPhantomParams>,
    plugin_state: Arc<DuskPhantomState>,
    display_code: String,
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
                self.params.code_version.fetch_add(1, Ordering::Relaxed);
                let mut loaded_code = self.params.code.lock().unwrap();
                *loaded_code = code.clone();
                self.display_code = code.clone();
            }
        });
    }
}

// Makes sense to also define this here, makes it a bit easier to keep track of
pub(crate) fn default_state() -> Arc<ViziaState> {
    ViziaState::new(|| (800, 600))
}

pub(crate) fn create(
    params: Arc<DuskPhantomParams>,
    plugin_state: Arc<DuskPhantomState>,
    editor_state: Arc<ViziaState>,
) -> Option<Box<dyn Editor>> {
    create_vizia_editor(editor_state, ViziaTheming::Custom, move |cx, _| {
        assets::register_noto_sans_light(cx);
        assets::register_noto_sans_thin(cx);

        Data {
            params: params.clone(),
            plugin_state: plugin_state.clone(),

            // Load initial code from params, clone it, and free the lock
            display_code: params.code.lock().unwrap().clone(),
        }
        .build(cx);

        VStack::new(cx, |cx| {
            // Title
            Label::new(cx, "DuskPhantom GUI")
                .font_family(vec![FamilyOwned::Name(String::from(assets::NOTO_SANS))])
                .font_weight(FontWeightKeyword::Thin)
                .font_size(30.0)
                .height(Pixels(50.0))
                .child_top(Stretch(1.0))
                .child_bottom(Pixels(0.0));
            Label::new(cx, "DuskPhantom").bottom(Stretch(1.0));

            // Code area
            Textbox::new_multiline(cx, Data::display_code, true)
                .width(Percentage(80.0))
                .height(Pixels(360.0))
                .bottom(Stretch(1.0))
                .on_edit(|cx, code| cx.emit(AppEvent::SetCode(code)));

            // Error message
            HStack::new(cx, |cx| {
                Label::new(
                    cx,
                    Data::params
                        .map(|param| format!("[{}]", param.code_version.load(Ordering::Relaxed))),
                )
                .width(Percentage(5.0))
                .right(Percentage(5.0));
                Label::new(
                    cx,
                    Data::plugin_state.map(|st| st.message.lock().unwrap().to_string()),
                )
                .width(Percentage(70.0));
            })
            .bottom(Stretch(1.0));

            // Peak meter
            PeakMeter::new(
                cx,
                Data::plugin_state
                    .map(|st| util::gain_to_db(st.peak_meter.load(Ordering::Relaxed))),
                Some(Duration::from_millis(600)),
            )
            .top(Pixels(10.0))
            .bottom(Stretch(1.0));
        })
        .row_between(Pixels(0.0))
        .child_left(Stretch(1.0))
        .child_right(Stretch(1.0));

        ResizeHandle::new(cx);
    })
}
