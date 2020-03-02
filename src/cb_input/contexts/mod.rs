extern crate sdl2;
use sdl2::{event::Event, keyboard::Keycode};

use crate::cb_system;
use cb_system::GameTick;

use super::*;
use input_type::{Press, Range, State};

pub mod fighting_context;
pub mod rts_context;
pub mod shooter_context;
