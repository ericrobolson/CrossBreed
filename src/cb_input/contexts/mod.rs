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
