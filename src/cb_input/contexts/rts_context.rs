use super::*;

#[derive(Debug, Copy, Clone)]
pub struct RtsUnitContext {
    pub select: Press,
    pub move_unit: Press,
    pub attack_move_unit: Press,
    pub hold: Press,
    pub cancel_queued_actions: Press,
    pub activate_ability: Press,
    pub cursor_x: Range,
    pub cursor_y: Range,
}

/*
    A unit has the following:
    ** Move speed
    ** Point cost
    ** Base size (like 24mm, 40mm, 50mm, 60mm, etc. in 40k)
    ** Flyer
    ** Passive Ability (or something that the player does not control)
    ** Active Ability (can toggle to autouse)
    ** Health points
    ** Armor points
    ** Attack range
    ** Attack damage
    ** Attack rate
*/

impl RtsUnitContext {
    pub fn new() -> Self {
        return Self {
            select: Press::NotPressed,
            move_unit: Press::NotPressed,
            attack_move_unit: Press::NotPressed,
            hold: Press::NotPressed,
            cancel_queued_actions: Press::NotPressed,
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
