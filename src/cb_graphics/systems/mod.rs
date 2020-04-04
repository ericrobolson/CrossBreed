extern crate specs;
use specs::prelude::*;

mod sprite_render_system;

pub fn gfx_build_dispatcher<'a, 'b>() -> specs::Dispatcher<'a, 'b> {
    return DispatcherBuilder::new()
        .with(
            sprite_render_system::SpriteRenderSystem,
            "sprite render system",
            &[],
        )
        .build();
}
