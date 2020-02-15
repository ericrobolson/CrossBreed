extern crate sdl2;
use sdl2::{event::Event, keyboard::Keycode, video::GLProfile};

use crate::cb_system;
use cb_system::{CbEvent, GameTick};

pub mod input_type;
use input_type::{Press, Range, State};

#[derive(Debug, Copy, Clone)]
pub struct MovementContext {
    pub jump: Press,
    pub crouching: State,
    pub running: State,
    pub prone: State,
    pub move_forward: State,
    pub move_backward: State,
    pub move_left: State,
    pub move_right: State,
}

impl MovementContext {
    pub fn new() -> Self {
        return Self {
            jump: Press::NotPressed,
            crouching: State::Off,
            running: State::Off,
            prone: State::Off,
            move_forward: State::Off,
            move_backward: State::Off,
            move_left: State::Off,
            move_right: State::Off,
        };
    }
}

pub fn get_movement_context(
    game_tick: GameTick,
    event_pump: &mut sdl2::EventPump,
    previous_context: &MovementContext,
) -> MovementContext {
    let mut context = previous_context.clone();
    // TODO: separate the os generation of inputs to a separate process, this will just be translating them to get the right context. also helps with multiple contexts valid at once
    // TODO: make this a configurable thing
    for event in event_pump.poll_iter() {
        match event {
            Event::Quit { .. }
            | Event::KeyDown {
                keycode: Some(Keycode::Escape),
                ..
            } => {
                //exit = true;
            }
            // Presses
            Event::KeyDown {
                keycode: Some(Keycode::Space),
                ..
            } => {
                context.jump = Press::Pressed;
            }
            // States - on
            Event::KeyDown {
                keycode: Some(Keycode::W),
                ..
            } => {
                context.move_forward = State::On;
            }
            Event::KeyDown {
                keycode: Some(Keycode::A),
                ..
            } => {
                context.move_left = State::On;
            }
            Event::KeyDown {
                keycode: Some(Keycode::S),
                ..
            } => {
                context.move_backward = State::On;
            }
            Event::KeyDown {
                keycode: Some(Keycode::D),
                ..
            } => {
                context.move_right = State::On;
            }
            Event::KeyDown {
                keycode: Some(Keycode::C),
                ..
            } => {
                context.crouching = State::On;
            }
            Event::KeyDown {
                keycode: Some(Keycode::LCtrl),
                ..
            } => {
                context.prone = State::On;
            }
            Event::KeyDown {
                keycode: Some(Keycode::LShift),
                ..
            } => {
                context.running = State::On;
            }
            // States - off
            Event::KeyUp {
                keycode: Some(Keycode::W),
                ..
            } => {
                context.move_forward = State::Off;
            }
            Event::KeyUp {
                keycode: Some(Keycode::A),
                ..
            } => {
                context.move_left = State::Off;
            }
            Event::KeyUp {
                keycode: Some(Keycode::S),
                ..
            } => {
                context.move_backward = State::Off;
            }
            Event::KeyUp {
                keycode: Some(Keycode::D),
                ..
            } => {
                context.move_right = State::Off;
            }
            Event::KeyUp {
                keycode: Some(Keycode::C),
                ..
            } => {
                context.crouching = State::Off;
            }
            Event::KeyUp {
                keycode: Some(Keycode::LCtrl),
                ..
            } => {
                context.prone = State::Off;
            }
            Event::KeyUp {
                keycode: Some(Keycode::LShift),
                ..
            } => {
                context.running = State::Off;
            }
            _ => {}
        }
    }

    return MovementContext::new();
}
