use crate::utils::binary::*;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
enum BaseType {
    Road,
    SlipperyRoad1,
    WeakOffroad,
    Offroad,
    HeavyOffroad,
    SlipperyRoad2,
    BoostPanel,
    BoostRamp,
    JumpPad,
    ItemRoad,
    SolidFall,
    MovingWater,
    Wall,
    InvisibleWall,
    ItemWall,
    Wall2,
    FallBoundary,
    CannonTrigger,
    ForceRecalculation,
    HalfPipeRamp,
    PlayerOnlyWall,
    MovingRoad,
    StickyRoad,
    Road2,
    SoundTrigger,
    WeakWall,
    EffectTrigger,
    ItemStateModifier,
    HalfPipeInvisibleWall,
    RotatingRoad,
    SpecialWall,
    InvisibleWall2,
}

#[derive(Clone, Copy, Hash, Eq, PartialEq)]
struct Flag {
    base_type: BaseType,
}

struct Prism {
    height: f32,
    pos_i: u16,
    fnrm_i: u16,
    enrm1_i: u16,
    enrm2_i: u16,
    enrm3_i: u16,
    flag: Flag,
}

struct Sections {
    position_vectors: Vec<[f32; 3]>,
    normals: Vec<[f32; 3]>,
    prisms: Vec<Prism>,
}

struct Header {
    pos_data_offset: u32,
    nrm_data_offset: u32,
    prism_data_offset: u32,
    block_data_offset: u32,
}

struct ParsedKcl {
    sections: Sections,
}

pub struct ObjData {
    pub vertices: Vec<[f32; 3]>,
    pub faces: Vec<[usize; 3]>,
    pub groups: HashMap<String, Vec<usize>>,
}

pub struct ObjMtlOutput {
    pub obj: String,
    pub mtl: String,
}

impl Header {
    fn parse(data: &[u8]) -> Result<Self, String> {
        let pos_data_offset = read_u32(data, 0x00)?;
        let nrm_data_offset = read_u32(data, 0x04)?;
        let prism_data_offset = read_u32(data, 0x08)?;
        let block_data_offset = read_u32(data, 0x0C)?;

        Ok(Header {
            pos_data_offset,
            nrm_data_offset,
            prism_data_offset,
            block_data_offset,
        })
    }
}

impl BaseType {
    fn from_u16(value: u16) -> Result<Self, String> {
        match value & 0x1F {
            0x00 => Ok(Self::Road),
            0x01 => Ok(Self::SlipperyRoad1),
            0x02 => Ok(Self::WeakOffroad),
            0x03 => Ok(Self::Offroad),
            0x04 => Ok(Self::HeavyOffroad),
            0x05 => Ok(Self::SlipperyRoad2),
            0x06 => Ok(Self::BoostPanel),
            0x07 => Ok(Self::BoostRamp),
            0x08 => Ok(Self::JumpPad),
            0x09 => Ok(Self::ItemRoad),
            0x0A => Ok(Self::SolidFall),
            0x0B => Ok(Self::MovingWater),
            0x0C => Ok(Self::Wall),
            0x0D => Ok(Self::InvisibleWall),
            0x0E => Ok(Self::ItemWall),
            0x0F => Ok(Self::Wall2),
            0x10 => Ok(Self::FallBoundary),
            0x11 => Ok(Self::CannonTrigger),
            0x12 => Ok(Self::ForceRecalculation),
            0x13 => Ok(Self::HalfPipeRamp),
            0x14 => Ok(Self::PlayerOnlyWall),
            0x15 => Ok(Self::MovingRoad),
            0x16 => Ok(Self::StickyRoad),
            0x17 => Ok(Self::Road2),
            0x18 => Ok(Self::SoundTrigger),
            0x19 => Ok(Self::WeakWall),
            0x1A => Ok(Self::EffectTrigger),
            0x1B => Ok(Self::ItemStateModifier),
            0x1C => Ok(Self::HalfPipeInvisibleWall),
            0x1D => Ok(Self::RotatingRoad),
            0x1E => Ok(Self::SpecialWall),
            0x1F => Ok(Self::InvisibleWall2),
            _ => Err(format!("Unknown KCL flag: {}", value)),
        }
    }

