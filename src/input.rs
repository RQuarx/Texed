use crate::{editor::Editor, EXIT_SUCCESS};
use sdl2::{
    event::Event,
    keyboard::{KeyboardState, Scancode},
    EventPump,
};

pub fn event_handler(event_pump: &mut EventPump, editor: &mut Editor) -> bool {
    for event in event_pump.poll_iter() {
        match event {
            Event::Quit { .. } => std::process::exit(EXIT_SUCCESS),
            Event::TextInput { text, .. } => {
                editor.file_content[editor.cursor_pos.y].insert_str(editor.cursor_pos.x, &text);
                editor.cursor_pos.x += 1;
                return true;
            }
            Event::KeyDown { scancode, .. } => {
                return input_handler(editor, scancode.unwrap(), event_pump.keyboard_state());
            }
            Event::Window { .. } => {
                return true;
            }
            _ => {
                return false;
            }
        }
    }
    false
}

pub fn input_handler(
    editor: &mut Editor,
    scancode: Scancode,
    keyboard_state: KeyboardState,
) -> bool {
    let cursor = &mut editor.cursor_pos;
    match scancode {
        Scancode::Backspace => {
            if cursor.is_zero() {
                return true;
            } else if cursor.x > 0 {
                editor.file_content[cursor.y].remove(cursor.x - 1);
                cursor.x -= 1;
            } else if cursor.x == 0 && cursor.y > 0 {
                cursor.x = editor.file_content[cursor.y - 1].len();
                let line = &editor.file_content[cursor.y].clone();
                editor.file_content[cursor.y - 1].push_str(line);
                editor.file_content.remove(cursor.y);
                cursor.y -= 1;
            }
            true
        }
        Scancode::Right => {
            handle_right_arrow(editor, keyboard_state.is_scancode_pressed(Scancode::LCtrl))
        }
        Scancode::Left => {
            handle_left_arrow(editor, keyboard_state.is_scancode_pressed(Scancode::LCtrl))
        }
        _ => false,
    }
}

fn handle_right_arrow(editor: &mut Editor, is_ctrl_pressed: bool) -> bool {
    let line_len = editor.file_content[editor.cursor_pos.y].len();

    if editor.cursor_pos.x < line_len {
        if is_ctrl_pressed {
            if editor.file_content[editor.cursor_pos.y]
                .chars()
                .nth(editor.cursor_pos.x)
                .unwrap()
                == ' '
            {
                editor.cursor_pos.x += 1;
            }
            // Move to the next whitespace or end of line
            while editor.cursor_pos.x < line_len {
                let character = editor.file_content[editor.cursor_pos.y]
                    .chars()
                    .nth(editor.cursor_pos.x)
                    .unwrap();
                if character.is_whitespace() {
                    break;
                }
                editor.cursor_pos.x += 1;
            }
        } else {
            editor.cursor_pos.x += 1;
        }
    }

    // Move to the next line if at the end of the current line
    if editor.cursor_pos.x >= line_len {
        if editor.cursor_pos.y < editor.file_content.len() - 1 {
            editor.cursor_pos.y += 1;
            editor.cursor_pos.x = 0;
        }
    }
    true
}

fn handle_left_arrow(editor: &mut Editor, is_ctrl_pressed: bool) -> bool {
    if editor.cursor_pos.x > 0 {
        if is_ctrl_pressed {
            if editor.file_content[editor.cursor_pos.y]
                .chars()
                .nth(editor.cursor_pos.x - 1)
                .unwrap()
                == ' '
            {
                editor.cursor_pos.x -= 1;
            }
            // Move to the previous whitespace or start of line
            while editor.cursor_pos.x > 0 {
                let character = editor.file_content[editor.cursor_pos.y]
                    .chars()
                    .nth(editor.cursor_pos.x - 1)
                    .unwrap();
                if character.is_whitespace() {
                    break;
                }
                editor.cursor_pos.x -= 1;
            }
        } else {
            editor.cursor_pos.x -= 1;
        }
    } else if editor.cursor_pos.y > 0 {
        // Move to the end of the previous line
        editor.cursor_pos.y -= 1;
        editor.cursor_pos.x = editor.file_content[editor.cursor_pos.y].len();
    }
    true
}
