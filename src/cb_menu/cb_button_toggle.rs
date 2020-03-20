use super::*;

use crate::cb_menu::gfx;
use gfx::{Color, Palette};

#[derive(Clone)]
pub struct CbButtonToggle {
    children: Vec<Box<dyn Form>>,
    palette: Palette,

    pub value: bool,

    form_position: FormPosition,
    outline_color: Option<Color>,
    fill_color: Option<Color>,
}

impl CbButtonToggle {
    pub fn new(palette: Palette) -> Self {
        return CbButtonToggle {
            children: vec![],
            palette: palette,
            value: true,
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

    fn set_fill_color(&mut self) {
        if self.value {
            self.fill_color = Some(self.palette.tertiary);
        } else {
            self.fill_color = Some(self.palette.background);
        }
    }
}

impl Form for CbButtonToggle {
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
        self.fill_color = Some(self.palette.accent);
    }

    fn on_unhover(&mut self) {
        self.set_fill_color();

        for child in self.children.iter_mut() {
            child.on_unhover();
        }
    }

    fn on_click(&mut self) {}
    fn on_release(&mut self) {
        self.value = !self.value;
        self.set_fill_color();
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