    fn color(&self) -> [u8; 4] {
        let (r, g, b, a) = match self {
            Self::Road => (255, 255, 255, 255),
            Self::SlipperyRoad1 => (255, 230, 204, 255),
            Self::WeakOffroad => (0, 204, 0, 255),
            Self::Offroad => (0, 153, 0, 255),
            Self::HeavyOffroad => (0, 102, 0, 255),
            Self::SlipperyRoad2 => (204, 230, 255, 255),
            Self::BoostPanel => (255, 128, 0, 255),
            Self::BoostRamp => (255, 153, 0, 255),
            Self::JumpPad => (255, 204, 0, 255),
            Self::ItemRoad => (230, 230, 255, 255),
            Self::SolidFall => (179, 26, 26, 255),
            Self::MovingWater => (0, 128, 255, 255),
            Self::Wall => (153, 153, 153, 255),
            Self::InvisibleWall => (0, 0, 153, 200),
            Self::ItemWall => (153, 153, 179, 255),
            Self::Wall2 => (153, 153, 153, 255),
            Self::FallBoundary => (204, 0, 0, 255),
            Self::CannonTrigger => (255, 0, 128, 255),
            Self::ForceRecalculation => (128, 0, 255, 50),
            Self::HalfPipeRamp => (0, 77, 255, 255),
            Self::PlayerOnlyWall => (204, 102, 0, 255),
            Self::MovingRoad => (230, 230, 255, 255),
            Self::StickyRoad => (230, 179, 255, 255),
            Self::Road2 => (255, 255, 255, 255),
            Self::SoundTrigger => (255, 0, 255, 50),
            Self::WeakWall => (102, 153, 102, 255),
            Self::EffectTrigger => (204, 0, 255, 50),
            Self::ItemStateModifier => (255, 0, 255, 50),
            Self::HalfPipeInvisibleWall => (0, 153, 0, 200),
            Self::RotatingRoad => (230, 230, 255, 255),
            Self::SpecialWall => (204, 179, 204, 255),
            Self::InvisibleWall2 => (0, 0, 153, 200),
        };
        [r, g, b, a]
    }

    fn name(&self) -> &'static str {
        match self {
            Self::Road => "road",
            Self::SlipperyRoad1 => "slippery_road1",
            Self::WeakOffroad => "weak_offroad",
            Self::Offroad => "offroad",
            Self::HeavyOffroad => "heavy_offroad",
            Self::SlipperyRoad2 => "slippery_road2",
            Self::BoostPanel => "boost_panel",
            Self::BoostRamp => "boost_ramp",
            Self::JumpPad => "jump_pad",
            Self::ItemRoad => "item_road",
            Self::SolidFall => "solid_fall",
            Self::MovingWater => "moving_water",
            Self::Wall => "wall",
            Self::InvisibleWall => "invisible_wall",
            Self::ItemWall => "item_wall",
            Self::Wall2 => "wall2",
            Self::FallBoundary => "fall_boundary",
            Self::CannonTrigger => "cannon_trigger",
            Self::ForceRecalculation => "force_recalc",
            Self::HalfPipeRamp => "halfpipe_ramp",
            Self::PlayerOnlyWall => "player_wall",
            Self::MovingRoad => "moving_road",
            Self::StickyRoad => "sticky_road",
            Self::Road2 => "road2",
            Self::SoundTrigger => "sound_trigger",
            Self::WeakWall => "weak_wall",
            Self::EffectTrigger => "effect_trigger",
            Self::ItemStateModifier => "item_state",
            Self::HalfPipeInvisibleWall => "halfpipe_invis",
            Self::RotatingRoad => "rotating_road",
            Self::SpecialWall => "special_wall",
            Self::InvisibleWall2 => "invisible_wall2",
        }
    }

    fn discriminant(&self) -> u8 {
        match self {
            Self::Road => 0x00,
            Self::SlipperyRoad1 => 0x01,
            Self::WeakOffroad => 0x02,
            Self::Offroad => 0x03,
            Self::HeavyOffroad => 0x04,
            Self::SlipperyRoad2 => 0x05,
            Self::BoostPanel => 0x06,
            Self::BoostRamp => 0x07,
            Self::JumpPad => 0x08,
            Self::ItemRoad => 0x09,
            Self::SolidFall => 0x0A,
            Self::MovingWater => 0x0B,
            Self::Wall => 0x0C,
            Self::InvisibleWall => 0x0D,
            Self::ItemWall => 0x0E,
            Self::Wall2 => 0x0F,
            Self::FallBoundary => 0x10,
            Self::CannonTrigger => 0x11,
            Self::ForceRecalculation => 0x12,
            Self::HalfPipeRamp => 0x13,
            Self::PlayerOnlyWall => 0x14,
            Self::MovingRoad => 0x15,
            Self::StickyRoad => 0x16,
            Self::Road2 => 0x17,
            Self::SoundTrigger => 0x18,
            Self::WeakWall => 0x19,
            Self::EffectTrigger => 0x1A,
            Self::ItemStateModifier => 0x1B,
            Self::HalfPipeInvisibleWall => 0x1C,
            Self::RotatingRoad => 0x1D,
            Self::SpecialWall => 0x1E,
            Self::InvisibleWall2 => 0x1F,
        }
    }
}

