pub type GameTick = u32;
pub type PlayerId = u8;

pub type GameUnit = i32;

pub const FRAMEDELAY: GameTick = 3;

pub struct Coordinate {
    pub x: GameUnit,
    pub y: GameUnit,
    pub z: GameUnit,
}

impl Coordinate {
    pub fn new(x: GameUnit, y: GameUnit, z: GameUnit) -> Self {
        return Self { x: x, y: y, z: z };
    }
}

pub struct CbEvent<T> {
    pub tick: GameTick,
    pub value: T,
}
