// Copyright 2020, Eric Olson, All rights reserved. Contact eric.rob.olson@gmail.com for questions regarding use.

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Range {
    pub value: i32,
}

impl Range {
    pub fn new(value: i32) -> Self {
        return Range { value: value };
    }
}

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
