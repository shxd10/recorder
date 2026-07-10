use bevy::mesh::Indices;
use bevy::{asset::RenderAssetUsages, prelude::*, render::render_resource::PrimitiveTopology};
use brres::{MatrixPrimitive, VertexNormalBuffer, VertexPositionBuffer, };
use brres::Mesh as BrresMesh;

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


// https://github.com/snailspeed3/RiiStudio/blob/master/source/librii/gx/Vertex.hpp#L155
pub enum PrimitiveType {
    Quads,
    Quads2,
    Triangles,
    TriangleStrip,
    TriangleFan,
    Lines,
    LineStrip,
    Points,
}

impl PrimitiveType {
    fn from_u8(value: u8) -> Self {
        match value {
            0 => PrimitiveType::Quads,
            1 => PrimitiveType::Quads2,
            2 => PrimitiveType::Triangles,
            3 => PrimitiveType::TriangleStrip,
            4 => PrimitiveType::TriangleFan,
            5 => PrimitiveType::Lines,
            6 => PrimitiveType::LineStrip,
            7 => PrimitiveType::Points,
            _ => panic!("Unknown primitive type: {}", value),
        }
    }

    fn to_hex(&self) -> u8 {
        match self {
            PrimitiveType::Quads => 0x80,
            PrimitiveType::Quads2 => 0x88,
            PrimitiveType::Triangles => 0x90,
            PrimitiveType::TriangleStrip => 0x98,
            PrimitiveType::TriangleFan => 0xA0,
            PrimitiveType::Lines => 0xA8,
            PrimitiveType::LineStrip => 0xB0,
            PrimitiveType::Points => 0xB8,
        }
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let archive = brres::Archive::from_path("test/course_model.brres").unwrap();

    let models = &archive.models;

    let pos_bufs: Vec<VertexPositionBuffer> = models
        .iter()
        .flat_map(|model| model.positions.clone())
        .collect();

    let nrm_bufs: Vec<VertexNormalBuffer> = models
        .iter()
        .flat_map(|model| model.normals.clone())
        .collect();

    let brres_meshes: Vec<BrresMesh> = models
        .iter()
        .flat_map(|model| model.meshes.clone())
        .collect();

    // first mesh for now
    let mesh = &brres_meshes[0];

    let primitives = decode_matrix_primitive(&mesh);

    let mut positions = Vec::new();
    let mut normals = Vec::new();
    let mut indices = Vec::new();

    for prim in primitives {
        println!("{:02x}", prim.prim_type.to_hex());
        // handle primitive types
    }

    let mesh_handle = meshes.add(create_mesh(positions, normals, indices));

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

#[derive(Clone)]
pub struct IndexedVertex {
    pub indices: [u16; 26],
}

pub struct Primitive {
    pub prim_type: PrimitiveType,
    pub vertices: Vec<IndexedVertex>,
}

// function translated and modified to rust from the python example (while returning what i need)
fn decode_matrix_primitive(mesh: &BrresMesh) -> Vec<Primitive> {
    let mut primitives = Vec::new();
    
    for mprim in &mesh.mprims {
        let buf = &mprim.vertex_data_buffer;
        let mut cursor = 0;
        
        while cursor + 4 <= buf.len() {
            let prim_type = buf[cursor];
            let prim_vtx_count = ((buf[cursor + 1] as u32) << 16)
                | ((buf[cursor + 2] as u32) << 8)
                | (buf[cursor + 3] as u32);
            cursor += 4;
            
            if prim_vtx_count == 0 || cursor + (prim_vtx_count as usize * 52) > buf.len() {
                break;
            }
            
            let mut vertices = Vec::new();
            
            for _ in 0..prim_vtx_count {
                let mut indices = [0u16; 26];
                for i in 0..26 {
                    indices[i] = (buf[cursor] as u16) | ((buf[cursor + 1] as u16) << 8);
                    cursor += 2;
                }
                vertices.push(IndexedVertex { indices });
            }
            
            primitives.push(Primitive { prim_type: PrimitiveType::from_u8(prim_type), vertices });
        }
    }
    
    primitives
}

fn create_mesh(pos: Vec<[f32; 3]>, nrm: Vec<[f32; 3]>, idx: Vec<u32>) -> Mesh {
    Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::default())
        .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, pos)
        .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, nrm)
        .with_inserted_indices(Indices::U32(idx))
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