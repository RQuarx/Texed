use sdl2::{
    rect::{Point, Rect},
    render::Canvas,
    ttf::Font,
    video::Window,
};

use crate::{editor::Editor, graphics::Offset, parse_config::Config, utils::hex_to_color};

pub struct Decorations {
    file_name: String,
}

impl Decorations {
    pub fn init(file_name: &str) -> Self {
        Decorations {
            file_name: file_name.to_string(),
        }
    }

    pub fn render_decorations(
        &self,
        canvas: &mut Canvas<Window>,
        config: &Config,
        font: &Font<'_, '_>,
    ) -> Offset {
        let rect = Rect::new(
            0,
            0,
            canvas.output_size().unwrap().0,
            font.height() as u32 + 10,
        );
        canvas.draw_rect(rect).unwrap();

        canvas.set_draw_color(hex_to_color(&config.colors.foreground));

        let text_y: u32 = (rect.height() as u32 - font.height() as u32) / 2;

        Editor::render_text(
            canvas,
            &self.file_name,
            hex_to_color(&config.colors.foreground),
            font,
            Offset { x: 10, y: text_y },
        );

        canvas.set_draw_color(hex_to_color(&config.colors.border));

        let _ = canvas.draw_line(
            Point::from((0, rect.height() as i32)),
            Point::from((
                font.size_of(&self.file_name).unwrap().0 as i32 + 10,
                rect.height() as i32,
            )),
        );

        Offset {
            x: 0,
            y: rect.height() as u32,
        }
    }
}
