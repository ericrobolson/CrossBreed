use super::*;

use crate::cb_menu::gfx;
use gfx::{Color, Pallate};

#[derive(Clone)]
pub struct CbForm {
    children: Vec<Box<dyn Form>>,
    pallate: Pallate,
    form_position: FormPosition,
    outline_color: Option<Color>,
    fill_color: Option<Color>,
}

impl CbForm {
    pub fn new(pallate: Pallate) -> Self {
        return CbForm {
            pallate: pallate,
            children: vec![],
            outline_color: Some(pallate.quaternary),
            fill_color: Some(pallate.secondary),
            form_position: FormPosition {
                x: 0,
                y: 0,
                height: 100,
                width: 100,
            },
        };
    }
}

impl Form for CbForm {
    fn set_pallate(&mut self, pallate: Pallate) {
        self.pallate = pallate;
    }
    fn get_pallate(&self) -> Pallate {
        return self.pallate;
    }

    fn set_position(&mut self, form_position: FormPosition) {
        self.form_position = form_position;
    }

    fn get_position(&self) -> FormPosition {
        return self.form_position;
    }

    fn update(&mut self) {
        for child in self.children.iter_mut() {
            child.update();
        }
    }

    fn get_children_mut(&mut self) -> &mut std::vec::Vec<std::boxed::Box<(dyn Form + 'static)>> {
        return &mut self.children;
    }

    fn on_hover(&mut self) {
        self.fill_color = Some(self.pallate.accent);
    }

    fn on_unhover(&mut self) {
        self.fill_color = Some(self.pallate.secondary);

        for child in self.children.iter_mut() {
            child.on_unhover();
        }
    }

    fn on_click(&mut self) {
        println!("form clicked!");
    }
    fn on_release(&mut self) {
        println!("form released!");
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
