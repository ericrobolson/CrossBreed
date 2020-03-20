#[derive(Clone)]
pub struct GuiEnvironment {
    root_form: CbForm,
    width: usize,
    height: usize,
    mouse_x: usize,
    mouse_y: usize,
    clicked_at_xy: Option<(usize, usize)>,
}

impl GuiEnvironment {
    pub fn new(width: usize, height: usize) -> Self {
        let root_form = CbForm::new();
        return Self {
            mouse_x: width / 2,
            mouse_y: height / 2,
            width: width,
            height: height,
            root_form: root_form,
            clicked_at_xy: None,
        };
    }

    fn get_form_at_mut(&mut self, x: usize, y: usize) -> Option<&mut impl Form> {
        if x >= self.root_form.form_position.x
            && x <= self.root_form.form_position.x + self.root_form.form_position.width
            && y >= self.root_form.form_position.y
            && y <= self.root_form.form_position.y + self.root_form.form_position.height
        {
            // change to do recursive search on each form?

            return Some(&mut self.root_form);
        }

        return None;
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
                    let form_result = self.get_form_at_mut(*x as usize, *y as usize);
                    if form_result.is_some() {
                        form_result.unwrap().on_hover();
                    } else {
                        self.root_form.on_unhover();
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
                    let xy = (*x as usize, *y as usize);
                    self.clicked_at_xy = Some(xy);

                    let form_result = self.get_form_at_mut(xy.0, xy.1);
                    if form_result.is_some() {
                        form_result.unwrap().on_click();
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
                        let xy = self.clicked_at_xy.unwrap();
                        self.clicked_at_xy = None;

                        let form_result = self.get_form_at_mut(xy.0, xy.1);
                        if form_result.is_some() {
                            form_result.unwrap().on_release();
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

/// Struct that represents the form's position, height and width.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct FormPosition {
    pub x: usize,
    pub y: usize,
    pub height: usize,
    pub width: usize,
}

/// Struct that represents a color
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Color {
    pub r: u8,
    pub b: u8,
    pub g: u8,
}

/// Common functionality each form will have.
pub trait Form: FormClone {
    fn set_position(&mut self, form_position: FormPosition);
    fn get_position(&self) -> FormPosition;
    fn update(&mut self);
    fn on_hover(&mut self);
    fn on_unhover(&mut self);
    fn on_click(&mut self);
    fn on_release(&mut self);
    fn draw(&self) -> Vec<CbMenuDrawVirtualMachine>;
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
    outline_color: Option<Color>,
    fill_color: Option<Color>,
    default_color: Option<Color>,
}

impl CbForm {
    pub fn new() -> Self {
        let default_color = Some(Color {
            r: 52,
            g: 52,
            b: 52,
        });

        return CbForm {
            children: vec![],
            outline_color: Some(Color { r: 0, g: 0, b: 0 }),
            fill_color: default_color,
            default_color: default_color,
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

    fn on_hover(&mut self) {
        if self.default_color.is_some() {
            self.fill_color = Some(Color {
                r: 202,
                g: 62,
                b: 71,
            });
        }
    }
    fn on_unhover(&mut self) {
        if self.default_color.is_some() {
            self.fill_color = self.default_color;
        }
    }
    fn on_click(&mut self) {
        println!("clicked!");
    }
    fn on_release(&mut self) {
        println!("released!");
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
                    self.fill_color.unwrap(),
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

pub enum CbMenuDrawVirtualMachine {
    WireframeRect(FormPosition, Color),
    FilledRect(FormPosition, Color),
}
