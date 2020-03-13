// Copyright 2020, Eric Olson, All rights reserved. Contact eric.rob.olson@gmail.com for questions regarding use.

use super::*;

use crate::cb_graphics;
use cb_graphics::Sdl2HardwareInterface;

fn new_shooter_context() -> CbInputContexts {
    return CbInputContexts::ShooterContext {
        networked: Networked::On,
        jump: Press::NotPressed,
        crouching: State::Off,
        running: State::Off,
        prone: State::Off,
        move_forward: State::Off,
        move_backward: State::Off,
        move_left: State::Off,
        move_right: State::Off,
        look_x: Range::new(0),
        look_y: Range::new(0),
    };
}

pub fn get_shooter_context_from_keys(
    hardware: &Sdl2HardwareInterface,
    previous_context: Option<CbInputContexts>,
) -> CbInputContexts {
    let mut ctx;
    if previous_context.is_none() {
        ctx = new_shooter_context();
    } else {
        ctx = previous_context.unwrap();
    }

    let mut new_jump = Press::NotPressed;

    let_mut_for![
        (
            new_crouching,
            new_running,
            new_prone,
            new_move_forward,
            new_move_backward,
            new_move_left,
            new_move_right
        ),
        State,
        State::Off
    ];

    let mut look_x = Range::new(0);
    let mut look_y = Range::new(0);

    match ctx {
        CbInputContexts::ShooterContext {
            networked: _,
            jump: _,
            crouching: _,
            running: _,
            prone: _,
            move_forward: _,
            move_backward: _,
            move_left: _,
            move_right: _,
            look_x: ctx_look_x,
            look_y: ctx_look_y,
        } => {
            look_x = ctx_look_x;
            look_y = ctx_look_y;

            for event in hardware.events {
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
                        new_jump = Press::Pressed;
                    }
                    // States - on
                    Event::KeyDown {
                        keycode: Some(Keycode::W),
                        ..
                    } => {
                        new_move_forward = State::On;
                    }
                    Event::KeyDown {
                        keycode: Some(Keycode::A),
                        ..
                    } => {
                        new_move_left = State::On;
                    }
                    Event::KeyDown {
                        keycode: Some(Keycode::S),
                        ..
                    } => {
                        new_move_backward = State::On;
                    }
                    Event::KeyDown {
                        keycode: Some(Keycode::D),
                        ..
                    } => {
                        new_move_right = State::On;
                    }
                    Event::KeyDown {
                        keycode: Some(Keycode::C),
                        ..
                    } => {
                        new_crouching = State::On;
                    }
                    Event::KeyDown {
                        keycode: Some(Keycode::LCtrl),
                        ..
                    } => {
                        new_prone = State::On;
                    }
                    Event::KeyDown {
                        keycode: Some(Keycode::LShift),
                        ..
                    } => {
                        new_running = State::On;
                    }
                    // States - off
                    Event::KeyUp {
                        keycode: Some(Keycode::W),
                        ..
                    } => {
                        new_move_forward = State::Off;
                    }
                    Event::KeyUp {
                        keycode: Some(Keycode::A),
                        ..
                    } => {
                        new_move_left = State::Off;
                    }
                    Event::KeyUp {
                        keycode: Some(Keycode::S),
                        ..
                    } => {
                        new_move_backward = State::Off;
                    }
                    Event::KeyUp {
                        keycode: Some(Keycode::D),
                        ..
                    } => {
                        new_move_right = State::Off;
                    }
                    Event::KeyUp {
                        keycode: Some(Keycode::C),
                        ..
                    } => {
                        new_crouching = State::Off;
                    }
                    Event::KeyUp {
                        keycode: Some(Keycode::LCtrl),
                        ..
                    } => {
                        new_prone = State::Off;
                    }
                    Event::KeyUp {
                        keycode: Some(Keycode::LShift),
                        ..
                    } => {
                        new_running = State::Off;
                    }
                    _ => {}
                }
            }
        }
        _ => {}
    }

    // Now apply cursor movements
    let cursor = sdl2::mouse::MouseState::new(hardware.pump);

    look_x.value = cursor.x();
    look_y.value = cursor.y();

    return CbInputContexts::ShooterContext {
        networked: Networked::On,
        jump: new_jump,
        crouching: new_crouching,
        running: new_running,
        prone: new_prone,
        move_forward: new_move_forward,
        move_backward: new_move_backward,
        move_left: new_move_left,
        move_right: new_move_right,
        look_x: look_x,
        look_y: look_y,
    };
}
