extern crate specs;
use specs::prelude::*;

use super::ComponentLinker;

init_components![MenuComponentsLinker, (RadialMenuComponent)];

pub trait Elm {
    fn view(&self);
    fn model(&self);
    fn update(&mut self);
    fn update_world(&self, world: &mut World);
}

pub struct RadialMenuComponent {
    choices: Vec<RadialMenuChoice>,
}

impl RadialMenuComponent {
    pub fn open(&mut self) {}

    pub fn close(&mut self) {}

    pub fn new() -> Self {
        return Self { choices: vec![] };
    }
}

pub struct RadialMenuChoice {
    id: usize,
    text: String,
}
