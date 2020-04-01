extern crate specs;
use specs::prelude::*;

use crate::cb_system;
use cb_system::{Coordinate, GameUnit};

use super::ComponentLinker;

init_components![
    RtsComponentsLinker,
    (
        ArmorComponent,
        HealthComponent,
        AttackComponent,
        RtsMovableComponent,
        UnitComponent,
        AbilitiesComponent
    )
];

pub struct ArmorComponent {
    pub max: u8,
    pub min: u8,
    pub current: u8,
}

pub struct HealthComponent {
    pub max: u8,
    pub min: u8,
    pub current: u8,
}

impl ArmorComponent {
    pub fn new() -> Self {
        return Self {
            max: 100,
            min: 0,
            current: 100,
        };
    }
}

impl HealthComponent {
    pub fn new() -> Self {
        return Self {
            max: 100,
            min: 0,
            current: 100,
        };
    }
}

pub struct AttackComponent {
    pub range: GameUnit,
    pub damage: GameUnit,
    pub attack_speed: GameUnit,
}

impl AttackComponent {
    pub fn new() -> Self {
        return Self {
            range: 100,
            damage: 10,
            attack_speed: 100,
        };
    }
}
// Copyright 2020, Eric Olson, All rights reserved. Contact eric.rob.olson@gmail.com for questions regarding use.

pub struct RtsMovableComponent {
    pub target: Option<Coordinate>,
    pub move_speed: GameUnit,
}

impl RtsMovableComponent {
    pub fn new() -> Self {
        return Self {
            target: None,
            move_speed: 10,
        };
    }
}

pub struct UnitComponent {
    pub point_cost: u8,
    pub base_size: GameUnit,
}

impl UnitComponent {
    pub fn new() -> Self {
        return Self {
            point_cost: 1,
            base_size: 10,
        };
    }
}

pub struct AbilitiesComponent {}

impl AbilitiesComponent {
    pub fn new() -> Self {
        return Self {};
    }
}
