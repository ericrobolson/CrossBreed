use super::*;

use crate::cb_menu::gfx;
use gfx::Palette;

#[derive(Clone)]
pub struct CbFormRow {
    form: CbForm,
}

impl CbFormRow {
    pub fn new(palette: Palette) -> Self {
        return CbFormRow {
            form: CbForm::new(palette),
        };
    }
}

impl Form for CbFormRow {
    fn add_child(&mut self, child: Box<Form>) {
        self.get_children_mut().push(child);
        self.update();
    }

    fn set_position(&mut self, form_position: FormPosition) {
        self.form.set_position(form_position);
    }

    fn get_position(&self) -> FormPosition {
        return self.form.get_position();
    }

    fn update(&mut self) -> Vec<(menu_events::EventId, menu_events::Events)> {
        let mut events = vec![];

        let self_pos = self.get_position();

        if self.get_children_mut().is_empty() == false {
            let children_count = self.get_children_mut().len();
            let children_width = self_pos.width / children_count;

            let mut current_x = self_pos.x;

            // Rescale current existing children
            for existing_child in self.get_children_mut().iter_mut() {
                let mut child_pos = existing_child.get_position();
                child_pos.x = current_x;
                child_pos.y = self_pos.y;

                child_pos.width = children_width;
                child_pos.height = self_pos.height;

                existing_child.set_position(child_pos);

                let mut child_events = existing_child.update();
                events.append(&mut child_events);

                current_x += children_width;
            }
        }

        return events;
    }

    fn get_children_mut(&mut self) -> &mut std::vec::Vec<std::boxed::Box<(dyn Form + 'static)>> {
        return self.form.get_children_mut();
    }

    fn on_hover(&mut self) {
        self.form.on_hover();
    }

    fn on_unhover(&mut self) {
        self.form.on_unhover();
    }

    fn on_click(&mut self, x: usize, y: usize) {
        self.form.on_click(x, y);
    }
    fn on_release(&mut self, x: usize, y: usize) {
        self.form.on_release(x, y);
    }

    fn draw(&self) -> Vec<CbMenuDrawVirtualMachine> {
        return self.form.draw();
    }
}
