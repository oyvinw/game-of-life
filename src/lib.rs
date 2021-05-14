use bevy::prelude::*;
use bevy::render::pass::ClearColor;
use crate::data::*;

mod helpers;
mod data;

pub fn run() {
    App::build()
        //app setup
        .insert_resource(WindowDescriptor {
            title: "rgol".to_string(),
            width: helpers::ARENA_WIDTH as f32 * 20.0,
            height: helpers::ARENA_HEIGHT as f32 * 20.0,
            resizable: false,
            ..Default::default()
        })
        .insert_resource(ClearColor(helpers::get_bevy_color(146, 131, 116)))
        .add_startup_system(setup.system())
        .add_startup_stage("populate with dead cells", SystemStage::single(populate_dead.system()))

        //helper systems
        .add_system_set_to_stage(
            CoreStage::PostUpdate,
            SystemSet::new()
                .with_system(helpers::position_translation.system())
                .with_system(helpers::size_scaling.system())
        )
        .add_plugins(DefaultPlugins)
        .run();
}

fn setup (mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>){
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.insert_resource(Materials {
        cell_alive_material: materials.add(helpers::get_bevy_color(184, 187, 38).into()),
        cell_dead_material: materials.add(helpers::get_bevy_color(40, 40, 40).into()),
    });
}

fn change_materials(mut commands: Commands, materials: Res<Materials>, q: Query<(&Cell, &Sprite)>){
    for (cell, sprite) in q.iter(){
        if (cell.alive){
            sprite = materials.cell_alive_material;
        } else {
            sprite.material: materials.cell_dead_material;
        }
    }

fn populate_dead(mut commands: Commands, materials: Res<Materials>){
    for x in 0..helpers::ARENA_WIDTH {
        for y in 0..helpers::ARENA_HEIGHT {
            commands.spawn_bundle(SpriteBundle {
                material: materials.cell_dead_material.clone(),
                ..Default::default()
            })
            .insert(Cell{ alive: false })
            .insert(Position{ x, y })
            .insert(SizeInGrid::square(0.9));
        }
    }
}

