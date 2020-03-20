// Copyright 2020, Eric Olson, All rights reserved. Contact eric.rob.olson@gmail.com for questions regarding use.
extern crate specs;
use specs::prelude::*;

use crate::cb_system;
use cb_system::{Coordinate, GameUnit};

use crate::cb_patterns;
use cb_patterns::command::MacroCommand;

use crate::cb_voxels;

use crate::cb_menu;
use cb_menu::Form;

pub struct VoxelComponent {
    pub chunk_manager: cb_voxels::CbChunkManager,
    pub editor: VoxelEditor,
}

pub struct VoxelEditor {
    pub editing: bool,
    pub created_menu: bool,
    pub z_index: usize,
}

impl VoxelEditor {
    pub fn new() -> Self {
        return Self {
            editing: true,
            created_menu: false,
            z_index: 0,
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

    pub fn start_editing(&mut self) -> Box<Form> {
        if self.editor.created_menu {
            panic!("already created menu!");
        }

        self.editor.created_menu = true;
        let palette = cb_menu::gfx::Palette::new();

        let mut columns = cb_menu::cb_form_column::CbFormColumn::new(palette);

        for x in 0..self.chunk_manager.get_voxel_width() {
            let mut row = cb_menu::cb_form_row::CbFormRow::new(palette);
            for y in 0..self.chunk_manager.get_voxel_width() {
                let mut button = cb_menu::cb_button_toggle::CbButtonToggle::new(palette);

                let (active, _, _, _) = self.chunk_manager.get_voxel(x, y, self.editor.z_index);

                button.value = *active;

                row.add_child(Box::new(button))
            }

            columns.add_child(Box::new(row));
        }

        return Box::new(columns);
    }

    pub fn stop_editing(&mut self) {
        self.editor.editing = false;
        self.editor.created_menu = false;
    }
}

init_component_implementations![VoxelComponent];

pub struct VoxelComponentController {
    command_stack: MacroCommand,
}
