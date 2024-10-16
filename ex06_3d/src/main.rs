use bevy::input::keyboard::KeyboardInput;
use bevy::prelude::*;
use bevy::render::mesh::VertexAttributeValues;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, (move_cube, rotate_on_timer))
        .run();
}

#[derive(Component)]
struct Cube;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // plane
    commands.spawn(PbrBundle {
        mesh: meshes.add(Plane3d::default().mesh().size(5.0, 5.0)),
        material: materials.add(Color::srgb(0.3, 0.5, 0.3)),
        ..default()
    });
    // cube
    // Assign vertex colors based on vertex positions
    let mut colorful_cube = Mesh::from(Cuboid::default());
    if let Some(VertexAttributeValues::Float32x3(positions)) =
        colorful_cube.attribute(Mesh::ATTRIBUTE_POSITION)
    {
        let colors: Vec<[f32; 4]> = positions
            .iter()
            .map(|[r, g, b]| [(1. - *r) / 2., (1. - *g) / 2., (1. - *b) / 2., 1.])
            .collect();
        colorful_cube.insert_attribute(Mesh::ATTRIBUTE_COLOR, colors);
    }
    commands.spawn(PbrBundle {
        mesh: meshes.add(colorful_cube),
        // This is the default color, but note that vertex colors are
        // multiplied by the base color, so you'll likely want this to be
        // white if using vertex colors.
        material: materials.add(Color::srgb(1., 1., 1.)),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..default()
    }).insert(Cube);

    // Light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 5.0, 4.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    // Camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

fn move_cube(
    mut player_query: Query<&mut Transform, With<Cube>>,
    mut char_input_events: EventReader<KeyboardInput>,
) {
    let mut offset = Vec3::ZERO;
    for event in char_input_events.read() {
        if event.state.is_pressed() {
            match event.key_code {
                KeyCode::KeyW => offset.z += 0.1,
                KeyCode::KeyS => offset.z -= 0.1,
                KeyCode::KeyA => offset.x -= 0.1,
                KeyCode::KeyD => offset.x += 0.1,
                KeyCode::KeyQ => offset.y += 0.1,
                KeyCode::KeyE => offset.y -= 0.1,
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

fn rotate_on_timer(time: Res<Time>, mut query: Query<&mut Transform, With<Cube>>) {
    for mut transform in query.iter_mut() {
        transform.rotate(Quat::from_rotation_y(time.delta_seconds()));
    }
}