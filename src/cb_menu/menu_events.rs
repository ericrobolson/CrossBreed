use uuid::Uuid;

pub type EventId = Uuid;

use crate::cb_math::cb_range::CbNormalizedRange;

pub fn EventId_new() -> EventId {
    return Uuid::new_v4();
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Events {
    SingleRangeChange(CbNormalizedRange),
    BoolValueChange(bool),
}

pub struct BoolEvent {
    pub value: bool,
    pub id: EventId,
}
