#[derive(Debug, Copy, Clone)]
pub struct Range {
    pub value: i32,
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
