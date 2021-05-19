pub const ARENA_WIDTH: i32 = 50;
pub const ARENA_HEIGHT: i32 = 50;

use crate::data::*;
use bevy::prelude::*;

pub fn size_scaling(windows: Res<Windows>, mut q: Query<(&SizeInGrid, &mut Sprite)>) {
    let window = windows.get_primary().unwrap();
    for (sprite_size, mut sprite) in q.iter_mut() {
        sprite.size = Vec2::new(
            sprite_size.width / ARENA_WIDTH as f32 * window.width() as f32,
            sprite_size.height / ARENA_HEIGHT as f32 * window.height() as f32,
        );
    }
}

pub fn position_translation(windows: Res<Windows>, mut q: Query<(&Position, &mut Transform)>) {
    let window = windows.get_primary().unwrap();
    for (pos, mut transform) in q.iter_mut() {
        transform.translation = Vec3::new(
            grid_to_world(pos.x as f32, window.width() as f32, ARENA_WIDTH as f32),
            grid_to_world(pos.y as f32, window.height() as f32, ARENA_HEIGHT as f32),
            0.,
        );
    }
}

pub fn world_pos_to_grid_pos(world_pos: Vec2, window: &Window) -> Position {
    let tile_height = window.height() / ARENA_HEIGHT as f32;
    let tile_width = window.width() / ARENA_WIDTH as f32;

    let grid_pos_x = world_pos.x / tile_width + (ARENA_WIDTH as f32 / 2.0);
    let grid_pos_y = world_pos.y / tile_height + (ARENA_HEIGHT as f32 / 2.0);

    let grid_pos = Vec2::new(grid_pos_x, grid_pos_y).floor();

    Position {
        x: grid_pos.x as i32,
        y: grid_pos.y as i32,
    }
}

pub fn get_neigbour_positions(pos: &Position) -> [Position; 8] {
    [
        Position {
            x: pos.x - 1,
            y: pos.y + 1,
        },
        Position {
            x: pos.x,
            y: pos.y + 1,
        },
        Position {
            x: pos.x + 1,
            y: pos.y + 1,
        },
        Position {
            x: pos.x - 1,
            y: pos.y,
        },
        Position {
            x: pos.x + 1,
            y: pos.y,
        },
        Position {
            x: pos.x - 1,
            y: pos.y - 1,
        },
        Position {
            x: pos.x,
            y: pos.y - 1,
        },
        Position {
            x: pos.x + 1,
            y: pos.y - 1,
        },
    ]
}

pub fn grid_to_world(pos: f32, bound_window: f32, bound_game: f32) -> f32 {
    let tile_size = bound_window / bound_game;
    pos / bound_game * bound_window - (bound_window / 2.) + (tile_size / 2.)
}

pub fn get_bevy_color(r: u8, g: u8, b: u8) -> Color {
    Color::rgb(r as f32 / 255., g as f32 / 255., b as f32 / 255.)
}
