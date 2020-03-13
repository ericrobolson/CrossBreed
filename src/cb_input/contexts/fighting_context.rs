// Copyright 2020, Eric Olson, All rights reserved. Contact eric.rob.olson@gmail.com for questions regarding use.

use super::*;

pub fn fighting_context_to_bytes() {
    unimplemented!();
}

pub fn fighting_context_from_bytes() {
    unimplemented!(); /* Need this to prevent hacking
                      // SOCD cleaning, using standards where L + R = neutral, U + D = U
                               {
                                   if new_left == State::On && new_right == State::On {
                                       new_left = State::Off;
                                       new_right = State::Off;
                                   }

                                   if new_down == State::On && new_up == State::On {
                                       new_down = State::Off;
                                       new_up = State::On;
                                   }
                               }
                       */
}

pub fn get_fighting_context_from_keys(
    events: &Vec<sdl2::event::Event>,
    previous_context: &CbInputContexts,
) -> CbInputContexts {
    let_mut_for![(new_up, new_down, new_left, new_right), State, State::Off];

    let_mut_for![
        (
            new_punch_light,
            new_punch_heavy,
            new_kick_light,
            new_kick_heavy
        ),
        Press,
        Press::NotPressed
    ];

    match previous_context {
        CbInputContexts::FightingContext {
            networked: _,
            up: _,
            down: _,
            left: _,
            right: _,
            punch_light: _,
            punch_heavy: _,
            kick_light: _,
            kick_heavy: _,
        } => {
            // TODO: make this a configurable thing
            // State keycodes
            let up_keycodes = vec![Keycode::Space];
            let down_keycodes = vec![Keycode::S];
            let left_keycodes = vec![Keycode::A];
            let right_keycodes = vec![Keycode::D];
            // Press keycodes
            let punch_light_keycodes = vec![Keycode::U];
            let punch_heavy_keycodes = vec![Keycode::I];
            let kick_light_keycodes = vec![Keycode::J];
            let kick_heavy_keycodes = vec![Keycode::K];

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
                                        new_up = state_toggle;
                                    }

                                    if down_keycodes.iter().any(|k| *k == keycode) {
                                        new_down = state_toggle;
                                    }

                                    if left_keycodes.iter().any(|k| *k == keycode) {
                                        new_left = state_toggle;
                                    }

                                    if right_keycodes.iter().any(|k| *k == keycode) {
                                        new_right = state_toggle;
                                    }
                                }

                                // Presses
                                {
                                    if punch_light_keycodes.iter().any(|k| *k == keycode) {
                                        new_punch_light = Press::Pressed;
                                    }

                                    if punch_heavy_keycodes.iter().any(|k| *k == keycode) {
                                        new_punch_heavy = Press::Pressed;
                                    }

                                    if kick_light_keycodes.iter().any(|k| *k == keycode) {
                                        new_kick_light = Press::Pressed;
                                    }

                                    if kick_heavy_keycodes.iter().any(|k| *k == keycode) {
                                        new_kick_heavy = Press::Pressed;
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
                                        new_up = state_toggle;
                                    }

                                    if down_keycodes.iter().any(|k| *k == keycode) {
                                        new_down = state_toggle;
                                    }

                                    if left_keycodes.iter().any(|k| *k == keycode) {
                                        new_left = state_toggle;
                                    }

                                    if right_keycodes.iter().any(|k| *k == keycode) {
                                        new_right = state_toggle;
                                    }
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }

            // SOCD cleaning, using standards where L + R = neutral, U + D = U
            {
                if new_left == State::On && new_right == State::On {
                    new_left = State::Off;
                    new_right = State::Off;
                }

                if new_down == State::On && new_up == State::On {
                    new_down = State::Off;
                    new_up = State::On;
                }
            }
        }
        _ => {}
    }

    return CbInputContexts::FightingContext {
        networked: Networked::On,
        up: new_up,
        down: new_down,
        left: new_left,
        right: new_right,
        punch_light: new_punch_light,
        punch_heavy: new_punch_heavy,
        kick_light: new_kick_light,
        kick_heavy: new_kick_heavy,
    };
    /*


    return context;
    */
}
