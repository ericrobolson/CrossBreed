extern crate specs;
use specs::prelude::*;

use super::ComponentLinker;

init_components![EditorComponentsLinker, (EditableComponent)];

pub struct EditableComponent {
    editing: bool,
}

impl EditableComponent {
    pub fn new(editing: bool) -> Self {
        return Self { editing: editing };
    }

    pub fn is_editing(&self) -> bool {
        return self.editing;
    }
}
