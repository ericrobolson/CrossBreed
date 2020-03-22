use super::*;

use menu_events::{BoolEvent, EventId, EventId_new, Events};

use crate::cb_menu::gfx;
use gfx::{Color, Palette};

#[derive(Clone)]
pub struct CbButtonToggle {
    event_id: EventId,
    children: Vec<Box<dyn Form>>,
    palette: Palette,

    pub value: bool,

    events: Vec<(EventId, Events)>,

    form_position: FormPosition,
    outline_color: Option<Color>,
    fill_color: Option<Color>,
}

impl CbButtonToggle {
    pub fn new(palette: Palette) -> Self {
        return CbButtonToggle {
            event_id: EventId_new(),
            events: vec![],
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

    pub fn subscribe_to_event(&self) -> EventId {
        return self.event_id;
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

    fn rebind_data(&mut self, events: &Vec<(menu_events::EventId, menu_events::Events)>) {
        for (id, event) in events.iter() {
            if *id == self.event_id {
                match event {
                    menu_events::Events::BoolValueChange(value) => {
                        self.value = *value;
                    }
                    menu_events::Events::SingleRangeChange(value) => {
                        self.value = value.value >= 0;
                    }
                }
            }
        }

        for child in self.get_children_mut().iter_mut() {
            child.rebind_data(events);
        }
    }

    fn get_position(&self) -> FormPosition {
        return self.form_position;
    }

    fn update(&mut self) -> Vec<(menu_events::EventId, menu_events::Events)> {
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

    fn on_hover(&mut self) {
        self.fill_color = Some(self.palette.accent);
    }

    fn on_unhover(&mut self) {
        self.set_fill_color();

        for child in self.children.iter_mut() {
            child.on_unhover();
        }
    }

    fn on_click(&mut self, x: usize, y: usize) {}
    fn on_release(&mut self, x: usize, y: usize) {
        self.value = !self.value;

        self.set_fill_color();

        self.events
            .push((self.event_id, Events::BoolValueChange(self.value)));
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
