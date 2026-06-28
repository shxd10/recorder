use crate::kcl::ObjData;
use bevy::{
    asset::RenderAssetUsages, mesh::Indices, prelude::*, render::render_resource::PrimitiveTopology,
};
use std::fs;

mod kcl;
mod utils;

// example brought down to the essentials from here: https://bevy.org/examples/3d-rendering/generate-custom-mesh/
// right now i want to check out how to actually render vertices in space for a simple KCL viewer

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
    let raw = fs::read("test/course.kcl").expect("Failed to read KCL file");
    let data = kcl::kcl_to_obj(&raw).expect("Failed to parse KCL file");
    let object = kcl::parse_obj(&data.obj).expect("Failed to parse OBJ data");

    // Create and save a handle to the mesh.
    let mesh_handle: Handle<Mesh> = meshes.add(create_kcl_mesh(&object));

    commands.spawn((
        Mesh3d(mesh_handle),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgba(1.0, 1.0, 1.0, 1.0),
            ..default()
        })),
    ));

    commands.spawn((Camera3d::default(), MainCamera));
    commands.spawn((PointLight::default(), Transform::from_xyz(1.0, 1.0, 1.0).looking_at(Vec3::ZERO, Vec3::Y)));
}

fn create_kcl_mesh(obj: &ObjData) -> Mesh {
    Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(),
    )
    .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, obj.vertices.clone())
    .with_inserted_indices(Indices::U32(
        obj.faces
            .iter()
            .flat_map(|face| face.iter().map(|&index| index as u32))
            .collect(),
    ))
    .with_computed_normals()
}

fn camera_movement(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&MainCamera, &mut Transform)>,
) {
    for (cam, mut transform) in &mut query {
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
            // 10000 as a temporary hardcoded speed
            transform.translation += dir.normalize() * 10000.0 * time.delta_secs();
        }
    }
}
