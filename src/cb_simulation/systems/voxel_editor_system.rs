use specs::prelude::*;

use crate::cb_cmd_line;
use crate::cb_input;
use crate::cb_simulation;
use crate::cb_voxels;
use cb_cmd_line::CbCmdMenu;
use cb_simulation::components::{gfx_components, physics_components, voxel_components};
use cb_simulation::CbSystemValues;
pub struct VoxelEditorSystem;

impl<'a> System<'a> for VoxelEditorSystem {
    type SystemData = (
        Read<'a, CbSystemValues>,
        WriteStorage<'a, gfx_components::CameraComponent>,
        WriteStorage<'a, voxel_components::VoxelComponent>,
        ReadStorage<'a, physics_components::TransformComponent>,
        ReadStorage<'a, physics_components::VelocityComponent>,
    );

    fn run(
        &mut self,
        (system_values, mut camera_components, mut voxel_components, _, _): Self::SystemData,
    ) {
        for input in system_values.world_inputs.iter() {
            for ctx in input.context_manager.get_contexts().iter() {
                if ctx.is_none() {
                    continue;
                }
                let ctx = ctx.unwrap();

                match ctx {
                    cb_input::contexts::CbInputContexts::VoxelEditorContext {
                        networked: _,
                        open_console: open_console,
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
                    } => {
                        const MAX_EDITOR_X: i32 = 2500;
                        const MAX_EDITOR_Y: i32 = 2500;
                        const MAX_EDITOR_Z: i32 = 2500;

                        for camera in (&mut camera_components).join() {
                            // Camera Rotations
                            {
                                const MAX_ROTATION: i32 = 100;

                                if rotate_camera_up == cb_input::input_type::Press::Pressed {
                                    camera.camera_target_y += MAX_ROTATION;
                                } else if rotate_camera_down == cb_input::input_type::Press::Pressed
                                {
                                    camera.camera_target_y -= MAX_ROTATION;
                                }
                            }

                            // Camera Positions
                            {
                                if front_view == cb_input::input_type::Press::Pressed {
                                    camera.camera_pos_x = MAX_EDITOR_X / 2;
                                    camera.camera_pos_y = MAX_EDITOR_Y / 2;
                                    camera.camera_pos_z = -MAX_EDITOR_Z;

                                    camera.camera_target_x = MAX_EDITOR_X / 2;
                                    camera.camera_target_y = MAX_EDITOR_Y / 2;
                                    camera.camera_target_z = MAX_EDITOR_Z / 2;
                                } else if left_view == cb_input::input_type::Press::Pressed {
                                    camera.camera_pos_x = MAX_EDITOR_X;
                                    camera.camera_pos_y = MAX_EDITOR_Y / 2;
                                    camera.camera_pos_z = MAX_EDITOR_Z / 2;

                                    camera.camera_target_x = MAX_EDITOR_X / 2;
                                    camera.camera_target_y = MAX_EDITOR_Y / 2;
                                    camera.camera_target_z = MAX_EDITOR_Z / 2;
                                } else if right_view == cb_input::input_type::Press::Pressed {
                                    camera.camera_pos_x = -MAX_EDITOR_X;
                                    camera.camera_pos_y = MAX_EDITOR_Y / 2;
                                    camera.camera_pos_z = MAX_EDITOR_Z / 2;

                                    camera.camera_target_x = MAX_EDITOR_X / 2;
                                    camera.camera_target_y = MAX_EDITOR_Y / 2;
                                    camera.camera_target_z = MAX_EDITOR_Z / 2;
                                } else if top_view == cb_input::input_type::Press::Pressed {
                                    camera.camera_pos_x = MAX_EDITOR_X / 2;
                                    camera.camera_pos_y = MAX_EDITOR_Y / 2;
                                    camera.camera_pos_z = MAX_EDITOR_Z;

                                    camera.camera_target_x = MAX_EDITOR_X / 2;
                                    camera.camera_target_y = MAX_EDITOR_Y / 2;
                                    camera.camera_target_z = MAX_EDITOR_Z / 2;
                                } else if toggle_orthographic_view
                                    == cb_input::input_type::Press::Pressed
                                {
                                    camera.camera_orthographic_view =
                                        !camera.camera_orthographic_view;
                                }
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
    }
}
