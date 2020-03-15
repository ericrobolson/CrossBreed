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

                        if open_console == cb_input::input_type::Press::Pressed {
                            let top_level_menu =
                                CbCmdMenu::root("VOXEL_EDITOR", vec!["Add Voxel", "Remove Voxel"]);

                            const ADD_VOXEL: &str = "1";
                            const REMOVE_VOXEL: &str = "2";

                            top_level_menu.print();

                            let mut mode_choice = "-1".to_string();
                            let mut done = false;
                            while !done {
                                mode_choice = top_level_menu.get_menu_choice();

                                if mode_choice == ADD_VOXEL {
                                    let max_value = cb_voxels::CHUNKS * cb_voxels::CHUNK_SIZE;

                                    println!(
                                        "Add Voxel in range x: 0..{}, y: 0..{}, z: 0..{}",
                                        max_value, max_value, max_value
                                    );

                                    let mut inner_done = false;
                                    while !inner_done {
                                        let value = top_level_menu.get_menu_choice();

                                        let splits: Vec<&str> = value.split(",").collect();

                                        if splits.len() != 3 {
                                            println!("Must enter in the format 'x,y,z'.");
                                            continue;
                                        }

                                        let mut values = vec![];

                                        for split in splits.iter() {
                                            let v = split.parse::<usize>();

                                            if v.is_err() {
                                                println!("invalid format!");
                                                continue;
                                            }

                                            let v = v.unwrap();
                                            values.push(v);
                                        }

                                        let x = values[0];
                                        let y = values[1];
                                        let z = values[2];

                                        for voxel_component in (&mut voxel_components).join() {
                                            let (active, _, _, _) = voxel_component
                                                .chunk_manager
                                                .get_voxel_mut(x, y, z, system_values.frame);

                                            *active = true;
                                        }

                                        done = true;
                                        inner_done = true;
                                    }
                                } else if mode_choice == REMOVE_VOXEL {
                                    println!("remove voxel");
                                    done = true;
                                } else {
                                    println!("Invalid choice! Try again.");
                                }
                            }
                        }

                        //add/remove voxels
                        {
                            if add_voxel == cb_input::input_type::Press::Pressed {
                                for voxel_component in (&mut voxel_components).join() {
                                    let x = cursor_x.map_to_range_usize(
                                        0,
                                        voxel_component.chunk_manager.get_voxel_width(),
                                    );

                                    let y = cursor_y.map_to_range_usize(
                                        0,
                                        voxel_component.chunk_manager.get_voxel_width(),
                                    );
                                    let z = 0;

                                    let (active, _, _, _) = voxel_component
                                        .chunk_manager
                                        .get_voxel_mut(x, y, z, system_values.frame);

                                    *active = true;
                                }
                            } else if remove_voxel == cb_input::input_type::Press::Pressed {
                                for voxel_component in (&mut voxel_components).join() {
                                    let x = cursor_x.map_to_range_usize(
                                        0,
                                        voxel_component.chunk_manager.get_voxel_width(),
                                    );

                                    let y = cursor_y.map_to_range_usize(
                                        0,
                                        voxel_component.chunk_manager.get_voxel_width(),
                                    );

                                    let z = 0;

                                    let (active, _, _, _) = voxel_component
                                        .chunk_manager
                                        .get_voxel_mut(x, y, z, system_values.frame);

                                    *active = false;
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
