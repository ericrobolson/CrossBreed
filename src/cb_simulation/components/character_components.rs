extern crate specs;
use specs::prelude::*;

use crate::cb_system;
use cb_system::{Coordinate3d, GameUnit};

use crate::cb_math;
use cb_math::{FInt, FUint};

use super::ComponentLinker;

init_components![
    ComponentsLinker,
    (
        HitPointsComponent,
        ArmorComponent,
        UnitBaseComponent,
        MoveSpeedComponent,
        RangedAttackComponent
    )
];

pub struct RangedAttackComponent {
    rate_of_fire: FUint,
    range: FUint,
    damage: FUint,
}

impl RangedAttackComponent {
    pub fn new(rate_of_fire: FUint, range: FUint, damage: FUint) -> Self {
        return Self {
            rate_of_fire: rate_of_fire,
            range: range,
            damage: damage,
        };
    }
}

pub struct HitPointsComponent {
    max: FUint,
    value: FUint,
}

impl HitPointsComponent {
    pub fn new(value: FUint, max: FUint) -> Self {
        return Self {
            value: value,
            max: max,
        };
    }
}

pub struct ArmorComponent {
    max: FUint,
    value: FUint,
}

impl ArmorComponent {
    pub fn new(value: FUint, max: FUint) -> Self {
        return Self {
            value: value,
            max: max,
        };
    }
}

pub struct UnitBaseComponent {
    pub base_size: FUint,
}

impl UnitBaseComponent {
    pub fn new(base_size: FUint) -> Self {
        return Self {
            base_size: base_size,
        };
    }
}

pub struct MoveSpeedComponent {
    pub value: FUint,
}

impl MoveSpeedComponent {
    pub fn new(value: FUint) -> Self {
        return Self { value: value };
    }
}
