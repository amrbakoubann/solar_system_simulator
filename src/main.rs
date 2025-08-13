use bevy::prelude::*;
use bevy::input::mouse::MouseMotion;

// Components for our celestial bodies
#[derive(Component)]
struct CelestialBody;

#[derive(Component)]
struct Name(String);

#[derive(Component)]
struct Mass(f32);

#[derive(Component)]
struct Velocity(Vec3);

#[derive(Component)]
struct CameraController {
    pub sensitivity: f32,
    pub speed: f32,
}

// Much smaller gravitational constant for stability
const GRAVITATIONAL_CONSTANT: f32 = 10.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup_scene)
        .add_systems(Update, (gravity_system, movement_system, camera_controller).chain())
        .run();
}

/// Gravity system with much more conservative physics
fn gravity_system(
    mut query: Query<(&mut Velocity, &Transform, &Mass), With<CelestialBody>>,
    time: Res<Time>,
) {
    let mut combinations = query.iter_combinations_mut();
    let dt = time.delta_seconds().min(1.0/60.0); // Cap delta time for stability
    
    while let Some([(mut vel1, transform1, mass1), (mut vel2, transform2, mass2)]) = 
        combinations.fetch_next() {
        
        let direction = transform2.translation - transform1.translation;
        let distance = direction.length();
        
        // Skip if too close to avoid singularities
        if distance < 2.0 {
            continue;
        }
        
        // Calculate gravitational force
        let force_magnitude = GRAVITATIONAL_CONSTANT * mass1.0 * mass2.0 / (distance * distance);
        let force_direction = direction / distance; // Normalize
        
        // Apply much smaller force changes
        let force_multiplier = 0.01; // Make forces much weaker
        vel1.0 += force_direction * force_magnitude / mass1.0 * dt * force_multiplier;
        vel2.0 -= force_direction * force_magnitude / mass2.0 * dt * force_multiplier;
    }
}

/// Movement system
fn movement_system(
    mut query: Query<(&mut Transform, &Velocity), With<CelestialBody>>,
    time: Res<Time>,
) {
    let dt = time.delta_seconds().min(1.0/60.0); // Cap delta time
    for (mut transform, velocity) in query.iter_mut() {
        transform.translation += velocity.0 * dt;
    }
}

/// Camera controller system
fn camera_controller(
    time: Res<Time>,
    mut mouse_events: EventReader<MouseMotion>,
    mouse_button: Res<ButtonInput<MouseButton>>,
    key_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Transform, &CameraController), With<Camera3d>>,
) {
    for (mut transform, controller) in query.iter_mut() {
        let mut velocity = Vec3::ZERO;
        let local_z = transform.local_z();
        let forward = -Vec3::new(local_z.x, 0., local_z.z);
        let right = Vec3::new(local_z.z, 0., -local_z.x);

        if key_input.pressed(KeyCode::KeyW) {
            velocity += forward;
        }
        if key_input.pressed(KeyCode::KeyS) {
            velocity -= forward;
        }
        if key_input.pressed(KeyCode::KeyA) {
            velocity -= right;
        }
        if key_input.pressed(KeyCode::KeyD) {
            velocity += right;
        }
        if key_input.pressed(KeyCode::Space) {
            velocity += Vec3::Y;
        }
        if key_input.pressed(KeyCode::ShiftLeft) {
            velocity -= Vec3::Y;
        }

        velocity = velocity.normalize_or_zero();
        transform.translation += velocity * time.delta_seconds() * controller.speed;

        if mouse_button.pressed(MouseButton::Right) {
            for mouse_event in mouse_events.read() {
                let (mut yaw, mut pitch, _) = transform.rotation.to_euler(EulerRot::YXZ);
                yaw -= mouse_event.delta.x * controller.sensitivity * time.delta_seconds();
                pitch -= mouse_event.delta.y * controller.sensitivity * time.delta_seconds();
                pitch = pitch.clamp(-1.54, 1.54);

                transform.rotation = Quat::from_axis_angle(Vec3::Y, yaw) * Quat::from_axis_angle(Vec3::X, pitch);
            }
        }
    }
}

/// Setup scene with very conservative orbital velocities
fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Camera positioned to see the whole system
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(-50.0, 30.0, 50.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        CameraController {
            sensitivity: 2.0,
            speed: 25.0,
        },
    ));

    // Sun - stationary and massive
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Sphere::new(3.0)),
            material: materials.add(StandardMaterial {
                base_color: Color::YELLOW,
                emissive: Color::rgb(1.0, 1.0, 0.0) * 2.0,
                ..default()
            }),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
        CelestialBody,
        Name("Sun".to_string()),
        Mass(1000.0),
        Velocity(Vec3::ZERO),
    ));

    // Inner planet - very slow orbit
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Sphere::new(0.8)),
            material: materials.add(StandardMaterial {
                base_color: Color::rgb(0.8, 0.4, 0.2),
                ..default()
            }),
            transform: Transform::from_xyz(12.0, 0.0, 0.0),
            ..default()
        },
        CelestialBody,
        Name("Inner Planet".to_string()),
        Mass(5.0),
        Velocity(Vec3::new(0.0, 0.0, 0.8)), // Very slow
    ));

    // Middle planet
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Sphere::new(1.0)),
            material: materials.add(StandardMaterial {
                base_color: Color::srgb(0.2, 0.4, 0.8),
                ..default()
            }),
            transform: Transform::from_xyz(20.0, 0.0, 0.0),
            ..default()
        },
        CelestialBody,
        Name("Middle Planet".to_string()),
        Mass(8.0),
        Velocity(Vec3::new(0.0, 0.0, 0.6)), // Even slower
    ));

    // Outer planet
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Sphere::new(0.9)),
            material: materials.add(StandardMaterial {
                base_color: Color::srgb(0.8, 0.3, 0.1),
                ..default()
            }),
            transform: Transform::from_xyz(30.0, 0.0, 0.0),
            ..default()
        },
        CelestialBody,
        Name("Outer Planet".to_string()),
        Mass(6.0),
        Velocity(Vec3::new(0.0, 0.0, 0.4)), // Slowest
    ));

    // Strong point light at the sun
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 10000.0,
            shadows_enabled: true,
            range: 200.0,
            ..default()
        },
        transform: Transform::from_xyz(0.0, 5.0, 0.0),
        ..default()
    });

    // Ambient light for visibility
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 0.4,
    });
}