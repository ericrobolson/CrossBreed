extern crate sdl2;
use sdl2::{event::Event, keyboard::Keycode, video::GLProfile};

use crate::cb_system;
use cb_system::{CbEvent, GameTick};

pub mod input_type;
use input_type::{Press, Range, State};

pub mod contexts;

pub fn get_os_inputs(event_pump: &mut sdl2::EventPump) -> Vec<sdl2::event::Event> {
    let events = event_pump.poll_iter().map(|e| e).collect();

    return events;
}
