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

fn get_os_inputs(event_pump: &mut sdl2::EventPump) -> Vec<sdl2::event::Event> {
    let events = event_pump.poll_iter().map(|e| e).collect();

    return events;
}

pub struct CbInputContextManager {
    current_frame_inputs: Vec<sdl2::event::Event>,
    current_tick: usize,
    previous_context: Option<CbInputContexts>,
}

impl CbInputContextManager {
    pub fn new() -> Self {
        return Self {
            current_frame_inputs: vec![],
            current_tick: 0,
            previous_context: None,
        };
    }

    pub fn reset(&mut self) {
        self.current_frame_inputs.clear();
    }

    pub fn read_os_inputs(&mut self, game_tick: usize, event_pump: &mut sdl2::EventPump) {
        self.current_tick = game_tick;
        self.current_frame_inputs = get_os_inputs(event_pump);
    }

    pub fn get_rmercury_inputs(&mut self) -> CbGameInput {
        let shooter_context;
        if self.previous_context.is_none() {
            shooter_context = None;
        } else {
            shooter_context = Some(self.previous_context.unwrap());
        }

        let shooter_context = contexts::shooter_context::get_shooter_context_from_keys(
            &self.current_frame_inputs,
            shooter_context,
        );

        // Note: each input has one context manager, but can have many contexts

        let mut ctx_mgr = contexts::CbContextManager::new();
        ctx_mgr.add_context(shooter_context);

        let game_input = CbGameInput::new(1, ctx_mgr);

        return game_input;
    }

    pub fn add_context(&mut self) {}
}
