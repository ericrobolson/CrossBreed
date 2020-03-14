use super::*;

use crate::cb_graphics;
use cb_graphics::Sdl2HardwareInterface;

fn new_voxel_editor_context() -> CbInputContexts {
    return CbInputContexts::VoxelEditorContext {
        networked: Networked::Off,
        cursor_x: Range::new(0),
        cursor_y: Range::new(0),
        toggle_orthographic_view: Press::NotPressed,
        front_view: Press::NotPressed,
        top_view: Press::NotPressed,
        right_view: Press::NotPressed,
        left_view: Press::NotPressed,
        rotate_camera_up: Press::NotPressed,
        rotate_camera_down: Press::NotPressed,
        rotate_camera_left: Press::NotPressed,
        rotate_camera_right: Press::NotPressed,
        add_voxel: Press::NotPressed,
        remove_voxel: Press::NotPressed,
    };
}

fn get_press_from_keys(press: &mut Press, keycode: sdl2::keyboard::Keycode, keys: &Vec<Keycode>) {
    if keys.iter().any(|k| *k == keycode) {
        *press = Press::Pressed;
    }
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

    // Declare keys to map to
    let toggle_orthographic_view_keys = vec![Keycode::Num0];
    let front_view_keys = vec![Keycode::Num1];
    let left_view_keys = vec![Keycode::Num2];
    let right_view_keys = vec![Keycode::Num3];
    let top_view_keys = vec![Keycode::Num4];

    let rotate_camera_up_keys = vec![Keycode::Up];
    let rotate_camera_down_keys = vec![Keycode::Down];
    let rotate_camera_left_keys = vec![Keycode::Left];
    let rotate_camera_right_keys = vec![Keycode::Right];

    let add_voxel_keys = vec![Keycode::E];
    let remove_voxel_keys = vec![Keycode::Q];

    let_mut_for![
        (
            toggle_orthographic_view,
            front_view,
            top_view,
            right_view,
            left_view,
            rotate_camera_up,
            rotate_camera_down,
            rotate_camera_left,
            rotate_camera_right,
            add_voxel,
            remove_voxel
        ),
        Press,
        Press::NotPressed
    ];

    let mut cursor_x = Range::new(0);
    let mut cursor_y = Range::new(0);

    // Apply key events
    {
        match ctx {
            CbInputContexts::VoxelEditorContext {
                networked: _,
                cursor_x: ctx_cursor_x,
                cursor_y: ctx_cursor_y,
                toggle_orthographic_view: ctx_toggle_orthographic_view,
                front_view: ctx_front_view,
                top_view: ctx_top_view,
                right_view: ctx_right_view,
                left_view: ctx_left_view,
                rotate_camera_up: ctx_rotate_camera_up,
                rotate_camera_down: ctx_rotate_camera_down,
                rotate_camera_left: ctx_rotate_camera_left,
                rotate_camera_right: ctx_rotate_camera_right,
                add_voxel: ctx_add_voxel,
                remove_voxel: ctx_remove_voxel,
            } => {
                cursor_x = ctx_cursor_x;
                cursor_y = ctx_cursor_y;

                for event in hardware.events {
                    match event {
                        Event::KeyDown { keycode: a, .. } => {
                            if a.is_some() {
                                let keycode = a.unwrap();

                                get_press_from_keys(
                                    &mut toggle_orthographic_view,
                                    keycode,
                                    &toggle_orthographic_view_keys,
                                );
                                get_press_from_keys(&mut front_view, keycode, &front_view_keys);
                                get_press_from_keys(&mut top_view, keycode, &top_view_keys);
                                get_press_from_keys(&mut right_view, keycode, &right_view_keys);
                                get_press_from_keys(&mut left_view, keycode, &left_view_keys);
                                get_press_from_keys(
                                    &mut rotate_camera_up,
                                    keycode,
                                    &rotate_camera_up_keys,
                                );
                                get_press_from_keys(
                                    &mut rotate_camera_down,
                                    keycode,
                                    &rotate_camera_down_keys,
                                );
                                get_press_from_keys(
                                    &mut rotate_camera_left,
                                    keycode,
                                    &rotate_camera_left_keys,
                                );
                                get_press_from_keys(
                                    &mut rotate_camera_right,
                                    keycode,
                                    &rotate_camera_right_keys,
                                );

                                get_press_from_keys(&mut add_voxel, keycode, &add_voxel_keys);
                                get_press_from_keys(&mut remove_voxel, keycode, &remove_voxel_keys);
                            }
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

            cursor_x.value -= xdiff;
            cursor_y.value -= ydiff;
        } else {
            cursor_x.value += cursor.x();
            cursor_y.value += cursor.y();
        }
    }

    return CbInputContexts::VoxelEditorContext {
        networked: Networked::Off,
        cursor_x: cursor_x,
        cursor_y: cursor_y,
        toggle_orthographic_view: toggle_orthographic_view,
        front_view: front_view,
        top_view: top_view,
        right_view: right_view,
        left_view: left_view,
        rotate_camera_up: rotate_camera_up,
        rotate_camera_down: rotate_camera_down,
        rotate_camera_left: rotate_camera_left,
        rotate_camera_right: rotate_camera_right,
        add_voxel: add_voxel,
        remove_voxel: remove_voxel,
    };
}
