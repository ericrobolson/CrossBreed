use crate::cb_math;
use cb_math::cb_range::CbNormalizedRange;

pub trait Presenter<'callback, T> {
    fn new() -> Self;

    fn get_view_position(&self) -> ViewPosition;
    fn set_view_position(&mut self, view_position: ViewPosition);

    fn handle_user_input(&mut self);

    fn get_model(&self) -> CbNormalizedRange;
    fn set_model(&mut self, value: T);
    fn get_view(&self) -> SliderView;
}
pub enum ViewObjectTypes {
    Rectangle,
}

pub struct ViewObject {
    pub object_type: ViewObjectTypes,
    pub view_position: ViewPosition,
}

impl ViewObject {
    fn new(object_type: ViewObjectTypes, view_position: ViewPosition) -> Self {
        return Self {
            object_type: object_type,
            view_position: view_position,
        };
    }
}

pub trait View {
    fn get_view_objects(&self) -> Vec<ViewObject>;
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct ViewPosition {
    pub x: usize,
    pub y: usize,
    pub width: usize,
    pub height: usize,
}

pub struct SliderView {
    pub view_position: ViewPosition,
    pub model: CbNormalizedRange,
}

impl View for SliderView {
    fn get_view_objects(&self) -> Vec<ViewObject> {
        let mut objects = vec![];

        let slider_obj;
        {
            let mut slider_pos = self.view_position;
            slider_pos.width = self.view_position.width / 2;

            let max_height = self.view_position.height;
            const MIN_HEIGHT: usize = 0;

            let actual_height = self.model.map_to_range_usize(0, max_height as usize);

            let mut y_diff = max_height as i32 - actual_height as i32;

            if y_diff <= 0 {
                y_diff = 0;
            }

            let y_diff = y_diff as usize;

            slider_pos.y += y_diff;

            slider_pos.height = actual_height;

            slider_obj = ViewObject::new(ViewObjectTypes::Rectangle, slider_pos);
        }

        objects.push(slider_obj);
        return objects;
    }
}

pub struct SliderPresenter {
    model: CbNormalizedRange,
    pub view_position: ViewPosition,
    pub editing: bool,
}

impl<'callback> Presenter<'callback, CbNormalizedRange> for SliderPresenter {
    fn new() -> Self {
        let mut model = CbNormalizedRange::default(); // TODO: what should this actually map to?
        return SliderPresenter {
            editing: false,
            model: model,
            view_position: ViewPosition {
                x: 0,
                y: 0,
                width: 100,
                height: 100,
            },
        };
    }

    fn get_view_position(&self) -> ViewPosition {
        return self.view_position;
    }

    fn set_view_position(&mut self, view_position: ViewPosition) {
        self.view_position = view_position;
    }

    fn handle_user_input(&mut self) {}

    fn set_model(&mut self, value: CbNormalizedRange) {
        self.model = value;
    }

    fn get_model(&self) -> CbNormalizedRange {
        return self.model;
    }

    fn get_view(&self) -> SliderView {
        return SliderView {
            view_position: self.get_view_position(),
            model: self.get_model(),
        };
    }
}
