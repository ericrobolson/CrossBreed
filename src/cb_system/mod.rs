// Copyright 2020, Eric Olson, All rights reserved. Contact eric.rob.olson@gmail.com for questions regarding use.

pub type GameTick = u32;
pub type PlayerId = u8;

pub type GameUnit = FInt;

use crate::cb_math;
use cb_math::FInt;

pub const FRAMEDELAY: GameTick = 3;

pub struct Coordinate3d {
    pub x: GameUnit,
    pub y: GameUnit,
    pub z: GameUnit,
}

impl Coordinate3d {
    pub fn new(x: GameUnit, y: GameUnit, z: GameUnit) -> Self {
        return Self { x: x, y: y, z: z };
    }

    pub fn zero() -> Self {
        let zero = FInt::from_num(0);
        return Self::new(zero, zero, zero);
    }

    pub fn one() -> Self {
        let one = FInt::from_num(1);
        return Self::new(one, one, one);
    }
}

pub struct Coordinate2d {
    pub x: GameUnit,
    pub y: GameUnit,
}

impl Coordinate2d {
    pub fn new(x: GameUnit, y: GameUnit) -> Self {
        return Self { x: x, y: y };
    }

    pub fn zero() -> Self {
        let zero = FInt::from_num(0);
        return Self::new(zero, zero);
    }

    pub fn one() -> Self {
        let one = FInt::from_num(1);
        return Self::new(one, one);
    }
}

pub struct CbEvent<T> {
    pub tick: GameTick,
    pub value: T,
}
