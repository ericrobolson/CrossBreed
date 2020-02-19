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
            max: 100,
            min: 0,
            current: 100,
        };
    }
}

pub struct HealthComponent {
    pub max: u8,
    pub min: u8,
    pub current: u8,
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

pub struct PassiveAbilityComponent {}

impl PassiveAbilityComponent {
    pub fn new() -> Self {
        return Self {};
    }
}

pub struct ActiveAbilityComponent {}

impl ActiveAbilityComponent {
    pub fn new() -> Self {
        return Self {};
    }
}

init_component_implementations![
    ArmorComponent,
    HealthComponent,
    AttackComponent,
    RtsMovableComponent,
    UnitComponent,
    PassiveAbilityComponent,
    ActiveAbilityComponent
];
