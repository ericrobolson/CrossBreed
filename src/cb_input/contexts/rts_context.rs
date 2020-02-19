use super::*;

#[derive(Debug, Copy, Clone)]
pub struct RtsUnitContext {
    pub select: Press,
    pub target: Press,
    pub cancel: Press,

    pub move_unit: Press,
    pub attack_move_unit: Press,
    pub activate_ability: Press,
    pub cursor_x: Range,
    pub cursor_y: Range,
}

impl RtsUnitContext {
    pub fn new() -> Self {
        return Self {
            select: Press::NotPressed,
            target: Press::NotPressed,
            cancel: Press::NotPressed,
            move_unit: Press::NotPressed,
            attack_move_unit: Press::NotPressed,
            activate_ability: Press::NotPressed,
            cursor_x: Range::new(0),
            cursor_y: Range::new(0),
        };
    }
}

pub fn get_rts_movement_context(
    game_tick: GameTick,
    events: &Vec<sdl2::event::Event>,
    previous_context: &RtsUnitContext,
) -> RtsUnitContext {
    let mut context = previous_context.clone();
    // TODO: make this a configurable thing
    for event in events {
        match event {
            Event::Quit { .. }
            | Event::KeyDown {
                keycode: Some(Keycode::Escape),
                ..
            } => {
                //exit = true;
            }
            _ => {}
        }
    }

    return RtsUnitContext::new();
}
