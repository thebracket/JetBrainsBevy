use bevy::input::keyboard::KeyboardInput;
use bevy::prelude::*;

#[derive(Component)]
struct Ferris;

#[derive(Component)]
struct Star(f32);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, (move_crab, move_stars))
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Load the star
    let rust_star = asset_server.load("rust_star.png");

    // Always good to have a camera (you can't see anything otherwise)
    commands.spawn(Camera2dBundle::default());

    // Spawn Ferris
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

    // Spawn some stars
    for n in 0..36 {
        let angle = n as f32 * std::f32::consts::PI / 18.0;
        let x = angle.cos() * 300.0;
        let y = angle.sin() * 300.0;
        commands.spawn(SpriteBundle {
            // Choose the texture
            texture: rust_star.clone(),
            // Place Ferris on the map
            transform: Transform {
                translation: Vec3::new(x, y, -1.0),
                ..Default::default()
            },
            ..default()
        }).insert(Star(n as f32)); // And tag him as Ferris
    }
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

fn move_stars(
    player_query: Query<&Transform, With<Ferris>>,
    mut star_query: Query<(&mut Star, &mut Transform), Without<Ferris>>
) {
    let player_pos = player_query
        .get_single()
        .unwrap()
        .translation;

    for (mut star, mut transform) in star_query.iter_mut() {
        // Figure out where we want to be
        let angle = star.0 * std::f32::consts::PI / 18.0;
        let x = angle.cos() * 300.0;
        let y = angle.sin() * 300.0;
        let desired_pos = Vec3::new(x, y, -1.0) + player_pos;
        let offset = desired_pos - transform.translation;
        let direction = offset.normalize();
        if direction.is_finite() {
            transform.translation = desired_pos;
        }

        star.0 += 0.1;
        if star.0 >= 36.0 {
            star.0 = 0.0;
        }
    }
}