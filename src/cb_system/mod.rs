pub type GameTick = u32;

pub enum CbEventType {
    Input,
    Network,
}

pub struct CbEvent {
    pub tick: GameTick,
}
