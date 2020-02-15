pub type GameTick = u32;
pub type PlayerId = u8;

pub const FRAMEDELAY: GameTick = 3;

pub struct CbEvent<T> {
    pub tick: GameTick,
    pub value: T,
}
