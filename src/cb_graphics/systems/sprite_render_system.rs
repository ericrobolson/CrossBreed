use crate::cb_simulation;
use cb_simulation::components;
use cb_simulation::CbSystemValues;

use crate::cb_inverse_kinematics;
use cb_inverse_kinematics::{fabrik, CbMatrix, IkRig};

use specs::prelude::*;
pub struct SpriteRenderSystem;

use crate::cb_menu;

use crate::cb_input;

impl<'a> System<'a> for SpriteRenderSystem {
    type SystemData = (
        Read<'a, CbSystemValues>,
        Entities<'a>,
        ReadStorage<'a, components::gfx_components::SpriteComponent>,
        WriteStorage<'a, components::gfx_components::SpriteRenderComponent>,
        ReadStorage<'a, components::physics_components::TransformComponent>,
    );

    fn run(
        &mut self,
        (
            sys_values,
            entities,
            sprite_components,
            mut sprite_render_components,
            transform_components,
        ): Self::SystemData,
    ) {
        // Build up sprite render components
        {
            let render_components_to_add: Vec<(
                specs::Entity,
                cb_simulation::components::gfx_components::SpriteRenderComponent,
            )> = (
                &entities,
                &sprite_components,
                &transform_components,
                !&sprite_render_components,
            )
                .join()
                .map(|(entity, sprite_component, transform_component, ())| {
                    return (
                        entity,
                        cb_simulation::components::gfx_components::SpriteRenderComponent::new(),
                    );
                })
                .collect();

            render_components_to_add.iter().for_each(|(entity, value)| {
                &mut sprite_render_components.insert(*entity, value.clone());
            });
        }

        // Update sprite render components
        for (entity, sprite, sprite_render, transform) in (
            &entities,
            &sprite_components,
            &mut sprite_render_components,
            &transform_components,
        )
            .join()
        {}

        // Render sprites
        for (entity, sprite, sprite_render, transform) in (
            &entities,
            &sprite_components,
            &mut sprite_render_components,
            &transform_components,
        )
            .join()
        {}

        println!("ran sprite renderer!");
    }
}
