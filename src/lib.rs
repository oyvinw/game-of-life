use crate::data::*;
use bevy::core::FixedTimestep;
use bevy::prelude::*;
use bevy::render::pass::ClearColor;
use std::collections::HashMap;
use bevy::ecs::schedule::ShouldRun;
use wasm_bindgen::prelude::*;


mod data;
mod helpers;

#[wasm_bindgen]
pub fn run() {
    let mut app = App::build();

    #[cfg(target_arch = "wasm32")]
    app.add_plugin(bevy_webgl2::WebGL2Plugin);
    
    app.insert_resource(WindowDescriptor {
            title: "rgol".to_string(),
            width: helpers::ARENA_WIDTH as f32 * 20.0,
            height: helpers::ARENA_HEIGHT as f32 * 20.0,
            resizable: false,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(helpers::get_bevy_color(60, 56, 54)))
        .insert_resource(MouseLoc(Vec2::new(0.0, 0.0)))
        .insert_resource(PosHash(HashMap::new()))
        .insert_resource(Simulation::Paused)
        .add_startup_system(setup.system())
        .add_startup_stage(
            "populate with dead cells",
            SystemStage::single(populate_dead.system()),
        )
        //helper systems
        .add_system(mouse_pos_update.system().before(Label::Input))
        .add_system(change_clicked_cell.system().label(Label::Input))
        .add_system(control_simulation.system().label(Label::Input))
        .add_system(
            change_materials
                .system()
                .label(Label::Draw)
                .after(Label::Calculating)
                .after(Label::Input),
        )
        .add_system(
            calculate_new_generation
                .system()
                .label(Label::Calculating)
                .before(Label::Draw)
                .with_run_criteria(FixedTimestep::step(0.4).chain(simulation_running.system()))
        )
        .add_system_set_to_stage(
            CoreStage::PostUpdate,
            SystemSet::new()
                .with_system(helpers::position_translation.system())
                .with_system(helpers::size_scaling.system()),
        )
        .run();
}

fn setup(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    commands
        .spawn()
        .insert_bundle(OrthographicCameraBundle::new_2d())
        .insert(MainCamera);
    commands.insert_resource(Materials {
        cell_alive_material: materials.add(helpers::get_bevy_color(184, 187, 38).into()),
        cell_dead_material: materials.add(helpers::get_bevy_color(40, 40, 40).into()),
    });
}

fn calculate_new_generation(
    mut cell_query: Query<(&mut Status, &Position)>,
) {
    //make lookup for current generation
    let mut lookup_table = HashMap::new();
    for (status, pos) in cell_query.iter_mut(){
        let status = *status;
        let pos = *pos;

        lookup_table.insert(pos, status);
    }

    //count neighbour based on loopup
    for (mut status, pos) in cell_query.iter_mut(){
        let mut alive_neighbor_count = 0;
        lookup_table.get(pos);

        for neighbor in helpers::get_neigbour_positions(pos).iter(){
            if let Some(nstatus) = lookup_table.get(neighbor){
                if *nstatus == Status::Alive {
                    alive_neighbor_count += 1;
                }
            }
        }

    //set status of next generation
        match *status { 
            Status::Alive => {
                if alive_neighbor_count < 2 {
                    *status = Status::Dying;
                }
                if alive_neighbor_count > 3 {
                    *status = Status::Dying;
                }
            }
            Status::Dead => {
                if alive_neighbor_count == 3 {
                    *status = Status::Born;
                }
            }
            _ => continue,
        }
    }
}

fn control_simulation(keys: Res<Input<KeyCode>>, mut sim: ResMut<Simulation>) {
    if keys.just_pressed(KeyCode::Space) {
        match *sim {
            Simulation::Running => { *sim = Simulation::Paused },
            Simulation::Paused => { *sim = Simulation::Running }
            Simulation::Step => { *sim = Simulation::Running } //TODO: something
        }
    }

    if keys.just_pressed(KeyCode::Space){

    }
}

fn simulation_running(In(should_run): In<ShouldRun>, mut sim_running: ResMut<Simulation>) -> ShouldRun {
    match *sim_running {
        Simulation::Running => should_run,
        Simulation::Paused => ShouldRun::No,
        Simulation::Step =>  { *sim_running = Simulation::Paused; ShouldRun::No }, //TODO: something
    }
}

fn change_materials(
    mut commands: Commands,
    materials: Res<Materials>,
    mut q: Query<(&mut Status, Entity)>,
) {
    for (mut status, entity) in q.iter_mut() {
        match *status {
            Status::Born => {
                commands
                    .entity(entity)
                    .remove::<ColorMaterial>()
                    .insert(materials.cell_alive_material.clone());
                *status = Status::Alive;
            }
            Status::Dying => {
                commands
                    .entity(entity)
                    .remove::<ColorMaterial>()
                    .insert(materials.cell_dead_material.clone());
                *status = Status::Dead;
            }
            _ => continue,
        }
    }
}

//Get mouse pos clicked in world space, and make clicked cell (if cell clicked) oposite Cell-value
fn change_clicked_cell(
    mouse_button: Res<Input<MouseButton>>,
    windows: Res<Windows>,
    mouse_pos: Res<MouseLoc>,
    pos_hash: Res<PosHash>,
    mut cell_query: Query<&mut Status>,
) {
    let wnd = windows.get_primary().expect("no primary window");
    if mouse_button.just_pressed(MouseButton::Left) {
        let pos_grid = helpers::world_pos_to_grid_pos(mouse_pos.0, wnd);

        let ent = pos_hash.0.get(&pos_grid);
        match ent {
            Some(entity) => {
                if let Ok(mut status) = cell_query.get_mut(*entity) {
                    *status = Status::Born;
                }
            }
            None => {}
        }
    }
}

fn mouse_pos_update(
    mut mouse_pos: ResMut<MouseLoc>,
    mut evr_cursor: EventReader<CursorMoved>,
    windows: Res<Windows>,
) {
    for ev in evr_cursor.iter() {
        let wnd = windows.get(ev.id).unwrap();
        let size = Vec2::new(wnd.width() as f32, wnd.height() as f32);
        mouse_pos.0 = ev.position - size / 2.0;
    }
}


fn populate_dead(mut commands: Commands, materials: Res<Materials>, mut pos_hash: ResMut<PosHash>) {
    for x in 0..helpers::ARENA_WIDTH {
        for y in 0..helpers::ARENA_HEIGHT {
            let cell = commands
                .spawn_bundle(SpriteBundle {
                    material: materials.cell_dead_material.clone(),
                    ..Default::default()
                })
                .insert(Position { x, y })
                .insert(SizeInGrid::square(0.9))
                .insert(Status::Dead)
                .id();

            pos_hash.0.insert(Position { x, y }, cell);
        }
    }
}
