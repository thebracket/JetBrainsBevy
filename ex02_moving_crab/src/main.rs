use bevy::input::keyboard::KeyboardInput;
use bevy::prelude::*;

#[derive(Component)]
struct Ferris;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, move_crab)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn(SpriteBundle {
        // Choose the texture
        texture: asset_server.load("ferris.png"),
        // Place Ferris on the map
        transform: Transform {
            translation: Vec3::new(0.0, 0.0, 0.0),
            ..Default::default()
        },
        ..default()
    }).insert(Ferris); // And tag him as Ferris
}

fn move_crab(
    mut player_query: Query<&mut Transform, With<Ferris>>,
    mut char_input_events: EventReader<KeyboardInput>
) {
    let mut offset = Vec3::ZERO;
    for event in char_input_events.read() {
        if event.state.is_pressed() {
            match event.key_code {
                KeyCode::KeyW => offset.y += 10.0,
                KeyCode::KeyS => offset.y -= 10.0,
                KeyCode::KeyA => offset.x -= 10.0,
                KeyCode::KeyD => offset.x += 10.0,
                _ => {}
            }
        }
    }
    // Don't bother running the rest of the function if there's no offset
    if offset == Vec3::ZERO {
        return;
    }

    // Move the player
    if let Ok(mut player) = player_query.get_single_mut() {
        player.translation += offset;
    }
}