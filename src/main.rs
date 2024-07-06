use bevy::{prelude::*, render::camera::ScalingMode};

#[derive(Component)]
pub struct Player{
    pub speed: f32,
}

#[derive(Component)]
pub struct Pig{
    pub lifetime: Timer,
}

#[derive(Component)]
pub struct Money(pub f32);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins
            .set(ImagePlugin::default_nearest())
            .set(WindowPlugin{
                primary_window: Some(Window{
                    title:"titulo".into(),
                    resolution:(640.0, 480.0).into(),
                    resizable:true,
                    ..default()
                }),
                ..default()
            })
            .build(),
        )
        .add_systems(Startup, setup)
        .add_systems(Update,character_movement)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>){
    let  mut camera =  Camera2dBundle::default();

    camera.projection.scaling_mode = ScalingMode::AutoMin{
        min_width: 256.0,
        min_height: 144.0,
    };

    commands.spawn(camera);

    let texture = asset_server.load("character.png");

    commands.spawn((
        SpriteBundle {
            texture,
            ..default()
        },
        Player{speed:100.0},
    ));
}

fn character_movement(
    mut characters: Query<(&mut Transform, &Sprite)>,
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
){
    for(mut transform, _) in &mut characters{
        if input.pressed(KeyCode::W){
            transform.translation.y += 100.0 * time.delta_seconds();
        }
        if input.pressed(KeyCode::S){
            transform.translation.y -= 100.0 * time.delta_seconds();
        }
        if input.pressed(KeyCode::A){
            transform.translation.x -= 100.0 * time.delta_seconds();
        }
        if input.pressed(KeyCode::D){
            transform.translation.x += 100.0 * time.delta_seconds();
        }
    }
}

