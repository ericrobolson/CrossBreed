// Copyright 2020, Eric Olson, All rights reserved. Contact eric.rob.olson@gmail.com for questions regarding use.

extern crate specs;
use specs::prelude::*;

use crate::cb_simulation::components::rts_components;
use rts_components::{
    ActiveAbilityComponent, ArmorComponent, AttackComponent, HealthComponent,
    PassiveAbilityComponent, RtsMovableComponent, UnitComponent,
};

use crate::cb_simulation::components::physics_components;
use physics_components::{TransformComponent, VelocityComponent};

pub fn new_unit(world: &mut specs::World) {
    // RTS components
    let armor = ArmorComponent::new();
    let health = HealthComponent::new();
    let attack = AttackComponent::new();
    let moveable = RtsMovableComponent::new();
    let unit = UnitComponent::new();
    let active_ability = ActiveAbilityComponent::new();
    let passive_ability = PassiveAbilityComponent::new();

    // Physics components
    let transform = TransformComponent::new();
    let velocity = VelocityComponent::new();

    world
        .create_entity()
        .with(armor)
        .with(moveable)
        .with(health)
        .with(attack)
        .with(unit)
        .with(active_ability)
        .with(passive_ability)
        .with(transform)
        .with(velocity)
        .build();
}
