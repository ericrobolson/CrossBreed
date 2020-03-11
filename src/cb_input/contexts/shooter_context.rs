use super::*;

#[derive(Debug, Copy, Clone)]
pub struct ShooterMovementContext {
    pub jump: Press,
    pub crouching: State,
    pub running: State,
    pub prone: State,
    pub move_forward: State,
    pub move_backward: State,
    pub move_left: State,
    pub move_right: State,
}

impl ShooterMovementContext {
    pub fn new() -> Self {
        return Self {
            jump: Press::NotPressed,
            crouching: State::Off,
            running: State::Off,
            prone: State::Off,
            move_forward: State::Off,
            move_backward: State::Off,
            move_left: State::Off,
            move_right: State::Off,
        };
    }

    pub fn get_shooter_movement_context(
        game_tick: usize,
        events: &Vec<sdl2::event::Event>,
        previous_context: &ShooterMovementContext,
    ) -> ShooterMovementContext {
        let mut context = previous_context.clone();

        // Make all presses not pressed
        context.jump = Press::NotPressed;

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
                // Presses
                Event::KeyDown {
                    keycode: Some(Keycode::Space),
                    ..
                } => {
                    context.jump = Press::Pressed;
                }
                // States - on
                Event::KeyDown {
                    keycode: Some(Keycode::W),
                    ..
                } => {
                    context.move_forward = State::On;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::A),
                    ..
                } => {
                    context.move_left = State::On;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::S),
                    ..
                } => {
                    context.move_backward = State::On;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::D),
                    ..
                } => {
                    context.move_right = State::On;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::C),
                    ..
                } => {
                    context.crouching = State::On;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::LCtrl),
                    ..
                } => {
                    context.prone = State::On;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::LShift),
                    ..
                } => {
                    context.running = State::On;
                }
                // States - off
                Event::KeyUp {
                    keycode: Some(Keycode::W),
                    ..
                } => {
                    context.move_forward = State::Off;
                }
                Event::KeyUp {
                    keycode: Some(Keycode::A),
                    ..
                } => {
                    context.move_left = State::Off;
                }
                Event::KeyUp {
                    keycode: Some(Keycode::S),
                    ..
                } => {
                    context.move_backward = State::Off;
                }
                Event::KeyUp {
                    keycode: Some(Keycode::D),
                    ..
                } => {
                    context.move_right = State::Off;
                }
                Event::KeyUp {
                    keycode: Some(Keycode::C),
                    ..
                } => {
                    context.crouching = State::Off;
                }
                Event::KeyUp {
                    keycode: Some(Keycode::LCtrl),
                    ..
                } => {
                    context.prone = State::Off;
                }
                Event::KeyUp {
                    keycode: Some(Keycode::LShift),
                    ..
                } => {
                    context.running = State::Off;
                }
                _ => {}
            }
        }

        return context;
    }
}
