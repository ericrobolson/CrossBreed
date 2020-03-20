use super::*;

use crate::cb_menu::gfx;
use gfx::Palette;

#[derive(Clone)]
pub struct CbFormColumn {
    form: CbForm,
}

impl CbFormColumn {
    pub fn new(palette: Palette) -> Self {
        return CbFormColumn {
            form: CbForm::new(palette),
        };
    }
}

impl Form for CbFormColumn {
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

    fn update(&mut self) {
        let self_pos = self.get_position();

        if self.get_children_mut().is_empty() == false {
            let children_count = self.get_children_mut().len();
            let children_height = self_pos.height / children_count;

            let mut current_y = self_pos.y;

            // Rescale current existing children
            for existing_child in self.get_children_mut().iter_mut() {
                let mut child_pos = existing_child.get_position();
                child_pos.x = self_pos.x;
                child_pos.y = current_y;

                child_pos.width = self_pos.width;
                child_pos.height = children_height;

                existing_child.set_position(child_pos);

                existing_child.update();

                current_y += children_height;
            }
        }
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

    fn on_click(&mut self) {
        self.form.on_click();
    }
    fn on_release(&mut self) {
        self.form.on_release();
    }

    fn draw(&self) -> Vec<CbMenuDrawVirtualMachine> {
        return self.form.draw();
    }
}
