// Copyright 2020, Eric Olson, All rights reserved. Contact eric.rob.olson@gmail.com for questions regarding use.

// Note; do this using ECS

extern crate specs;
use specs::prelude::*;

use crate::cb_math;
use cb_math::cb_range::CbNormalizedRange;

pub mod gfx_components;
pub mod physics_components;
pub mod rts_components;
pub mod voxel_components;

use crate::cb_patterns::presenter;
use presenter::{Presenter, SliderPresenter};

pub struct EditableComponent {
    editing: bool,
}

impl EditableComponent {
    pub fn new(editing: bool) -> Self {
        return Self { editing: editing };
    }
}

pub struct RangePresentableTestComponent {
    pub range: CbNormalizedRange,
    pub presenter: SliderPresenter,
    editing_component: bool,
}

impl<'a> RangePresentableTestComponent {
    fn get_value_mut(&'a mut self) -> &'a mut CbNormalizedRange {
        return &mut self.range;
    }

    fn sync_presenter(&mut self) {
        self.presenter.set_model(self.range);
    }

    fn update_from_presenter(&mut self) {
        self.range = self.presenter.get_model();
    }

    pub fn sync(&mut self) {
        if self.editing_component {
            self.update_from_presenter();
        } else if self.presenter.editing == false {
            self.sync_presenter();
        }
    }

    pub fn new() -> Self {
        let v = Self {
            range: cb_math::cb_range::CbNormalizedRange::default(),
            presenter: SliderPresenter::new(),
            editing_component: false,
        };
        return v;
    }
}

init_component_implementations![EditableComponent, RangePresentableTestComponent];
