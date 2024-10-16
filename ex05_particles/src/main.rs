use bevy::input::keyboard::KeyboardInput;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_hanabi::prelude::*;

#[derive(Component)]
struct Ferris;

#[derive(Event)]
struct ThrusterFired(Vec3);

#[derive(Resource)]
struct ParticleEffects {
    effect: Handle<EffectAsset>,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins(HanabiPlugin)
        .add_event::<ThrusterFired>()
        .add_systems(Startup, (setup, setup_particles))
        .add_systems(Update, move_crab)
        .add_systems(Update, fire_thrusters)
        .add_systems(Update, expire_particles)
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
    commands.spawn(SpriteBundle {
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
    mut player_query: Query<(&mut Velocity, &Transform), With<Ferris>>,
    mut char_input_events: EventReader<KeyboardInput>,
    mut thruster: EventWriter<ThrusterFired>,
) {
    let (mut player_velocity, player_transform) = player_query.get_single_mut().unwrap();

    let mut offset = Vect::ZERO;
    for event in char_input_events.read() {
        match event.key_code {
            KeyCode::KeyW => {
                offset.y += 500.0;
                thruster.send(ThrusterFired(player_transform.translation));
            },
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
    player_velocity.linvel = offset;
}

fn setup_particles(
    mut effects: ResMut<Assets<EffectAsset>>,
    mut commands: Commands,
) {
    let mut gradient = Gradient::new();
    gradient.add_key(0.0, Vec4::new(1., 1., 0., 1.));
    gradient.add_key(1.0, Vec4::splat(0.));

    // Create a new expression module
    let mut module = Module::default();

    // On spawn, randomly initialize the position of the particle
    // to be over the surface of a sphere of radius 2 units.
    let init_pos = SetPositionSphereModifier {
        center: module.lit(Vec3::ZERO),
        radius: module.lit(0.05),
        dimension: ShapeDimension::Surface,
    };

    // Also initialize a radial initial velocity to 6 units/sec
    // away from the (same) sphere center.
    let init_vel = SetVelocitySphereModifier {
        center: module.lit(Vec3::ZERO),
        speed: module.lit(6.),
    };

    // Initialize the total lifetime of the particle, that is
    // the time for which it's simulated and rendered. This modifier
    // is almost always required, otherwise the particles won't show.
    let lifetime = module.lit(3.0); // literal value "10.0"
    let init_lifetime = SetAttributeModifier::new(
        Attribute::LIFETIME, lifetime);

    // Every frame, add a gravity-like acceleration downward
    let accel = module.lit(Vec3::new(0., -3., 0.));
    let update_accel = AccelModifier::new(accel);

    // Create the effect asset
    let effect = EffectAsset::new(
        // Maximum number of particles alive at a time
        vec![32768],
        // Spawn at a rate of 5 particles per second
        Spawner::rate(5.0.into()),
        // Move the expression module into the asset
        module
    )
        .with_name("MyEffect")
        .init(init_pos)
        .init(init_vel)
        .init(init_lifetime)
        .update(update_accel)
        // Render the particles with a color gradient over their
        // lifetime. This maps the gradient key 0 to the particle spawn
        // time, and the gradient key 1 to the particle death (10s).
        .render(ColorOverLifetimeModifier { gradient });

    // Insert into the asset system
    let effect_handle = effects.add(effect);
    commands.insert_resource(ParticleEffects { effect: effect_handle });
}

#[derive(Component)]
struct ParticleBurst(f32);

fn fire_thrusters(
    mut reader: EventReader<ThrusterFired>,
    effects: Res<ParticleEffects>,
    mut commands: Commands,
) {
    for event in reader.read() {
        println!("Thruster fired at {:?}", event.0);
        commands
            .spawn(ParticleEffectBundle {
                effect: ParticleEffect::new(effects.effect.clone()),
                transform: Transform::from_translation(event.0),
                ..Default::default()
            }).insert(ParticleBurst(3.0));
    }
}

fn expire_particles(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut ParticleBurst)>,
) {
    for (entity, mut burst) in query.iter_mut() {
        burst.0 -= time.delta_seconds();
        if burst.0 <= 0.0 {
            commands.entity(entity).despawn();
        }
    }
}