impl Flag {
    fn from_u16(value: u16) -> Result<Self, String> {
        let base_type = BaseType::from_u16(value)?;
        Ok(Flag { base_type })
    }
}

impl Prism {
    fn parse(data: &[u8], offset: usize) -> Result<Self, String> {
        let height = read_f32(data, offset)?;
        let pos_i = read_u16(data, offset + 4)?;
        let fnrm_i = read_u16(data, offset + 6)?;
        let enrm1_i = read_u16(data, offset + 8)?;
        let enrm2_i = read_u16(data, offset + 10)?;
        let enrm3_i = read_u16(data, offset + 12)?;
        let flag = Flag::from_u16(read_u16(data, offset + 14)?)?;

        Ok(Prism {
            height,
            pos_i,
            fnrm_i,
            enrm1_i,
            enrm2_i,
            enrm3_i,
            flag,
        })
    }
}

impl Sections {
    fn parse(data: &[u8], header: &Header) -> Result<Self, String> {
        let real_prism_offset = header.prism_data_offset as usize + 0x10;
        let max_prisms = (header.block_data_offset as usize - real_prism_offset) / 0x10;
        let mut prisms = Vec::with_capacity(max_prisms);
        for i in 0..max_prisms {
            prisms.push(Prism::parse(data, real_prism_offset + i * 0x10)?);
        }

        let max_pos = prisms.iter().map(|p| p.pos_i).max().unwrap_or(0) as usize + 1;
        let max_nrm = prisms
            .iter()
            .map(|p| p.fnrm_i.max(p.enrm1_i).max(p.enrm2_i).max(p.enrm3_i))
            .max()
            .unwrap_or(0) as usize
            + 1;

        let position_vectors = read_vec_f32(data, header.pos_data_offset as usize, max_pos * 3)?
            .chunks(3)
            .map(|c| [c[0], c[1], c[2]])
            .collect();

        let normals = read_vec_f32(data, header.nrm_data_offset as usize, max_nrm * 3)?
            .chunks(3)
            .map(|c| [c[0], c[1], c[2]])
            .collect();

        Ok(Sections {
            position_vectors,
            normals,
            prisms,
        })
    }
}

fn prism_to_triangle(prism: &Prism, positions: &[[f32; 3]], normals: &[[f32; 3]]) -> [[f32; 3]; 3] {
    let pos = positions[prism.pos_i as usize];
    let fnrm = normals[prism.fnrm_i as usize];
    let enrm1 = normals[prism.enrm1_i as usize];
    let enrm2 = normals[prism.enrm2_i as usize];
    let enrm3 = normals[prism.enrm3_i as usize];

    let cross_a = cross(enrm1, fnrm);
    let cross_b = cross(enrm2, fnrm);
    let v1 = pos;
    let v2 = add(v1, scale(cross_b, prism.height / dot(cross_b, enrm3)));
    let v3 = add(v1, scale(cross_a, prism.height / dot(cross_a, enrm3)));

    [v1, v2, v3]
}

