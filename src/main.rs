use bevy::mesh::Indices;
use bevy::{asset::RenderAssetUsages, prelude::*, render::render_resource::PrimitiveTopology};
use brres::Mesh as BrresMesh;
use brres::{MatrixPrimitive, VertexNormalBuffer, VertexPositionBuffer};

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

#[derive(Debug, Clone, Copy)]
pub enum GXAttr {
    PNMTXIDX = 0,
    TEX0MTXIDX = 1,
    TEX1MTXIDX = 2,
    TEX2MTXIDX = 3,
    TEX3MTXIDX = 4,
    TEX4MTXIDX = 5,
    TEX5MTXIDX = 6,
    TEX6MTXIDX = 7,
    TEX7MTXIDX = 8,
    POS = 9,
    NRM = 10,
    CLR0 = 11,
    CLR1 = 12,
    TEX0 = 13,
    TEX1 = 14,
    TEX2 = 15,
    TEX3 = 16,
    TEX4 = 17,
    TEX5 = 18,
    TEX6 = 19,
    TEX7 = 20,
}

impl GXAttr {
    pub fn has_attribute(vcd: i32, attr: &Self) -> bool {
        (vcd & (1 << *attr as i32)) != 0
    }
}

fn setup(
    mut commands: Commands,
    mut bevy_meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let archive = brres::Archive::from_path("test/course_model.brres").unwrap();

    let models = &archive.models;

    let mut global_mesh_idx = 0;
    for model in models.iter() {
        let name = &model.name;
        let info = &model.info;

        let bones = &model.bones;
        let mats = &model.materials;
        let meshes = &model.meshes;

        let pos_bufs = &model.positions;
        let nrm_bufs = &model.normals;
        let uv_bufs = &model.texcoords;
        let clr_bufs = &model.colors;

        let matrices = &model.matrices;

        for mesh in meshes.iter() {
            let mesh_idx = global_mesh_idx;
            global_mesh_idx += 1;

            let primitives = decode_matrix_primitive(mesh);
            let (vertices, indices) = build_mesh_topology(&primitives);

            let has_pos = GXAttr::has_attribute(mesh.vcd, &GXAttr::POS);
            let has_nrm = GXAttr::has_attribute(mesh.vcd, &GXAttr::NRM);
            let has_uv = GXAttr::has_attribute(mesh.vcd, &GXAttr::TEX0);

            let pos_buf = if has_pos {
                match pos_bufs.iter().find(|buf| buf.name == mesh.pos_buffer) {
                    Some(buf) => buf,
                    None => {
                        println!("Missing position buffer: {}", mesh.pos_buffer);
                        continue;
                    }
                }
            } else {
                println!("Mesh has no position attribute: {}", mesh.name);
                continue;
            };

            let nrm_buf = {
                if has_nrm {
                    nrm_bufs.iter().find(|buf| buf.name == mesh.nrm_buffer)
                } else {
                    None
                }
            };

            // first channel for now
            let uv_buf = if has_uv {
                mesh.uv_buffer
                    .first()
                    .and_then(|name| uv_bufs.iter().find(|buf| buf.name == *name))
            } else {
                None
            };

            let vcd = mesh.vcd;

            let mut positions = Vec::with_capacity(vertices.len());
            
            let mut normals = if has_nrm {
                Some(Vec::with_capacity(vertices.len()))
            } else {
                None
            };
            
            let mut uvs = if has_uv {
                Some(Vec::with_capacity(vertices.len()))
            } else {
                None
            };

            for vtx in &vertices {
                let pos_index = vtx.indices[GXAttr::POS as usize] as usize;
                positions.push(pos_buf.data[pos_index]);
            
                if let Some(normals) = &mut normals {
                    let nrm_index = vtx.indices[GXAttr::NRM as usize] as usize;
                    normals.push(nrm_buf.unwrap().data[nrm_index]);
                }
            
                if let Some(uvs) = &mut uvs {
                    let uv_index = vtx.indices[GXAttr::TEX0 as usize] as usize;
                    uvs.push(uv_buf.unwrap().data[uv_index]);
                }
            }

            println!(
                "Mesh {mesh_idx}: GX vertices: {}, indices: {}, positions: {}, normals: {}, uvs: {}",
                vertices.len(),
                indices.len(),
                positions.len(),
                if let Some(normals) = &normals { normals.len() } else { 0 },
                if let Some(uvs) = &uvs { uvs.len() } else { 0 },
            );

            let mesh_handle = bevy_meshes.add(create_mesh(positions, normals, uvs, indices));

            commands.spawn((
                Mesh3d(mesh_handle),
                MeshMaterial3d(materials.add(StandardMaterial {
                    base_color: Color::srgba(1.0, 1.0, 1.0, 1.0),
                    ..default()
                })),
            ));
        }
    }

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

fn build_mesh_topology(primitives: &[Primitive]) -> (Vec<IndexedVertex>, Vec<u32>) {
    let mut vertices = Vec::<IndexedVertex>::new();
    let mut indices = Vec::new();

    for prim in primitives {
        //println!("0x{:02x}", prim.prim_type.to_hex());
        let offset = vertices.len() as u32;
        vertices.extend(prim.vertices.clone());

        match prim.prim_type {
            PrimitiveType::Quads | PrimitiveType::Quads2 => {
                for i in (0..prim.vertices.len()).step_by(4) {
                    let v0 = offset + i as u32;
                    let v1 = offset + i as u32 + 1;
                    let v2 = offset + i as u32 + 2;
                    let v3 = offset + i as u32 + 3;

                    indices.push(v0);
                    indices.push(v1);
                    indices.push(v2);

                    indices.push(v0);
                    indices.push(v2);
                    indices.push(v3);
                }
            }
            PrimitiveType::Triangles => {
                for i in 0..prim.vertices.len() {
                    indices.push(offset + i as u32);
                }
            }
            PrimitiveType::TriangleStrip => {
                for i in 2..prim.vertices.len() {
                    let v0 = offset + i as u32 - 2;
                    let v1 = offset + i as u32 - ((!i as u32) & 1);
                    let v2 = offset + i as u32 - ((i as u32) & 1);

                    indices.push(v0);
                    indices.push(v1);
                    indices.push(v2);
                }
            }
            PrimitiveType::TriangleFan => {
                for i in 2..prim.vertices.len() {
                    let v0 = offset;
                    let v1 = offset + i as u32 - 1;
                    let v2 = offset + i as u32;

                    indices.push(v0);
                    indices.push(v1);
                    indices.push(v2);
                }
            }
            _ => {
                println!(
                    "Unsupported primitive (i don't render Lines, LineStrip, and Points): {:?}",
                    prim.prim_type.to_hex()
                );
            }
        }
    }

    (vertices, indices)
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
                    indices[i] = ((buf[cursor] as u16) | ((buf[cursor + 1] as u16) << 8));
                    cursor += 2;
                }
                vertices.push(IndexedVertex { indices });
            }

            primitives.push(Primitive {
                prim_type: PrimitiveType::from_u8(prim_type),
                vertices,
            });
        }
    }

    primitives
}

fn create_mesh(pos: Vec<[f32; 3]>, nrm: Option<Vec<[f32; 3]>>, uvs: Option<Vec<[f32; 2]>>, indices: Vec<u32>) -> Mesh {
    let mut mesh = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(),
    );

    mesh.insert_attribute(
        Mesh::ATTRIBUTE_POSITION,
        pos,
    );

    if let Some(normals) = nrm {
        mesh.insert_attribute(
            Mesh::ATTRIBUTE_NORMAL,
            normals,
        );
    }

    if let Some(uvs) = uvs {
        mesh.insert_attribute(
            Mesh::ATTRIBUTE_UV_0,
            uvs,
        );
    }

    mesh.insert_indices(Indices::U32(indices));

    mesh
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
