// Copyright 2020, Eric Olson, All rights reserved. Contact eric.rob.olson@gmail.com for questions regarding use.

extern crate sdl2;
use sdl2::{event::Event, keyboard::Keycode, video::GLProfile};

use crate::cb_system;
use cb_system::{CbEvent, GameTick};

pub mod input_type;
use input_type::{Press, Range, State};

pub mod contexts;
use contexts::{
    CbContextManager, CbInputContexts, ContextId, SHOOTER_CONTEXT_ID, VOXEL_EDITOR_CONTEXT_ID,
};

pub mod cb_input;
pub use cb_input::CbGameInput;

use crate::cb_graphics;
use cb_graphics::Sdl2HardwareInterface;

fn get_os_inputs(event_pump: &mut sdl2::EventPump) -> Vec<sdl2::event::Event> {
    let events = event_pump.poll_iter().map(|e| e).collect();

    return events;
}

pub struct CbInputContextManager {
    active_contexts: Vec<ContextId>,
    previous_context: Option<CbContextManager>,
}

impl CbInputContextManager {
    pub fn new() -> Self {
        return Self {
            previous_context: None,
            active_contexts: vec![],
        };
    }

    pub fn add_context(&mut self, context_id: ContextId) {
        let already_active_context = self.active_contexts.iter().find(|x| **x == context_id);

        if already_active_context.is_none() {
            self.active_contexts.push(context_id);
        }
    }

    pub fn remove_context(&mut self, context_id: ContextId) {
        self.active_contexts.retain(|i| *i != context_id);
    }

    pub fn read_os_inputs(&mut self, event_pump: &mut sdl2::EventPump) -> Vec<sdl2::event::Event> {
        let current_frame_inputs = get_os_inputs(event_pump);

        return current_frame_inputs;
    }

    pub fn get_rmercury_inputs(&mut self, input_interface: &Sdl2HardwareInterface) -> CbGameInput {
        let mut ctx_mgr = contexts::CbContextManager::new();

        for active_context in self.active_contexts.iter() {
            // SHOOTER CONTEXT
            if *active_context == SHOOTER_CONTEXT_ID {
                let shooter_context = contexts::shooter_context::get_shooter_context_from_keys(
                    &input_interface,
                    self.previous_context,
                );

                ctx_mgr.add_context(shooter_context);
            }
            // VOXEL EDITOR CONTEXT
            else if *active_context == VOXEL_EDITOR_CONTEXT_ID {
                let voxel_editor_context =
                    contexts::voxel_editor_context::get_voxel_editor_context_from_keys(
                        &input_interface,
                        self.previous_context,
                    );

                ctx_mgr.add_context(voxel_editor_context);
            }
        }

        self.previous_context = Some(ctx_mgr);

        let game_input = CbGameInput::new(1, ctx_mgr);

        return game_input;
    }
}
