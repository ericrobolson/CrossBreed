pub mod form;
pub use form::{Form, FormPosition};

pub mod menu_events;

pub mod forms;
use forms::CbForm;

pub mod gfx;
pub use gfx::{CbMenuDrawVirtualMachine, Palette};

#[derive(Clone)]
pub struct GuiEnvironment {
    root_form: Box<CbForm>,
    events: Vec<(menu_events::EventId, menu_events::Events)>,
    width: usize,
    height: usize,
    mouse_x: usize,
    mouse_y: usize,
    clicked_at_xy: Option<(usize, usize)>,
}

pub trait EditorComponent {
    /// Pass in events to the editor, and return any events for databinding.
    fn handle_events(
        &mut self,
        events: &Vec<(menu_events::EventId, menu_events::Events)>,
        frame: usize,
    ) -> Vec<(menu_events::EventId, menu_events::Events)>;

    /// Initialize the editor. Will panic if already implemented.
    fn init_editor(&mut self) -> Box<Form>;
    /// Return whether the current component has an editor opened.
    fn is_editing(&self) -> bool;
    /// Close the current editor.
    fn close_editor(&mut self);
}

impl GuiEnvironment {
    pub fn new(width: usize, height: usize) -> Self {
        return Self {
            events: vec![],
            mouse_x: width / 2,
            mouse_y: height / 2,
            width: width,
            height: height,
            root_form: Box::new(CbForm::new(Palette::new())),
            clicked_at_xy: None,
        };
    }

    pub fn handle_databinding_changes(
        &mut self,
        events: &Vec<(menu_events::EventId, menu_events::Events)>,
    ) {
        self.root_form.rebind_data(events);
    }

    pub fn add_form(&mut self, form: Box<Form>) {
        self.root_form.add_child(form);
    }

    pub fn reset(&mut self) {
        self.root_form.reset();
    }

    pub fn get_events(&self) -> Vec<(menu_events::EventId, menu_events::Events)> {
        return self.events.clone();
    }

    pub fn update(&mut self, events: Vec<sdl2::event::Event>) {
        // Traverse all forms, doing a tree traversal
        // and go through them and adjust their positions + child positions based on the parent form's info

        self.events.clear();

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
                            child.unwrap().on_click(x_u, y_u);
                        } else {
                            self.root_form.on_click(x_u, y_u);
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
                        let (start_x_u, start_y_u) = self.clicked_at_xy.unwrap();
                        self.clicked_at_xy = None;

                        if self
                            .root_form
                            .get_position()
                            .in_position(start_x_u, start_y_u)
                        {
                            let child = self.root_form.get_child_mut(start_x_u, start_y_u);

                            let (x_u, y_u) = (*x as usize, *y as usize);

                            if child.is_some() {
                                child.unwrap().on_release(x_u, y_u);
                            } else {
                                self.root_form.on_release(x_u, y_u);
                            }
                        }
                    }
                }
                _ => {}
            };
        });

        // Use observables to update form data?
        // Use observables to update sim data?

        let events = self.root_form.update();

        self.events = events;
    }

    pub fn draw(&self) -> Vec<CbMenuDrawVirtualMachine> {
        return self.root_form.draw();
    }
}
