use super::*;

use crate::cb_menu;
use cb_menu::{
    form::{Form, FormPosition},
    gfx::{CbMenuDrawVirtualMachine, Color, Palette},
    menu_events::{BoolEvent, EventId, EventId_new, Events},
};

use crate::cb_math::cb_range;
use cb_range::CbNormalizedRange;

#[derive(Clone)]
pub struct CbSliderHorizontal {
    event_id: EventId,
    events: Vec<(EventId, Events)>,
    children: Vec<Box<dyn Form>>,
    palette: Palette,

    pub x_value: CbNormalizedRange,
    form_position: FormPosition,
    outline_color: Option<Color>,
    fill_color: Option<Color>,

    start_xy: Option<(usize, usize)>,
}

impl CbSliderHorizontal {
    pub fn new(palette: Palette) -> Self {
        return CbSliderHorizontal {
            event_id: EventId_new(),
            events: vec![],
            children: vec![],
            palette: palette,
            x_value: CbNormalizedRange::default(),
            outline_color: Some(palette.quaternary),
            fill_color: Some(palette.primary),
            start_xy: None,
            form_position: FormPosition {
                x: 0,
                y: 0,
                height: 100,
                width: 100,
            },
        };
    }

    pub fn subscribe_to_event(&self) -> EventId {
        return self.event_id;
    }
}

impl Form for CbSliderHorizontal {
    fn rebind_data(&mut self, events: &Vec<(EventId, Events)>) {
        for (id, event) in events.iter() {
            if *id == self.event_id {
                match event {
                    Events::BoolValueChange(value) => {
                        if *value {
                            self.x_value.value = self.x_value.max();
                        } else {
                            self.x_value.value = self.x_value.min();
                        }
                    }
                    Events::SingleRangeChange(value) => {
                        self.x_value = *value;
                    }
                }
            }
        }

        for child in self.get_children_mut().iter_mut() {
            child.rebind_data(events);
        }
    }

    fn set_position(&mut self, form_position: FormPosition) {
        self.form_position = form_position;
    }

    fn get_position(&self) -> FormPosition {
        return self.form_position;
    }

    fn update(&mut self) -> Vec<(EventId, Events)> {
        let mut events = vec![];

        events.append(&mut self.events);
        self.events.clear();

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
    fn on_release(&mut self, x: usize, y: usize) {
        {
            let (start_x, start_y) = (self.form_position.x, self.form_position.y);

            let (start_x, start_y) = (start_x as i32, start_y as i32);
            let (x, y) = (x as i32, y as i32);

            let mut xdiff = x - start_x;
            let mut ydiff = y - start_y;

            if xdiff < 0 {
                xdiff = 0;
            } else if xdiff > self.form_position.width as i32 {
                xdiff = self.form_position.width as i32;
            }

            let xvalue = xdiff as usize;

            if ydiff < 0 {
                ydiff = 0;
            } else if ydiff > self.form_position.height as i32 {
                ydiff = self.form_position.height as i32;
            }

            let yvalue = ydiff as usize;

            let xrange = CbNormalizedRange::new(xvalue as i32, 0, self.form_position.width as i32);
            let yrange = CbNormalizedRange::new(yvalue as i32, 0, self.form_position.height as i32);

            self.x_value = xrange;

            self.events
                .push((self.event_id, Events::SingleRangeChange(self.x_value)));
        }
    }

    fn draw(&self) -> Vec<CbMenuDrawVirtualMachine> {
        let mut draw_calls = vec![];

        // Draw self stuff
        {
            // Draw background
            if self.fill_color.is_some() {
                let call = CbMenuDrawVirtualMachine::FilledRect(
                    self.form_position,
                    self.fill_color.unwrap(),
                );

                draw_calls.push(call);
            }

            // Slider
            {
                let mut slider_pos = self.form_position;

                let slider_width = self.x_value.map_to_range_usize(0, self.form_position.width);
                slider_pos.width = slider_width;

                let call = CbMenuDrawVirtualMachine::FilledRect(slider_pos, self.palette.accent);

                draw_calls.push(call);

                let call = CbMenuDrawVirtualMachine::WireframeRect(
                    slider_pos,
                    self.outline_color.unwrap(),
                );

                draw_calls.push(call);
            }

            // Draw outline
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
