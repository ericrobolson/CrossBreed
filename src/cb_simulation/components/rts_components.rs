extern crate specs;
use specs::prelude::*;

use crate::cb_system;
use cb_system::{Coordinate, GameUnit};

pub struct ArmorComponent {
    pub max: u8,
    pub min: u8,
    pub current: u8,
}

impl ArmorComponent {
    pub fn new() -> Self {
        return Self {
            max: 0,
            min: 0,
            current: 0,
        };
    }
}

pub struct HealthComponent {
    pub max: u8,
    pub min: u8,
    pub current: u8,
}

pub struct AttackComponent {
    pub range: GameUnit,
    pub damage: GameUnit,
    pub attack_speed: GameUnit,
}

pub struct RtsMovableComponent {
    pub target: Option<Coordinate>,
    pub move_speed: GameUnit,
}

pub struct UnitComponent {
    pub point_cost: u8,
    pub base_size: GameUnit,
}

pub struct PassiveAbilityComponent {}

pub struct ActiveAbilityComponent {}

init_component_implementations![
    ArmorComponent,
    HealthComponent,
    AttackComponent,
    RtsMovableComponent,
    UnitComponent,
    PassiveAbilityComponent,
    ActiveAbilityComponent
];
