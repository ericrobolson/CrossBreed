// Copyright 2020, Eric Olson, All rights reserved. Contact eric.rob.olson@gmail.com for questions regarding use.

extern crate sdl2;
use sdl2::{event::Event, keyboard::Keycode, video::GLProfile};

use crate::cb_system;
use cb_system::{CbEvent, GameTick};

pub mod input_type;
use input_type::{Press, Range, State};

pub mod contexts;
use contexts::shooter_context::ShooterMovementContext;

pub mod cb_input;
pub use cb_input::CbGameInput;

pub fn get_os_inputs(event_pump: &mut sdl2::EventPump) -> Vec<sdl2::event::Event> {
    let events = event_pump.poll_iter().map(|e| e).collect();

    return events;
}

pub struct CbInputContextManager {
    current_frame_inputs: Vec<sdl2::event::Event>,
    current_tick: usize,
    previous_shooter_movement_context: ShooterMovementContext,
}

impl CbInputContextManager {
    pub fn new() -> Self {
        return Self {
            current_frame_inputs: vec![],
            current_tick: 0,
            previous_shooter_movement_context: ShooterMovementContext::new(),
        };
    }

    pub fn reset(&mut self) {
        self.current_frame_inputs.clear();
    }

    pub fn get_os_inputs(&mut self, game_tick: usize, event_pump: &mut sdl2::EventPump) {
        self.current_tick = game_tick;
        self.current_frame_inputs = get_os_inputs(event_pump);
    }

    pub fn get_shooter_movement_context(&mut self) -> ShooterMovementContext {
        let movement_ctx = ShooterMovementContext::get_shooter_movement_context(
            self.current_tick,
            &self.current_frame_inputs,
            &self.previous_shooter_movement_context,
        );

        self.previous_shooter_movement_context = movement_ctx;

        return movement_ctx;
    }

    pub fn get_rmercury_inputs(&self) -> Vec<CbGameInput> {
        // unimplemented!()
        return vec![];
    }

    pub fn add_context(&mut self) {}
}