pub fn kcl_to_obj(data: &[u8]) -> Result<ObjMtlOutput, String> {
    let header = Header::parse(data)?;
    let sections = Sections::parse(data, &header)?;
    let parsed = ParsedKcl { sections };

    let mut obj = String::new();
    let mut mtl = String::new();

    let pos_buf = &parsed.sections.position_vectors;
    let nrm_buf = &parsed.sections.normals;

    let mut vertex_offset = 1usize;

    let mut groups: HashMap<BaseType, Vec<usize>> = HashMap::new();
    for (i, prism) in parsed.sections.prisms.iter().enumerate() {
        groups.entry(prism.flag.base_type).or_default().push(i);
    }

    let mut sorted: Vec<(BaseType, Vec<usize>)> = groups.into_iter().collect();
    sorted.sort_by_key(|(base_type, _)| base_type.discriminant());

    for (base_type, prism_indices) in &sorted {
        let color = base_type.color();
        let name = base_type.name();

        mtl.push_str(&format!("newmtl {}\n", name));
        mtl.push_str(&format!(
            "Ka {:.4} {:.4} {:.4}\n",
            color[0] as f32 / 255.0,
            color[1] as f32 / 255.0,
            color[2] as f32 / 255.0,
        ));
        mtl.push_str(&format!(
            "Kd {:.4} {:.4} {:.4}\n",
            color[0] as f32 / 255.0,
            color[1] as f32 / 255.0,
            color[2] as f32 / 255.0,
        ));
        mtl.push_str(&format!(
            "Ks 0.0000 0.0000 0.0000\n"
        ));
        mtl.push_str(&format!("Ns 32.0000\n"));
        mtl.push_str(&format!("d {:.4}\n\n", color[3] as f32 / 255.0));

        obj.push_str(&format!("g {}\n", name));
        obj.push_str(&format!("usemtl {}\n", name));

        for &i in prism_indices {
            let [v1, v2, v3] = prism_to_triangle(&parsed.sections.prisms[i], pos_buf, nrm_buf);
            obj.push_str(&format!("v {} {} {}\n", v1[0], v1[1], v1[2]));
            obj.push_str(&format!("v {} {} {}\n", v2[0], v2[1], v2[2]));
            obj.push_str(&format!("v {} {} {}\n", v3[0], v3[1], v3[2]));
            let base = vertex_offset;
            vertex_offset += 3;
            obj.push_str(&format!("f {} {} {}\n", base, base + 1, base + 2));
        }

        obj.push('\n');
    }

    Ok(ObjMtlOutput { obj, mtl })
}

pub fn parse_obj(data: &str) -> Result<ObjData, String> {
    let mut vertices: Vec<[f32; 3]> = Vec::new();
    let mut faces: Vec<[usize; 3]> = Vec::new();
    let mut groups: HashMap<String, Vec<usize>> = HashMap::new();
    let mut current_group = "default".to_string();

    for line in data.lines() {
        let line = line.trim();

        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.is_empty() {
            continue;
        }

        match parts[0] {
            "v" => {
                if parts.len() < 4 {
                    return Err("Invalid vertex format".to_string());
                }
                let x: f32 = parts[1].parse()
                    .map_err(|_| "Failed to parse vertex x".to_string())?;
                let y: f32 = parts[2].parse()
                    .map_err(|_| "Failed to parse vertex y".to_string())?;
                let z: f32 = parts[3].parse()
                    .map_err(|_| "Failed to parse vertex z".to_string())?;
                vertices.push([x, y, z]);
            }
            "f" => {
                if parts.len() < 4 {
                    return Err("Invalid face format".to_string());
                }

                let v1: usize = parts[1].split('/').next()
                    .ok_or("Invalid face index".to_string())?
                    .parse::<usize>()
                    .map_err(|_| "Failed to parse face index".to_string())?
                    .saturating_sub(1);

                let v2: usize = parts[2].split('/').next()
                    .ok_or("Invalid face index".to_string())?
                    .parse::<usize>()
                    .map_err(|_| "Failed to parse face index".to_string())?
                    .saturating_sub(1);

                let v3: usize = parts[3].split('/').next()
                    .ok_or("Invalid face index".to_string())?
                    .parse::<usize>()
                    .map_err(|_| "Failed to parse face index".to_string())?
                    .saturating_sub(1);

                let face_idx = faces.len();
                faces.push([v1, v2, v3]);
                groups.entry(current_group.clone()).or_default().push(face_idx);
            }
            "g" => {
                if parts.len() > 1 {
                    current_group = parts[1].to_string();
                }
            }
            "usemtl" => {
                if parts.len() > 1 {
                    current_group = parts[1].to_string();
                }
            }
            _ => {}
        }
    }

    Ok(ObjData {
        vertices,
        faces,
        groups,
    })
}
