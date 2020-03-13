// Copyright 2020, Eric Olson, All rights reserved. Contact eric.rob.olson@gmail.com for questions regarding use.

extern crate sdl2;
use sdl2::{event::Event, keyboard::Keycode, video::GLProfile};

use crate::cb_system;
use cb_system::{CbEvent, GameTick};

pub mod input_type;
use input_type::{Press, Range, State};

pub mod contexts;
use contexts::CbInputContexts;

pub mod cb_input;
pub use cb_input::CbGameInput;

use crate::cb_graphics;
use cb_graphics::Sdl2HardwareInterface;

fn get_os_inputs(event_pump: &mut sdl2::EventPump) -> Vec<sdl2::event::Event> {
    let events = event_pump.poll_iter().map(|e| e).collect();

    return events;
}

pub struct CbInputContextManager {
    current_frame_inputs: Vec<sdl2::event::Event>,
    previous_context: Option<CbInputContexts>,
}

impl CbInputContextManager {
    pub fn new() -> Self {
        return Self {
            current_frame_inputs: vec![],
            previous_context: None,
        };
    }

    pub fn read_os_inputs(&mut self, event_pump: &mut sdl2::EventPump) -> Vec<sdl2::event::Event> {
        let current_frame_inputs = get_os_inputs(event_pump);

        return current_frame_inputs;
    }

    pub fn get_rmercury_inputs(&mut self, input_interface: &Sdl2HardwareInterface) -> CbGameInput {
        let shooter_context;
        if self.previous_context.is_none() {
            shooter_context = None;
        } else {
            shooter_context = Some(self.previous_context.unwrap());
        }

        // TODO: need to fix this; not quite sure if this should be managed in this class?
        let shooter_context = contexts::shooter_context::get_shooter_context_from_keys(
            &input_interface,
            shooter_context,
        );

        // Note: each input has one context manager, but can have many contexts

        let mut ctx_mgr = contexts::CbContextManager::new();
        ctx_mgr.add_context(shooter_context);

        self.previous_context = Some(shooter_context);

        let game_input = CbGameInput::new(1, ctx_mgr);

        return game_input;
    }
}
