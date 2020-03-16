use specs::prelude::*;

pub struct EditorSystem;

impl<'a> System<'a> for EditorSystem {
    type SystemData = ();

    fn run(&mut self, (): Self::SystemData) {}
}
