use sdl2::{rect::Rect, render::Canvas, ttf::Font, video::Window};

use crate::{
    graphics::Offset,
    parse_config::Config,
    utils::{self, hex_to_color},
};

pub struct Decorations {
    file_name: String,
}

impl Decorations {
    pub fn init(file_name: &str) -> Self {
        Decorations { file_name: file_name.to_string() }
    }

    pub fn render_decorations(
        &self,
        canvas: &mut Canvas<Window>,
        config: &Config,
        font: &Font<'_, '_>,
    ) -> Offset {
        let rect = Rect::new(0, 0, canvas.logical_size().0, font.height() as u32 + 10);
        canvas.draw_rect(rect).unwrap();

        canvas.set_draw_color(hex_to_color(&config.colors.foreground));

        let text_y: u32 = (rect.height() as u32 - font.height() as u32) / 2;

        utils::render_text(
            canvas,
            &self.file_name,
            font,
            hex_to_color(&config.colors.foreground),
            Offset { x: 10, y: text_y },
        );

        Offset {
            x: 0,
            y: rect.height() as u32,
        }
    }
}
