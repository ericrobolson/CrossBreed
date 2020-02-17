#[derive(Debug, Copy, Clone)]
pub struct Range {
    pub value: i32,
}

impl Range {
    pub fn new(value: i32) -> Self {
        return Range { value: value };
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Press {
    Pressed,
    NotPressed,
}

#[derive(Debug, Copy, Clone)]
pub enum State {
    On,
    Off,
}
