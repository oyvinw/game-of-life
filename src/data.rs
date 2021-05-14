use bevy::prelude::*;

#[derive(Default, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Position {
    pub x: u32,
    pub y: u32,
}

pub struct Cell {
    pub alive: bool,
}

pub struct Materials {
    pub cell_alive_material: Handle<ColorMaterial>,
    pub cell_dead_material: Handle<ColorMaterial>,
}

pub struct SizeInGrid {
    pub width: f32,
    pub height: f32,
}

impl SizeInGrid{
    pub fn square(x: f32) -> SizeInGrid {
        SizeInGrid {
            width: x,
            height: x,
        }
    }
}