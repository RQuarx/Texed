use std::{fs, path::PathBuf};

use sdl2::{rect::Point, render::Canvas, ttf::Font, video::Window};

use crate::{
    graphics::Offset,
    parse_config::{Config, CursorType},
    utils::{self, hex_to_color, render_text},
};

pub struct CursorPos {
    pub x: usize,
    pub y: usize,
}

impl CursorPos {
    pub fn is_zero(&self) -> bool {
        self.x == 0 && self.y == 0
    }
}

pub struct Editor {
    pub file_content: Vec<String>,
    pub cursor_pos: CursorPos,
    pub first_rendered_index: u32,
}

impl Editor {
    pub fn init(file_path: PathBuf) -> Self {
        let file_path = file_path.canonicalize().expect("File path doesn't exist");
        let file_content = fs::read_to_string(&file_path).expect("File does not exist!");
        let file_content: Vec<String> = file_content
            .lines()
            .map(|line| format!("{} ", line))
            .collect();

        Editor {
            file_content,
            cursor_pos: CursorPos { x: 0, y: 0 },
            first_rendered_index: 0,
        }
    }

    pub fn render_loop(&self, canvas: &mut Canvas<Window>, config: &Config, font: &Font<'_, '_>) {
        let mut y_offset: u32 = 0;

        for (i, line) in self.file_content.iter().enumerate() {
            let line_number_width = self.render_line_number(canvas, i, config, font, y_offset);
            self.render_text(
                canvas,
                line,
                config,
                font,
                Offset {
                    x: line_number_width,
                    y: y_offset,
                },
            );
            y_offset += font.height() as u32;
            self.render_cursor(canvas, config, font, &self.cursor_pos, line_number_width);
        }
    }

    fn render_line_number(
        &self,
        canvas: &mut Canvas<Window>,
        line_index: usize,
        config: &Config,
        font: &Font<'_, '_>,
        y_offset: u32,
    ) -> u32 {
        let line_number = if config.line_num.relative {
            (self.cursor_pos.y as i32 - line_index as i32).abs()
        } else {
            line_index as i32 + 1
        };

        let display_line_number = if config.line_num.zero_indexing {
            line_number
        } else {
            line_number + 1
        };

        let line_number_str = display_line_number.to_string();
        let max_digits = (self.file_content.len() as f32).log(10.0).ceil() as usize + 1;
        let padded_line_number_str = format!("{:>width$} ", line_number_str, width = max_digits);

        let color = if line_index == self.cursor_pos.y {
            hex_to_color(&config.colors.foreground)
        } else {
            hex_to_color(&config.colors.alt_foreground)
        };

        render_text(
            canvas,
            &padded_line_number_str,
            font,
            color,
            Offset { x: 0, y: y_offset },
        );

        font.size_of(&padded_line_number_str).unwrap().0 + 20
    }

    fn render_text(
        &self,
        canvas: &mut Canvas<Window>,
        text: &str,
        config: &Config,
        font: &Font<'_, '_>,
        offset: Offset,
    ) {
        utils::render_text(
            canvas,
            text,
            font,
            hex_to_color(&config.colors.foreground),
            offset,
        );
    }

    fn render_cursor(
        &self,
        canvas: &mut Canvas<Window>,
        config: &Config,
        font: &Font<'_, '_>,
        cursor_pos: &CursorPos,
        line_number_width: u32,
    ) {
        match config.cursor.cursor_type {
            CursorType::Beam => {
                self.render_beam_cursor(canvas, config, font, cursor_pos, line_number_width);
            },
            CursorType::Line => {
                self.render_line_cursor(canvas, config, font, cursor_pos, line_number_width);
            },
            CursorType::Hollow | CursorType::Box => todo!()
        }
    }

    fn render_beam_cursor(
        &self,
        canvas: &mut Canvas<Window>,
        config: &Config,
        font: &Font<'_, '_>,
        cursor_pos: &CursorPos,
        line_number_width: u32,
    ) {
        let (mut x, mut y) = font
            .size_of(self.file_content[cursor_pos.y].split_at(cursor_pos.x).0)
            .unwrap();
        x += line_number_width;
        y *= cursor_pos.y as u32 - self.first_rendered_index;

        let start_point = Point::from((x as i32, y as i32));
        let end_point = Point::from((x as i32, y as i32 + font.height()));

        canvas.set_draw_color(hex_to_color(&config.colors.cursor));
        let _ = canvas.draw_line(start_point, end_point);
    }

    fn render_line_cursor(
        &self,
        canvas: &mut Canvas<Window>,
        config: &Config,
        font: &Font<'_, '_>,
        cursor_pos: &CursorPos,
        line_number_width: u32,
    ) {
        let (mut x, mut y) = font
            .size_of(self.file_content[cursor_pos.y].split_at(cursor_pos.x).0)
            .unwrap();
        x += line_number_width;
        y *= cursor_pos.y as u32 - self.first_rendered_index;

        let start_point = Point::from((x as i32, y as i32));
        let end_point = Point::from((x as i32, y as i32 + font.height()));

        canvas.set_draw_color(hex_to_color(&config.colors.cursor));
        let _ = canvas.draw_line(start_point, end_point);
    }
}
