use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;

const BALL_SIZE: f32 = 5.;

#[derive(Component)]
struct Position(Vec2);

#[derive(Component)]
struct Ball;

#[derive(Bundle)]
struct BallBundle {
    ball: Ball,
    position: Position
}

impl BallBundle {
    fn new() -> Self {
        Self {
            ball: Ball,
            position: Position(Vec2::new(0., 10.)),
        }
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn_empty()
            .insert(Camera2dBundle::default());
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (spawn_ball, spawn_camera))
        .add_systems(Update, (project_positions))
        .run();
}

fn spawn_ball(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    println!("Spawning ball...");

    let shape = Mesh::from(Circle::new(BALL_SIZE));
    let color = ColorMaterial::from(Color::rgb(1., 0., 0.));

    let mesh_handle = meshes.add(shape);
    let material_handle = materials.add(color);

    commands.spawn((
        BallBundle::new(0., 0.),
        MaterialMesh2dBundle {
            mesh: mesh_handle.into(),
            material: material_handle,
            ..default()
        },
    ));
}

fn project_positions(
    mut positionables: Query<(&mut Transform, &Position)>
) {
    for (mut transform, position) in &mut positionables {
        transform.translation = position.0.extend(0.);
    }
}