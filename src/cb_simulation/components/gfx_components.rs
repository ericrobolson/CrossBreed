// Copyright 2020, Eric Olson, All rights reserved. Contact eric.rob.olson@gmail.com for questions regarding use.

extern crate specs;
use specs::prelude::*;

use super::ComponentLinker;

init_components![
    GfxComponentsLinker,
    (CameraComponent, SpriteComponent, SpriteRenderComponent)
];

pub struct SpriteComponent {}

impl SpriteComponent {
    pub fn new() -> Self {
        return Self {};
    }
}

#[derive(Clone)]
pub struct SpriteRenderComponent {}

impl SpriteRenderComponent {
    pub fn new() -> Self {
        return Self {};
    }
}

pub struct CameraComponent {
    pub camera_orthographic_view: bool,

    //NOTE: Maybe this seems wrong and should instead use the normal position components?
    pub camera_pos_x: i32,
    pub camera_pos_y: i32,
    pub camera_pos_z: i32,

    pub camera_target_x: i32,
    pub camera_target_y: i32,
    pub camera_target_z: i32,

    pub camera_pitch: i32,
    pub camera_yaw: i32,
    pub camera_roll: i32,
}

impl CameraComponent {
    pub fn new() -> Self {
        return Self {
            camera_orthographic_view: false,
            camera_pos_x: 0,
            camera_pos_y: 0,
            camera_pos_z: 0,

            camera_target_x: 0,
            camera_target_y: 0,
            camera_target_z: 0,
            camera_pitch: 0,
            camera_yaw: 0,
            camera_roll: 0,
        };
    }
}
