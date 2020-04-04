use super::*;

use crate::cb_menu;
use cb_menu::{
    form::{Form, FormPosition},
    gfx::{CbMenuDrawVirtualMachine, Color, Palette},
    menu_events,
};

#[derive(Clone)]
pub struct CbButton {
    children: Vec<Box<dyn Form>>,
    palette: Palette,

    form_position: FormPosition,
    outline_color: Option<Color>,
    fill_color: Option<Color>,
}

impl CbButton {
    pub fn new(palette: Palette) -> Self {
        return CbButton {
            children: vec![],
            palette: palette,
            outline_color: Some(palette.quaternary),
            fill_color: Some(palette.primary),
            form_position: FormPosition {
                x: 0,
                y: 0,
                height: 100,
                width: 100,
            },
        };
    }
}

impl Form for CbButton {
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

    fn on_hover(&mut self) {
        self.fill_color = Some(self.palette.accent);
    }

    fn on_unhover(&mut self) {
        self.fill_color = Some(self.palette.primary);

        for child in self.children.iter_mut() {
            child.on_unhover();
        }
    }

    fn on_click(&mut self, x: usize, y: usize) {
        println!("button clicked!");
    }
    fn on_release(&mut self, x: usize, y: usize) {
        println!("button released!");
    }

    fn draw(&self) -> Vec<CbMenuDrawVirtualMachine> {
        let mut draw_calls = vec![];

        // Draw self stuff
        {
            if self.fill_color.is_some() {
                let call = CbMenuDrawVirtualMachine::FilledRect(
                    self.form_position,
                    self.fill_color.unwrap(),
                );

                draw_calls.push(call);
            }

            if self.outline_color.is_some() {
                let call = CbMenuDrawVirtualMachine::WireframeRect(
                    self.form_position,
                    self.outline_color.unwrap(),
                );

                draw_calls.push(call);
            }
        }

        for child in self.children.iter() {
            draw_calls.append(&mut child.draw());
        }

        return draw_calls;
    }
}
