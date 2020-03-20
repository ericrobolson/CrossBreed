#[derive(Clone)]
pub struct GuiEnvironment {
    root_form: CbForm,
}

impl GuiEnvironment {
    pub fn new() -> Self {
        return Self {
            root_form: CbForm::new(),
        };
    }

    pub fn update(&mut self, events: Vec<sdl2::event::Event>) {
        // Traverse all forms, doing a tree traversal
        // and go through them and adjust their positions + child positions based on the parent form's info

        // Use observables to update form data?
        // Use observables to update sim data?

        self.root_form.update();
    }

    pub fn draw(&self) {
        self.root_form.draw();
    }
}

/// Struct that represents the form's position, height and width.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct FormPosition {
    pub x: usize,
    pub y: usize,
    pub height: usize,
    pub width: usize,
}

/// Common functionality each form will have.
pub trait Form: FormClone {
    fn set_position(&mut self, form_position: FormPosition);
    fn get_position(&self) -> FormPosition;
    fn update(&mut self);
    fn on_hover(&mut self);
    fn on_click(&mut self);
    fn on_release(&mut self);
    fn draw(&self);
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

#[derive(Clone)]
pub struct CbForm {
    children: Vec<Box<dyn Form>>,
    form_position: FormPosition,
}

impl CbForm {
    pub fn new() -> Self {
        return CbForm {
            children: vec![],
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

    fn on_hover(&mut self) {}
    fn on_click(&mut self) {}
    fn on_release(&mut self) {}

    fn draw(&self) {
        // Draw self stuff

        for child in self.children.iter() {
            child.draw();
        }
    }
}
