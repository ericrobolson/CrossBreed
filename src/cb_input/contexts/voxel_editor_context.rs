use super::*;

use crate::cb_graphics;
use cb_graphics::Sdl2HardwareInterface;

fn new_voxel_editor_context() -> CbInputContexts {
    return CbInputContexts::VoxelEditorContext {
        networked: Networked::On,
        look_x: Range::new(0),
        look_y: Range::new(0),
    };
}

pub fn get_voxel_editor_context_from_keys(
    hardware: &Sdl2HardwareInterface,
    previous_context: Option<CbContextManager>,
) -> CbInputContexts {
    let ctx;
    if previous_context.is_none() {
        ctx = new_voxel_editor_context();
    } else {
        // Attempt to find the previous context and use that, otherwise use a new one
        let prev_mgr = previous_context.unwrap();

        let c = prev_mgr.get_context(VOXEL_EDITOR_CONTEXT_ID);

        if c.is_some() {
            ctx = c.unwrap();
        } else {
            ctx = new_voxel_editor_context();
        }
    }

    let mut new_jump = Press::NotPressed;

    let_mut_for![
        (
            new_crouching,
            new_running,
            new_prone,
            new_move_forward,
            new_move_backward,
            new_move_left,
            new_move_right
        ),
        State,
        State::Off
    ];

    let mut look_x = Range::new(0);
    let mut look_y = Range::new(0);

    // Apply key events
    {
        match ctx {
            CbInputContexts::VoxelEditorContext {
                networked: _,
                look_x: ctx_look_x,
                look_y: ctx_look_y,
            } => {
                look_x = ctx_look_x;
                look_y = ctx_look_y;

                for event in hardware.events {
                    match event {
                        Event::Quit { .. }
                        | Event::KeyDown {
                            keycode: Some(Keycode::Escape),
                            ..
                        } => {
                            //exit = true;
                        }
                        // Presses
                        Event::KeyDown {
                            keycode: Some(Keycode::Space),
                            ..
                        } => {
                            new_jump = Press::Pressed;
                        }
                        // States - on
                        Event::KeyDown {
                            keycode: Some(Keycode::W),
                            ..
                        } => {
                            new_move_forward = State::On;
                        }
                        Event::KeyDown {
                            keycode: Some(Keycode::A),
                            ..
                        } => {
                            new_move_left = State::On;
                        }
                        Event::KeyDown {
                            keycode: Some(Keycode::S),
                            ..
                        } => {
                            new_move_backward = State::On;
                        }
                        Event::KeyDown {
                            keycode: Some(Keycode::D),
                            ..
                        } => {
                            new_move_right = State::On;
                        }
                        Event::KeyDown {
                            keycode: Some(Keycode::C),
                            ..
                        } => {
                            new_crouching = State::On;
                        }
                        Event::KeyDown {
                            keycode: Some(Keycode::LCtrl),
                            ..
                        } => {
                            new_prone = State::On;
                        }
                        Event::KeyDown {
                            keycode: Some(Keycode::LShift),
                            ..
                        } => {
                            new_running = State::On;
                        }
                        // States - off
                        Event::KeyUp {
                            keycode: Some(Keycode::W),
                            ..
                        } => {
                            new_move_forward = State::Off;
                        }
                        Event::KeyUp {
                            keycode: Some(Keycode::A),
                            ..
                        } => {
                            new_move_left = State::Off;
                        }
                        Event::KeyUp {
                            keycode: Some(Keycode::S),
                            ..
                        } => {
                            new_move_backward = State::Off;
                        }
                        Event::KeyUp {
                            keycode: Some(Keycode::D),
                            ..
                        } => {
                            new_move_right = State::Off;
                        }
                        Event::KeyUp {
                            keycode: Some(Keycode::C),
                            ..
                        } => {
                            new_crouching = State::Off;
                        }
                        Event::KeyUp {
                            keycode: Some(Keycode::LCtrl),
                            ..
                        } => {
                            new_prone = State::Off;
                        }
                        Event::KeyUp {
                            keycode: Some(Keycode::LShift),
                            ..
                        } => {
                            new_running = State::Off;
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }
    }

    // Now apply cursor movements
    {
        let cursor = sdl2::mouse::MouseState::new(hardware.pump);

        let default_cursor_x = hardware.window_width / 2;
        let default_cursor_y = hardware.window_height / 2;

        if hardware.reset_cursor {
            let xdiff = default_cursor_x - cursor.x();
            let ydiff = default_cursor_y - cursor.y();

            look_x.value -= xdiff;
            look_y.value -= ydiff;
        } else {
            look_x.value += cursor.x();
            look_y.value += cursor.y();
        }
    }

    return CbInputContexts::VoxelEditorContext {
        networked: Networked::On,
        look_x: look_x,
        look_y: look_y,
    };
}
