// Copyright 2020, Eric Olson, All rights reserved. Contact eric.rob.olson@gmail.com for questions regarding use.

use crate::cb_math;
use cb_math::cb_range::CbNormalizedRange;

pub type Range = CbNormalizedRange;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Press {
    Pressed,
    NotPressed,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum State {
    On,
    Off,
}
