use sdl2::{render::Canvas, ttf::Font, video::Window, EventPump};

use crate::{
    decorations::Decorations,
    editor::Editor,
    input::event_handler,
    parse_config::Config,
    utils::hex_to_color,
};

pub struct Offset {
    pub x: u32,
    pub y: u32,
}

pub fn run(
    decorations: &mut Decorations,
    editor: &mut Editor,
    canvas: &mut Canvas<Window>,
    event_pump: &mut EventPump,
    config: &Config,
    font: &Font<'_, '_>,
) {
    loop {
        if event_handler(event_pump, editor) {
            render_window(canvas, decorations, editor, config, font);
        }
    }
}

fn render_window(
    canvas: &mut Canvas<Window>,
    decorations: &mut Decorations,
    editor: &mut Editor,
    config: &Config,
    font: &Font<'_, '_>,
) {
    canvas.set_draw_color(hex_to_color(&config.colors.background));
    canvas.clear();
    let offset = decorations.render_decorations(canvas, config, font);
    editor.render_loop(
        canvas,
        config,
        font,
        offset,
    );
    canvas.present();
}
