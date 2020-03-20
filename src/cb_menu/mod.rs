pub mod form;
pub use form::{Form, FormPosition};

pub mod cb_form;
use cb_form::CbForm;

pub mod cb_button;
use cb_button::CbFormButton;

pub mod gfx;
pub use gfx::{CbMenuDrawVirtualMachine, Pallate};

#[derive(Clone)]
pub struct GuiEnvironment {
    root_form: Box<dyn Form>,
    width: usize,
    height: usize,
    mouse_x: usize,
    mouse_y: usize,
    clicked_at_xy: Option<(usize, usize)>,
}

impl GuiEnvironment {
    pub fn new(width: usize, height: usize) -> Self {
        let pallate = Pallate::new();

        let mut root_form = CbForm::new(pallate);

        let mut button = CbFormButton::new(pallate);

        root_form.add_child(Box::new(button));

        return Self {
            mouse_x: width / 2,
            mouse_y: height / 2,
            width: width,
            height: height,
            root_form: Box::new(root_form),
            clicked_at_xy: None,
        };
    }

    pub fn update(&mut self, events: Vec<sdl2::event::Event>) {
        // Traverse all forms, doing a tree traversal
        // and go through them and adjust their positions + child positions based on the parent form's info

        events.iter().for_each(|e| {
            match e {
                sdl2::event::Event::MouseMotion {
                    timestamp: _,
                    window_id,
                    which: _,
                    mousestate: _,
                    x: x,
                    y: y,
                    xrel: _,
                    yrel: _,
                } => {
                    let x_u = *x as usize;
                    let y_u = *y as usize;

                    self.root_form.on_unhover();

                    if self.root_form.get_position().in_position(x_u, y_u) {
                        let child = self.root_form.get_child_mut(x_u, y_u);

                        if child.is_some() {
                            child.unwrap().on_hover();
                        } else {
                            self.root_form.on_hover();
                        }
                    }
                }
                sdl2::event::Event::MouseButtonDown {
                    timestamp: _,
                    window_id,
                    which: _,
                    mouse_btn: _,
                    clicks: _,
                    x: x,
                    y: y,
                } => {
                    let x_u = *x as usize;
                    let y_u = *y as usize;

                    let xy = (x_u, y_u);
                    self.clicked_at_xy = Some(xy);

                    if self.root_form.get_position().in_position(x_u, y_u) {
                        let child = self.root_form.get_child_mut(x_u, y_u);

                        if child.is_some() {
                            child.unwrap().on_click();
                        } else {
                            self.root_form.on_click();
                        }
                    }
                }
                sdl2::event::Event::MouseButtonUp {
                    timestamp: _,
                    window_id,
                    which: _,
                    mouse_btn: _,
                    clicks: _,
                    x: x,
                    y: y,
                } => {
                    if self.clicked_at_xy.is_some() {
                        let (x_u, y_u) = self.clicked_at_xy.unwrap();
                        self.clicked_at_xy = None;

                        if self.root_form.get_position().in_position(x_u, y_u) {
                            let child = self.root_form.get_child_mut(x_u, y_u);

                            if child.is_some() {
                                child.unwrap().on_release();
                            } else {
                                self.root_form.on_release();
                            }
                        }
                    }
                }
                _ => {}
            };
        });

        // Use observables to update form data?
        // Use observables to update sim data?

        self.root_form.update();
    }

    pub fn draw(&self) -> Vec<CbMenuDrawVirtualMachine> {
        return self.root_form.draw();
    }
}
