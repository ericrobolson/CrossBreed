// Copyright 2020, Eric Olson, All rights reserved. Contact eric.rob.olson@gmail.com for questions regarding use.

extern crate sdl2;
use sdl2::{event::Event, keyboard::Keycode};

use crate::cb_system;
use cb_system::GameTick;

use crate::cb_graphics;

use super::*;
use input_type::{Press, Range, State};

pub mod fighting_context;
pub mod rts_context;
pub mod shooter_context;
pub mod voxel_editor_context;

#[derive(Debug, Copy, Clone)]
pub enum Networked {
    On,
    Off,
}

#[derive(Debug, Copy, Clone)]
pub enum CbInputContexts {
    FightingContext {
        networked: Networked,
        up: State,
        down: State,
        left: State,
        right: State,
        punch_light: Press,
        punch_heavy: Press,
        kick_light: Press,
        kick_heavy: Press,
    },
    RtsContext {
        networked: Networked,

        select: Press,
        target: Press,
        cancel: Press,

        move_unit: Press,
        attack_move_unit: Press,
        activate_ability: Press,
        cursor_x: Range,
        cursor_y: Range,
    },
    ShooterContext {
        networked: Networked,
        jump: Press,
        crouching: State,
        running: State,
        prone: State,
        move_forward: State,
        move_backward: State,
        move_left: State,
        move_right: State,
        look_x: Range,
        look_y: Range,
    },
    VoxelEditorContext {
        networked: Networked,
        open_console: Press,
        cursor_x: Range,
        cursor_y: Range,
        toggle_orthographic_view: Press,
        front_view: Press,
        top_view: Press,
        right_view: Press,
        left_view: Press,
        rotate_camera_up: Press,
        rotate_camera_down: Press,
        rotate_camera_left: Press,
        rotate_camera_right: Press,
        add_voxel: Press,
        remove_voxel: Press,
    },
}

pub type ContextId = u8;

pub fn get_normalized_cursor_coordinates(hardware: &Sdl2HardwareInterface) -> (Range, Range) {
    let cursor = sdl2::mouse::MouseState::new(hardware.pump);
    let cursor_x = Range::new(cursor.x(), 0, hardware.window_width);
    let cursor_y = Range::new(cursor.y(), 0, hardware.window_height);

    return (cursor_x, cursor_y);
}

//NOTE: ALWAYS ADD TO THE END TO PRESERVE BACKWARDS COMPATIBILITY!!!!
pub const EMPTY_CONTENTS: ContextId = 0;
pub const FIGHTING_CONTEXT_ID: ContextId = EMPTY_CONTENTS + 1;
pub const RTS_CONTEXT_ID: ContextId = FIGHTING_CONTEXT_ID + 1;
pub const SHOOTER_CONTEXT_ID: ContextId = RTS_CONTEXT_ID + 1;
pub const VOXEL_EDITOR_CONTEXT_ID: ContextId = SHOOTER_CONTEXT_ID + 1;

//END NOTE

pub fn get_context_id_from_context(context: CbInputContexts) -> ContextId {
    match context {
        CbInputContexts::FightingContext {
            networked: _,
            up: _,
            down: _,
            left: _,
            right: _,
            punch_light: _,
            punch_heavy: _,
            kick_light: _,
            kick_heavy: _,
        } => FIGHTING_CONTEXT_ID,
        CbInputContexts::RtsContext {
            networked: _,
            select: _,
            target: _,
            cancel: _,

            move_unit: _,
            attack_move_unit: _,
            activate_ability: _,
            cursor_x: _,
            cursor_y: _,
        } => RTS_CONTEXT_ID,
        CbInputContexts::ShooterContext {
            networked: _,
            jump: _,
            crouching: _,
            running: _,
            prone: _,
            move_forward: _,
            move_backward: _,
            move_left: _,
            move_right: _,
            look_x: _,
            look_y: _,
        } => SHOOTER_CONTEXT_ID,
        CbInputContexts::VoxelEditorContext {
            networked: _,
            open_console: _,
            cursor_x: _,
            cursor_y: _,
            toggle_orthographic_view: _,
            front_view: _,
            top_view: _,
            right_view: _,
            left_view: _,
            rotate_camera_up: _,
            rotate_camera_down: _,
            rotate_camera_left: _,
            rotate_camera_right: _,
            add_voxel: _,
            remove_voxel: _,
        } => VOXEL_EDITOR_CONTEXT_ID,
    }
}

const NUM_ACTIVE_CONTEXTS: usize = 10;

#[derive(Debug, Copy, Clone)]
pub struct CbContextManager {
    contexts: [Option<CbInputContexts>; NUM_ACTIVE_CONTEXTS],
}

impl CbContextManager {
    pub fn new() -> Self {
        return Self {
            contexts: [None; NUM_ACTIVE_CONTEXTS],
        };
    }

    pub fn get_contexts(&self) -> &[Option<CbInputContexts>; NUM_ACTIVE_CONTEXTS] {
        return &self.contexts;
    }

    pub fn get_context(&self, context_id: ContextId) -> Option<CbInputContexts> {
        for ctx in self.contexts.iter() {
            if ctx.is_none() {
                continue;
            }

            let ctx = ctx.unwrap();

            let id = get_context_id_from_context(ctx);

            if id == context_id {
                return Some(ctx);
            }
        }
        return None;
    }

    pub fn add_context(&mut self, context: CbInputContexts) {
        self.contexts[0] = Some(context);
    }

    pub fn to_bits(&self) -> Vec<u8> {
        unimplemented!();
    }

    pub fn from_bits(bits: Vec<u8>) -> Self {
        unimplemented!();
    }
}
