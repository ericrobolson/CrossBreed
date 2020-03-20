use super::*;

/// Struct that represents the form's position, height and width.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct FormPosition {
    pub x: usize,
    pub y: usize,
    pub height: usize,
    pub width: usize,
}

impl FormPosition {
    pub fn in_position(&self, x: usize, y: usize) -> bool {
        return x >= self.x && x <= self.x + self.width && y >= self.y && y <= self.y + self.height;
    }
}

/// Common functionality each form will have.
pub trait Form: FormClone {
    fn set_position(&mut self, form_position: FormPosition);
    fn get_position(&self) -> FormPosition;
    fn update(&mut self) -> Vec<(menu_events::EventId, menu_events::Events)>;
    fn rebind_data(&mut self, events: &Vec<(menu_events::EventId, menu_events::Events)>) {
        for child in self.get_children_mut().iter_mut() {
            child.rebind_data(&events);
        }
    }

    fn on_hover(&mut self);
    fn on_unhover(&mut self);
    fn on_click(&mut self, x: usize, y: usize);
    fn on_release(&mut self, x: usize, y: usize);
    fn draw(&self) -> Vec<CbMenuDrawVirtualMachine>;

    fn add_child(&mut self, child: Box<Form>) {
        let mut child = child;

        let self_pos = self.get_position();
        let mut child_pos = child.get_position();

        child_pos.width = self_pos.width;
        child_pos.width = self_pos.height;
        child_pos.x = self_pos.x;
        child_pos.y = self_pos.y;

        child.set_position(child_pos);

        self.get_children_mut().push(child);
    }

    fn get_children_mut(&mut self) -> &mut std::vec::Vec<std::boxed::Box<(dyn Form + 'static)>>;

    fn get_child_mut(&mut self, x: usize, y: usize) -> Option<&mut Box<Form>> {
        if self.get_position().in_position(x, y) {
            for child in self.get_children_mut().iter_mut() {
                if child.get_position().in_position(x, y) {
                    let child_child = child.get_child_mut(x, y);
                    if child_child.is_some() {
                        return child.get_child_mut(x, y);
                    }

                    return Some(child);
                }
            }
        }

        return None;
    }
}

pub trait FormClone {
    fn clone_box(&self) -> Box<Form>;
}

impl<T: 'static + Form + Clone> FormClone for T {
    fn clone_box(&self) -> Box<Form> {
        Box::new(self.clone())
    }
}

impl Clone for Box<Form> {
    fn clone(&self) -> Box<Form> {
        self.clone_box()
    }
}
