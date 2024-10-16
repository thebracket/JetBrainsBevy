use bevy::input::keyboard::KeyboardInput;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Component)]
struct Ferris;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_systems(Startup, setup)
        .add_systems(Update, move_crab)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Spawn a camera
    commands.spawn(Camera2dBundle::default());

    /* Create the ground. */
    commands
        .spawn(Collider::cuboid(250.0, 50.0))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, -300.0, 0.0)));

    commands
        .spawn(Collider::cuboid(1000.0, 50.0))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, -350.0, 0.0)));

    commands
        .spawn(Collider::cuboid(1000.0, 50.0))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, 350.0, 0.0)));

    commands
        .spawn(Collider::cuboid(50.0, 1000.0))
        .insert(TransformBundle::from(Transform::from_xyz(-500.0, 0.0, 0.0)));

    commands
        .spawn(Collider::cuboid(50.0, 1000.0))
        .insert(TransformBundle::from(Transform::from_xyz(500.0, 0.0, 0.0)));

    // Add a platform
    commands
        .spawn(Collider::cuboid(100.0, 50.0))
        .insert(TransformBundle::from(Transform::from_xyz(150.0, 10.0, 0.0)));

    // Create Ferris
    commands
        .spawn(SpriteBundle {
            texture: asset_server.load("ferris.png"),
            ..default()
        })
        .insert(RigidBody::Dynamic)
        .insert(Collider::round_cuboid(25.0, 25.0, 5.0))
        .insert(Restitution::coefficient(0.7))
        .insert(Velocity::zero())
        .insert(Ferris);
}

fn move_crab(
    mut player_query: Query<&mut Velocity, With<Ferris>>,
    mut char_input_events: EventReader<KeyboardInput>,
) {
    let mut offset = Vect::ZERO;
    for event in char_input_events.read() {
        match event.key_code {
            KeyCode::KeyW => offset.y += 500.0,
            KeyCode::KeyA => offset.x -= 100.0,
            KeyCode::KeyD => offset.x += 100.0,
            _ => {}
        }
    }
    // Don't bother running the rest of the function if there's no offset
    if offset == Vect::ZERO {
        return;
    }

    // Move the player
    if let Ok(mut player) = player_query.get_single_mut() {
        player.linvel = offset;
    }
}
