use std::ops::DerefMut;

use crate::{
    editor::{Editor, EditorMode},
    utils::is_whitespace,
    EXIT_SUCCESS,
};
use sdl2::{
    event::Event,
    keyboard::{KeyboardState, Scancode},
    EventPump,
};

pub fn event_handler(event_pump: &mut EventPump, editor: &mut Editor) -> bool {
    let is_shift_pressed = event_pump
        .keyboard_state()
        .is_scancode_pressed(Scancode::LShift);
    let is_ctrl_pressed = event_pump
        .keyboard_state()
        .is_scancode_pressed(Scancode::LCtrl);
    let mut changed: bool = false;

    for event in event_pump.poll_iter() {
        match event {
            Event::Quit { .. } => std::process::exit(EXIT_SUCCESS),
            Event::TextInput { text, .. } => {
                if editor.editor_mode == EditorMode::Insert {
                    editor.file_content[editor.cursor_pos.y].insert_str(editor.cursor_pos.x, &text);
                    editor.cursor_pos.x += text.len();
                }
                changed = true;
            }
            Event::KeyDown { scancode, .. } => match editor.editor_mode {
                EditorMode::Insert => {
                    changed = input_handler_insert_mode(editor, scancode.unwrap(), is_ctrl_pressed);
                }
                EditorMode::Normal => {
                    changed = input_handler_normal_mode(
                        editor,
                        scancode.unwrap(),
                        is_ctrl_pressed,
                        is_shift_pressed,
                    );
                }
                _ => {}
            },
            Event::Window { .. } => changed = true,
            _ => continue,
        }
    }
    changed
}

fn input_handler_normal_mode(
    editor: &mut Editor,
    scancode: Scancode,
    is_ctrl_pressed: bool,
    is_shift_pressed: bool,
) -> bool {
    match scancode {
        Scancode::Right => {
            handle_right_arrow(editor, is_ctrl_pressed);
            return true;
        }
        Scancode::Left => {
            handle_left_arrow(editor, is_ctrl_pressed);
            return true;
        }
        Scancode::Semicolon => {
            if is_shift_pressed {
                editor.editor_mode = EditorMode::Command;
                return true;
            }
            false
        }
        Scancode::Insert | Scancode::I => {
            editor.editor_mode = EditorMode::Insert;
            false
        }
        _ => false,
    }
}

fn input_handler_insert_mode(
    editor: &mut Editor,
    scancode: Scancode,
    is_ctrl_pressed: bool,
) -> bool {
    match scancode {
        Scancode::Escape => {
            editor.editor_mode = EditorMode::Normal;
            return true;
        }
        Scancode::Backspace => handle_backspace(editor, is_ctrl_pressed),
        Scancode::Right => {
            handle_right_arrow(editor, is_ctrl_pressed);
            return true;
        }
        Scancode::Left => {
            handle_left_arrow(editor, is_ctrl_pressed);
            return true;
        }
        Scancode::Return => {
            handle_return(editor);
            return true;
        }
        _ => false,
    }
}

fn handle_return(editor: &mut Editor) {
    let current_line = &mut editor.file_content[editor.cursor_pos.y].clone();

    let (before_cursor, after_cursor) = current_line.split_at(editor.cursor_pos.x);

    editor.file_content[editor.cursor_pos.y] = before_cursor.to_string();

    editor
        .file_content
        .insert(editor.cursor_pos.y + 1, after_cursor.to_string());

    editor.cursor_pos.x = 0;
    editor.cursor_pos.y += 1;
}

fn handle_backspace(editor: &mut Editor, is_ctrl_pressed: bool) -> bool {
    // Do nothing if the cursor is at 0, 0
    if editor.cursor_pos.is_zero() {
        return false;
    }

    if is_ctrl_pressed {
        handle_ctrl_backspace(editor);
        return true;
    }

    // Merge line if the cursor x position is 0
    if editor.cursor_pos.x == 0 && editor.cursor_pos.y > 0 {
        let current_line = editor.file_content.remove(editor.cursor_pos.y);
        editor.cursor_pos.y -= 1;
        editor.file_content[editor.cursor_pos.y].push_str(&current_line);
        editor.cursor_pos.x = editor.file_content[editor.cursor_pos.y].len() - current_line.len();
        return true;
    }

    // Ordinary backspace function
    editor.file_content[editor.cursor_pos.y].remove(editor.cursor_pos.x - 1);
    editor.cursor_pos.x -= 1;
    true
}

fn handle_ctrl_backspace(editor: &mut Editor) {
    while editor.cursor_pos.x > 0
        && is_whitespace(
            &editor.file_content[editor.cursor_pos.y],
            editor.cursor_pos.x - 1,
        )
    {
        editor.file_content[editor.cursor_pos.y].remove(editor.cursor_pos.x - 1);
        editor.cursor_pos.x -= 1;
    }

    while editor.cursor_pos.x > 0
        && !is_whitespace(
            &editor.file_content[editor.cursor_pos.y],
            editor.cursor_pos.x - 1,
        )
    {
        editor.file_content[editor.cursor_pos.y].remove(editor.cursor_pos.x - 1);
        editor.cursor_pos.x -= 1;
    }
}

fn handle_right_arrow(editor: &mut Editor, is_ctrl_pressed: bool) {
    let line_len = editor.file_content[editor.cursor_pos.y].len() - 1;

    if editor.cursor_pos.x < line_len {
        if !is_ctrl_pressed {
            editor.cursor_pos.x += 1;
            return;
        }

        // If ctrl is pressed
        if is_whitespace(
            &editor.file_content[editor.cursor_pos.y],
            editor.cursor_pos.x,
        ) {
            editor.cursor_pos.x += 1;
        }
        // Move to the next whitespace or end of line
        while editor.cursor_pos.x < line_len
            && !is_whitespace(
                &editor.file_content[editor.cursor_pos.y],
                editor.cursor_pos.x,
            )
        {
            editor.cursor_pos.x += 1;
        }
    } else if editor.cursor_pos.x >= line_len && editor.cursor_pos.y < editor.file_content.len() - 1
    {
        // Move to the next line if at the end of the current line
        editor.cursor_pos.y += 1;
        editor.cursor_pos.x = 0;
    }
}

fn handle_left_arrow(editor: &mut Editor, is_ctrl_pressed: bool) {
    if editor.cursor_pos.x > 0 {
        if !is_ctrl_pressed {
            editor.cursor_pos.x -= 1;
            return;
        }

        // If ctrl is pressed
        if is_whitespace(
            &editor.file_content[editor.cursor_pos.y],
            editor.cursor_pos.x - 1,
        ) {
            editor.cursor_pos.x -= 1;
        }

        // Move to the previous whitespace or start of line
        while editor.cursor_pos.x > 0
            && !is_whitespace(
                &editor.file_content[editor.cursor_pos.y],
                editor.cursor_pos.x - 1,
            )
        {
            editor.cursor_pos.x -= 1;
        }
    } else if editor.cursor_pos.y > 0 {
        // Move to the end of the previous line
        editor.cursor_pos.y -= 1;
        editor.cursor_pos.x = editor.file_content[editor.cursor_pos.y].len();
    }
}
