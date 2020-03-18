use specs::prelude::*;

pub struct EditorSystem;

impl<'a> System<'a> for EditorSystem {
    type SystemData = ();

    fn run(&mut self, (): Self::SystemData) {
        println!("editor system");
        // If no entity selected, wait for selection
        // TODO: add ability to create entities on the fly?
    }
}
