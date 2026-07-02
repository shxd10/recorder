use bevy::{asset::RenderAssetUsages, prelude::*, render::render_resource::PrimitiveTopology};

mod kcl;
mod utils;

// example brought down to the essentials from here: https://bevy.org/examples/3d-rendering/generate-custom-mesh/

#[derive(Component)]
struct MainCamera;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, camera_movement)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let archive = brres::Archive::from_path("test/course_model.brres").unwrap();

    let vertex_data = archive
        .models
        .iter()
        .flat_map(|model| model.positions.iter())
        .flat_map(|buffer| buffer.data.iter().cloned())
        .collect();

    let normal_data = archive
        .models
        .iter()
        .flat_map(|model| model.normals.iter())
        .flat_map(|buffer| buffer.data.iter().cloned())
        .collect();

    // Create and save a handle to the mesh.
    let mesh_handle: Handle<Mesh> = meshes.add(create_mesh(vertex_data, normal_data));

    commands.spawn((
        Mesh3d(mesh_handle),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgba(1.0, 1.0, 1.0, 1.0),
            ..default()
        })),
    ));

    commands.spawn((Camera3d::default(), MainCamera));
    commands.spawn((
        PointLight::default(),
        Transform::from_xyz(1.0, 1.0, 1.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

fn create_mesh(vert: Vec<[f32; 3]>, nrm: Vec<[f32; 3]>) -> Mesh {
    Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(),
    )
    .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, vert)
    .with_computed_normals()
}

fn camera_movement(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Transform, With<MainCamera>>,
) {
    for mut transform in &mut query {
        let mut rot = Vec2::ZERO;

        if keys.pressed(KeyCode::ArrowLeft) {
            rot.x += 1.0;
        }
        if keys.pressed(KeyCode::ArrowRight) {
            rot.x -= 1.0;
        }
        if keys.pressed(KeyCode::ArrowUp) {
            rot.y += 1.0;
        }
        if keys.pressed(KeyCode::ArrowDown) {
            rot.y -= 1.0;
        }

        if rot != Vec2::ZERO {
            let sensitivity = 2.0;
            let yaw = rot.x * sensitivity * time.delta_secs();
            let pitch = rot.y * sensitivity * time.delta_secs();

            let yaw_quat = Quat::from_rotation_y(yaw);
            let pitch_quat = Quat::from_rotation_x(pitch);
            transform.rotation = yaw_quat * transform.rotation;
            transform.rotation = transform.rotation * pitch_quat;
        }

        let mut dir = Vec3::ZERO;
        let forward = transform.forward();
        let right = transform.right();
        if keys.pressed(KeyCode::KeyW) {
            dir += *forward;
        }
        if keys.pressed(KeyCode::KeyS) {
            dir -= *forward;
        }
        if keys.pressed(KeyCode::KeyD) {
            dir += *right;
        }
        if keys.pressed(KeyCode::KeyA) {
            dir -= *right;
        }
        if keys.pressed(KeyCode::Space) {
            dir += Vec3::Y;
        }
        if keys.pressed(KeyCode::ShiftLeft) {
            dir -= Vec3::Y;
        }
        if dir != Vec3::ZERO {
            // hardcoded 10000 for now
            transform.translation += dir.normalize() * 10000.0 * time.delta_secs();
        }
    }
}
