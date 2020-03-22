use super::*;

use crate::cb_menu::gfx;
use gfx::{Color, Palette};

#[derive(Clone)]
pub struct CbForm {
    children: Vec<Box<dyn Form>>,
    palette: Palette,
    form_position: FormPosition,
    outline_color: Option<Color>,
    fill_color: Option<Color>,
}

impl CbForm {
    pub fn new(palette: Palette) -> Self {
        return CbForm {
            palette: palette,
            children: vec![],
            outline_color: Some(palette.quaternary),
            fill_color: Some(palette.secondary),
            form_position: FormPosition {
                x: 0,
                y: 0,
                height: 480,
                width: 640,
            },
        };
    }

    pub fn reset(&mut self) {
        self.children = vec![];
    }
}

impl Form for CbForm {
    fn set_position(&mut self, form_position: FormPosition) {
        self.form_position = form_position;
    }

    fn get_position(&self) -> FormPosition {
        return self.form_position;
    }

    fn update(&mut self) -> Vec<(menu_events::EventId, menu_events::Events)> {
        let mut events = vec![];
        for child in self.children.iter_mut() {
            let mut child_events = child.update();

            events.append(&mut child_events);
        }

        return events;
    }

    fn get_children_mut(&mut self) -> &mut std::vec::Vec<std::boxed::Box<(dyn Form + 'static)>> {
        return &mut self.children;
    }

    fn on_hover(&mut self) {}

    fn on_unhover(&mut self) {
        for child in self.children.iter_mut() {
            child.on_unhover();
        }
    }

    fn on_click(&mut self, x: usize, y: usize) {
        println!("form clicked!");
    }
    fn on_release(&mut self, x: usize, y: usize) {
        println!("form released!");
    }

    fn draw(&self) -> Vec<CbMenuDrawVirtualMachine> {
        let mut draw_calls = vec![];

        for child in self.children.iter() {
            draw_calls.append(&mut child.draw());
        }

        return draw_calls;
    }
}
