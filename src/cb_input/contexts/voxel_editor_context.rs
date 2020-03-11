use super::*;

#[derive(Debug, Copy, Clone)]
pub struct VoxelEditorContext {
    pub undo: Press,
    pub redo: Press,
    pub cursor_x: Range,
    pub cursor_y: Range,
}

impl VoxelEditorContext {
    pub fn new() -> Self {
        return Self {
            undo: Press::NotPressed,
            redo: Press::NotPressed,
            cursor_x: Range::new(0),
            cursor_y: Range::new(0),
        };
    }
}

pub fn get_voxel_editor_context(
    game_tick: GameTick,
    events: &Vec<sdl2::event::Event>,
    previous_context: &VoxelEditorContext,
) -> VoxelEditorContext {
    let mut context = previous_context.clone();
    // TODO: make this a configurable thing

    let mut ctl_pressed = false;
    let mut z_pressed = false;
    let mut y_pressed = false;

    for event in events {
        match event {
            Event::KeyDown {
                keycode: Some(Keycode::LCtrl),
                ..
            } => {
                ctl_pressed = true;
            }
            Event::KeyDown {
                keycode: Some(Keycode::Z),
                ..
            } => {
                z_pressed = true;
            }
            Event::KeyDown {
                keycode: Some(Keycode::Y),
                ..
            } => {
                y_pressed = true;
            }
            _ => {}
        }
    }

    return VoxelEditorContext::new();
}
