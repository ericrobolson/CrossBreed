// Copyright 2020, Eric Olson, All rights reserved. Contact eric.rob.olson@gmail.com for questions regarding use.

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
    pub parry_block: State,
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
            parry_block: State::Off,
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
    {
        context.punch_light = Press::NotPressed;
        context.punch_heavy = Press::NotPressed;
        context.kick_light = Press::NotPressed;
        context.kick_heavy = Press::NotPressed;
        context.switch_stance = Press::NotPressed;
    }

    // TODO: make this a configurable thing
    // State keycodes
    let up_keycodes = vec![Keycode::Space];
    let down_keycodes = vec![Keycode::S];
    let left_keycodes = vec![Keycode::A];
    let right_keycodes = vec![Keycode::D];
    let parry_block_keycodes = vec![Keycode::O];

    // Press keycodes
    let punch_light_keycodes = vec![Keycode::U];
    let punch_heavy_keycodes = vec![Keycode::I];
    let kick_light_keycodes = vec![Keycode::J];
    let kick_heavy_keycodes = vec![Keycode::K];
    let switch_stance_keycodes = vec![Keycode::L];

    // end TODO: make this a configurable thing

    // Map events
    {
        for event in events {
            match event {
                Event::KeyDown { keycode: a, .. } => {
                    if a.is_some() {
                        let keycode = a.unwrap();

                        // States
                        {
                            let state_toggle = State::On;

                            if up_keycodes.iter().any(|k| *k == keycode) {
                                context.up = state_toggle;
                            }

                            if down_keycodes.iter().any(|k| *k == keycode) {
                                context.down = state_toggle;
                            }

                            if left_keycodes.iter().any(|k| *k == keycode) {
                                context.left = state_toggle;
                            }

                            if right_keycodes.iter().any(|k| *k == keycode) {
                                context.right = state_toggle;
                            }

                            if parry_block_keycodes.iter().any(|k| *k == keycode) {
                                context.parry_block = state_toggle;
                            }
                        }

                        // Presses
                        {
                            if punch_light_keycodes.iter().any(|k| *k == keycode) {
                                context.punch_light = Press::Pressed;
                            }

                            if punch_heavy_keycodes.iter().any(|k| *k == keycode) {
                                context.punch_heavy = Press::Pressed;
                            }

                            if kick_light_keycodes.iter().any(|k| *k == keycode) {
                                context.kick_light = Press::Pressed;
                            }

                            if kick_heavy_keycodes.iter().any(|k| *k == keycode) {
                                context.kick_heavy = Press::Pressed;
                            }

                            if switch_stance_keycodes.iter().any(|k| *k == keycode) {
                                context.switch_stance = Press::Pressed;
                            }
                        }
                    }
                }
                Event::KeyUp { keycode: a, .. } => {
                    if a.is_some() {
                        let keycode = a.unwrap();

                        // States
                        {
                            let state_toggle = State::Off;

                            if up_keycodes.iter().any(|k| *k == keycode) {
                                context.up = state_toggle;
                            }

                            if down_keycodes.iter().any(|k| *k == keycode) {
                                context.down = state_toggle;
                            }

                            if left_keycodes.iter().any(|k| *k == keycode) {
                                context.left = state_toggle;
                            }

                            if right_keycodes.iter().any(|k| *k == keycode) {
                                context.right = state_toggle;
                            }

                            if parry_block_keycodes.iter().any(|k| *k == keycode) {
                                context.parry_block = state_toggle;
                            }
                        }
                    }
                }
                _ => {}
            }
        }
    }

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

    // SOCD cleaning, using standards where L + R = neutral, U + D = U
    {
        if context.left == State::On && context.right == State::On {
            context.left = State::Off;
            context.right = State::Off;
        }

        if context.down == State::On && context.up == State::On {
            context.down = State::Off;
            context.up = State::On;
        }
    }

    return context;
}
