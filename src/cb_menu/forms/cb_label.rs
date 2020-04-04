use super::*;

use crate::cb_menu;
use cb_menu::{
    form::{Form, FormPosition},
    gfx::{CbMenuDrawVirtualMachine, Color, Palette},
    menu_events,
};

#[derive(Clone)]
pub struct CbLabel {
    children: Vec<Box<dyn Form>>,
    palette: Palette,
    label: String,
    form_position: FormPosition,
    text_color: Option<Color>,
}

impl CbLabel {
    pub fn new(palette: Palette, text: String) -> Self {
        return CbLabel {
            children: vec![],
            palette: palette,
            label: text,
            text_color: Some(palette.quaternary),
            form_position: FormPosition {
                x: 0,
                y: 0,
                height: 100,
                width: 100,
            },
        };
    }
}

impl Form for CbLabel {
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

    fn on_click(&mut self, x: usize, y: usize) {}
    fn on_release(&mut self, x: usize, y: usize) {}

    fn draw(&self) -> Vec<CbMenuDrawVirtualMachine> {
        let mut draw_calls = vec![];

        // Draw self stuff
        {
            let text_color;
            {
                if self.text_color.is_some() {
                    text_color = self.text_color.unwrap();
                } else {
                    text_color = Color::black();
                }
            }

            let call =
                CbMenuDrawVirtualMachine::Text(self.form_position, text_color, self.label.clone());

            draw_calls.push(call);
        }

        for child in self.children.iter() {
            draw_calls.append(&mut child.draw());
        }

        return draw_calls;
    }
}
