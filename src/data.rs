use bevy::prelude::*;
use std::collections::HashMap;

#[derive(SystemLabel, Debug, Hash, PartialEq, Eq, Clone)]
pub enum Label {
    Input,
    Calculating,
    Draw,
}

#[derive(Hash, Debug, PartialEq, Eq, Clone, Copy)]
pub enum Status {
    Born,
    Dying,
    Dead,
    Alive,
}

pub enum Simulation {Running, Paused, Step}

pub struct MouseLoc(pub Vec2);
pub struct PosHash(pub HashMap<Position, Entity>);

pub struct MainCamera;

#[derive(Default, Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

pub struct Materials {
    pub cell_alive_material: Handle<ColorMaterial>,
    pub cell_dead_material: Handle<ColorMaterial>,
}

pub struct SizeInGrid {
    pub width: f32,
    pub height: f32,
}

impl SizeInGrid {
    pub fn square(x: f32) -> SizeInGrid {
        SizeInGrid {
            width: x,
            height: x,
        }
    }
}
