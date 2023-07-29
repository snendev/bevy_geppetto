use bevy::{
    prelude::{
        shape, App, Assets, Camera2dBundle, Color, ColorMaterial, Commands, Component, FixedUpdate,
        Input, KeyCode, Mesh, Query, Res, ResMut, Startup, Transform, With,
    },
    sprite::MaterialMesh2dBundle,
};

use bevy_geppetto::Test;

#[derive(Component)]
struct MyPlayer;

fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn((
        MyPlayer,
        MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(10.).into()).into(),
            material: materials.add(ColorMaterial::from(Color::PURPLE)),
            ..Default::default()
        },
    ));
}

const SPEED: f32 = 5.;

fn move_player(
    mut player_query: Query<&mut Transform, With<MyPlayer>>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    for mut transform in player_query.iter_mut() {
        if keyboard_input.pressed(KeyCode::W) || keyboard_input.pressed(KeyCode::Up) {
            transform.translation.y += SPEED;
        }
        if keyboard_input.pressed(KeyCode::A) || keyboard_input.pressed(KeyCode::Left) {
            transform.translation.x -= SPEED;
        }
        if keyboard_input.pressed(KeyCode::S) || keyboard_input.pressed(KeyCode::Down) {
            transform.translation.y -= SPEED;
        }
        if keyboard_input.pressed(KeyCode::D) || keyboard_input.pressed(KeyCode::Right) {
            transform.translation.x += SPEED;
        }
    }
}

fn main() {
    Test {
        label: "Game sandbox".to_string(),
        setup: |app: &mut App| {
            app.add_systems(Startup, spawn_player)
                .add_systems(FixedUpdate, move_player);
        },
    }
    .run();
}
