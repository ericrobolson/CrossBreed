// Copyright 2020, Eric Olson, All rights reserved. Contact eric.rob.olson@gmail.com for questions regarding use.
extern crate specs;
use specs::prelude::*;

use crate::cb_system;
use cb_system::{Coordinate, GameUnit};

use crate::cb_patterns;
use cb_patterns::command::MacroCommand;

use crate::cb_voxels;

use crate::cb_math::cb_range::CbNormalizedRange;

use crate::cb_menu;
use cb_menu::{menu_events, menu_events::EventId, Form};

#[derive(Clone)]
pub struct VoxelComponent {
    pub chunk_manager: cb_voxels::CbChunkManager,
    pub editor: VoxelEditor,
}

#[derive(Clone)]
pub struct VoxelEditor {
    pub editing: bool,
    pub created_menu: bool,
    pub z_index: usize,
    z_index_callback: Option<EventId>,
    toggle_active_callbacks: Vec<ActivateCallbacks>,
}

impl VoxelEditor {
    pub fn new() -> Self {
        return Self {
            editing: true,
            z_index_callback: None,
            created_menu: false,
            z_index: 0,
            toggle_active_callbacks: vec![],
        };
    }

    pub fn reset(&mut self) {
        self.editing = false;
        self.created_menu = false;
        self.z_index_callback = None;
        self.z_index = 0;
        self.toggle_active_callbacks = vec![];
    }
}

#[derive(Clone)]
struct ActivateCallbacks {
    event_id: EventId,
    x_location: usize,
    y_location: usize,
}

impl ActivateCallbacks {
    pub fn new(event_id: EventId, x_location: usize, y_location: usize) -> Self {
        return Self {
            event_id: event_id,
            x_location: x_location,
            y_location: y_location,
        };
    }
}

impl VoxelComponent {
    pub fn new() -> Self {
        return Self {
            editor: VoxelEditor::new(),
            chunk_manager: cb_voxels::CbChunkManager::new(),
        };
    }

    pub fn handle_events(
        &mut self,
        events: &Vec<(menu_events::EventId, menu_events::Events)>,
        frame: usize,
    ) -> Vec<(menu_events::EventId, menu_events::Events)> {
        let mut databinding_changes = vec![];

        for (event_id, event) in events.iter() {
            for callback in self.editor.toggle_active_callbacks.iter_mut() {
                if callback.event_id == *event_id {
                    let mut voxel = self.chunk_manager.get_voxel_mut(
                        callback.x_location,
                        callback.y_location,
                        self.editor.z_index,
                        frame,
                    );

                    voxel.0 = !voxel.0;
                }
            }

            // Handle zindex callback
            if self.editor.z_index_callback.is_some()
                && self.editor.z_index_callback.unwrap() == *event_id
            {
                match event {
                    menu_events::Events::SingleRangeChange(range) => {
                        self.editor.z_index =
                            range.map_to_range_usize(0, self.chunk_manager.get_voxel_width());

                        // Update button databindings
                        for callback in self.editor.toggle_active_callbacks.iter() {
                            let (x, y, z) = (
                                callback.x_location,
                                callback.y_location,
                                self.editor.z_index,
                            );

                            let (voxel_active, _, _, _) = self.chunk_manager.get_voxel(x, y, z);

                            let event = (
                                callback.event_id,
                                cb_menu::menu_events::Events::BoolValueChange(*voxel_active),
                            );

                            databinding_changes.push(event);
                        }
                    }
                    _ => {}
                }
            }
        }

        return databinding_changes;
    }

    pub fn start_editing(&mut self) -> Box<Form> {
        if self.editor.created_menu {
            panic!("already created menu!");
        }

        self.editor.created_menu = true;

        // Initialize the menu
        let mut columns;
        {
            let palette = cb_menu::gfx::Palette::new();
            columns = cb_menu::cb_form_column::CbFormColumn::new(palette);

            let num_voxels_in_slice = cb_voxels::CHUNK_SIZE * cb_voxels::CHUNKS;

            for x in (0..num_voxels_in_slice).rev() {
                let mut row = cb_menu::cb_form_row::CbFormRow::new(palette);
                for y in (0..num_voxels_in_slice).rev() {
                    let (x, y) = (y, x);

                    let mut button = cb_menu::cb_button_toggle::CbButtonToggle::new(palette);

                    let (active, _, _, _) = self.chunk_manager.get_voxel(x, y, self.editor.z_index);

                    button.value = *active;

                    let callback = ActivateCallbacks::new(button.subscribe_to_event(), x, y);
                    self.editor.toggle_active_callbacks.push(callback);

                    row.add_child(Box::new(button))
                }

                columns.add_child(Box::new(row));
            }

            // Add slider component to control selected Z index
            let mut slider = cb_menu::cb_slider_horizontal::CbSliderHorizontal::new(palette);
            let slider_value = CbNormalizedRange::new(
                self.editor.z_index as i32,
                0,
                self.chunk_manager.get_voxel_width() as i32,
            );

            slider.x_value = slider_value;

            self.editor.z_index_callback = Some(slider.subscribe_to_event());

            columns.add_child(Box::new(slider));
        }

        return Box::new(columns);
    }

    pub fn stop_editing(&mut self) {
        self.editor.reset()
    }
}

init_component_implementations![VoxelComponent];

pub struct VoxelComponentController {
    command_stack: MacroCommand,
}
