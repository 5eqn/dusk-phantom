use nih_plug_vizia::vizia::context::Context;

pub const JB_MONO: &str = "JetBrainsMono Nerd Font";

const JB_MONO_REGULAR: &[u8] = include_bytes!("../../res/jb_mono/JetBrainsMonoNerdFont-Regular.ttf");

pub fn register_jb_mono_regular(cx: &mut Context) {
    cx.add_font_mem(JB_MONO_REGULAR);
}
