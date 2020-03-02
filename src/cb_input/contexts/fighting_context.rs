use super::*;

#[derive(Debug, Copy, Clone)]
pub struct FightingContext {
    // Movements
    pub up: State,
    pub down: State,
    pub left: State,
    pub right: State,

    // Attacks
    pub punch_light: Press,
    pub punch_heavy: Press,
    pub kick_light: Press,
    pub kick_heavy: Press,

    // Misc
    pub switch_stance: Press,
    pub light_block: State,
}

impl FightingContext {
    pub fn new() -> Self {
        return Self {
            up: State::Off,
            down: State::Off,
            left: State::Off,
            right: State::Off,

            punch_light: Press::NotPressed,
            punch_heavy: Press::NotPressed,
            kick_light: Press::NotPressed,
            kick_heavy: Press::NotPressed,

            switch_stance: Press::NotPressed,
            light_block: State::Off,
        };
    }
}

pub fn get_fighting_context(
    game_tick: GameTick,
    events: &Vec<sdl2::event::Event>,
    previous_context: &FightingContext,
) -> FightingContext {
    let mut context = previous_context.clone();

    // Reset presses
    context.punch_light = Press::NotPressed;
    context.punch_heavy = Press::NotPressed;
    context.kick_light = Press::NotPressed;
    context.kick_heavy = Press::NotPressed;
    context.switch_stance = Press::NotPressed;

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

    return context;
}
