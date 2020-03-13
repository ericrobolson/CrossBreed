// Copyright 2020, Eric Olson, All rights reserved. Contact eric.rob.olson@gmail.com for questions regarding use.

extern crate sdl2;
use sdl2::{event::Event, keyboard::Keycode};

use crate::cb_system;
use cb_system::GameTick;

use super::*;
use input_type::{Press, Range, State};

pub mod fighting_context;
pub mod rts_context;
pub mod shooter_context;
pub mod voxel_editor_context;

/*

note: was considering using OOP for the contexts, but instead decided to experiment with an enum based approach as I can leverage pattern matching. 2020/03/12

trait CbContextDeserializable {
    fn from_context_id(context_id: usize) -> Self;
}

trait CbContextSerializable {
    fn to_context_id(&self) -> usize;
}
*/
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
        select: Press,
        target: Press,
        cancel: Press,

        move_unit: Press,
        attack_move_unit: Press,
        activate_ability: Press,
        cursor_x: Range,
        cursor_y: Range,
    },
}

type ContextId = u8;

//NOTE: ALWAYS ADD TO THE END TO PRESERVE BACKWARDS COMPATIBILITY!!!!
const_identities![
    (EMPTY_CONTENTS, FIGHTING_CONTEXT_ID, RTS_CONTEXT_ID),
    ContextId
];
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
            select: _,
            target: _,
            cancel: _,

            move_unit: _,
            attack_move_unit: _,
            activate_ability: _,
            cursor_x: _,
            cursor_y: _,
        } => RTS_CONTEXT_ID,
    }
}

const NUM_ACTIVE_CONTEXTS: usize = 10;

#[derive(Debug, Copy, Clone)]
pub struct CbContextManager {
    contexts: [Option<CbInputContexts>; NUM_ACTIVE_CONTEXTS],
}

impl CbContextManager {
    fn get_contexts(&self) -> &[Option<CbInputContexts>; NUM_ACTIVE_CONTEXTS] {
        return &self.contexts;
    }

    pub fn to_bits(&self) -> Vec<u8> {
        unimplemented!();
    }

    pub fn from_bits(bits: Vec<u8>) -> Self {
        unimplemented!();
    }
}
