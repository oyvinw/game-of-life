pub const ARENA_WIDTH: u32 = 30;
pub const ARENA_HEIGHT: u32 = 30;

use bevy::prelude::*;
use crate::data::*;

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
    fn convert(pos: f32, bound_window: f32, bound_game: f32) -> f32 {
        let tile_size = bound_window / bound_game;
        pos / bound_game * bound_window - (bound_window / 2.) + (tile_size / 2.)
    }
    let window = windows.get_primary().unwrap();
    for (pos, mut transform) in q.iter_mut() {
        transform.translation = Vec3::new(
            convert(pos.x as f32, window.width() as f32, ARENA_WIDTH as f32),
            convert(pos.y as f32, window.height() as f32, ARENA_HEIGHT as f32),
            0.,
        );
    }
}

pub fn get_bevy_color(r: u8, g: u8, b: u8) -> Color {
    Color::rgb(r as f32 / 255., g as f32 / 255., b as f32 / 255.)
}