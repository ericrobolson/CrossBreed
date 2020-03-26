// Copyright 2020, Eric Olson, All rights reserved. Contact eric.rob.olson@gmail.com for questions regarding use.
extern crate specs;
use specs::prelude::*;

use crate::cb_system;
use cb_system::{Coordinate, GameUnit};

use crate::cb_menu;
use cb_menu::{forms, menu_events, menu_events::EventId, EditorComponent, Form};

#[derive(Clone)]
pub struct FmSynthComponent {
    pub editor: FmSynthEditor,
}

impl FmSynthComponent {
    pub fn new() -> Self {
        return Self {
            editor: FmSynthEditor::new(),
        };
    }
}

#[derive(Clone)]
pub struct FmSynthEditor {
    pub editing: bool,
    pub created_menu: bool,
    pub z_index: usize,
    z_index_callback: Option<EventId>,
    toggle_active_callbacks: Vec<ActivateCallbacks>,
}

impl EditorComponent for FmSynthComponent {
    fn is_editing(&self) -> bool {
        return self.editor.editing;
    }

    fn handle_events(
        &mut self,
        events: &Vec<(menu_events::EventId, menu_events::Events)>,
        frame: usize,
    ) -> Vec<(menu_events::EventId, menu_events::Events)> {
        let mut databinding_changes = vec![];

        for (event_id, event) in events.iter() {
            for callback in self.editor.toggle_active_callbacks.iter_mut() {
                if callback.event_id == *event_id {}
            }
        }

        return databinding_changes;
    }

    fn init_editor(&mut self) -> Box<Form> {
        if self.editor.created_menu {
            panic!("already created menu!");
        }

        self.editor.created_menu = true;

        // Initialize the menu
        let mut menu;
        {
            let palette = cb_menu::gfx::Palette::new();

            menu = forms::CbForm::new(palette);
            let label = forms::CbLabel::new(palette, "Some String".to_string());

            menu.get_children_mut().push(Box::new(label));
        }

        return Box::new(menu);
    }

    fn close_editor(&mut self) {
        self.editor.reset()
    }
}

impl FmSynthEditor {
    pub fn new() -> Self {
        return Self {
            editing: false,
            z_index_callback: None,
            created_menu: false,
            z_index: 0,
            toggle_active_callbacks: vec![],
        };
    }

    pub fn reset(&mut self) {
        self.editing = false;
        self.created_menu = false;
        self.z_index_callback = None;
        self.z_index = 0;
        self.toggle_active_callbacks = vec![];
    }
}

#[derive(Clone)]
struct ActivateCallbacks {
    event_id: EventId,
    x_location: usize,
    y_location: usize,
}

impl ActivateCallbacks {
    pub fn new(event_id: EventId, x_location: usize, y_location: usize) -> Self {
        return Self {
            event_id: event_id,
            x_location: x_location,
            y_location: y_location,
        };
    }
}

init_component_implementations![FmSynthComponent];
