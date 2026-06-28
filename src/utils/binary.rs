pub fn read_u16(data: &[u8], offset: usize) -> Result<u16, String> {
    if data.len() < offset + 2 {
        return Err("Not enough data to read u16".into());
    }
    Ok(u16::from_be_bytes(
        data[offset..offset + 2].try_into().unwrap(),
    ))
}

pub fn read_u32(data: &[u8], offset: usize) -> Result<u32, String> {
    if data.len() < offset + 4 {
        return Err("Not enough data to read u32".into());
    }
    Ok(u32::from_be_bytes(
        data[offset..offset + 4].try_into().unwrap(),
    ))
}

pub fn read_f32(data: &[u8], offset: usize) -> Result<f32, String> {
    if data.len() < offset + 4 {
        return Err("Not enough data to read f32".into());
    }
    Ok(f32::from_bits(u32::from_be_bytes(
        data[offset..offset + 4].try_into().unwrap(),
    )))
}

pub fn read_vec_f32(data: &[u8], offset: usize, count: usize) -> Result<Vec<f32>, String> {
    if data.len() < offset + count * 4 {
        return Err("Not enough data to read f32 vec".into());
    }
    let mut result = Vec::with_capacity(count);
    for i in 0..count {
        result.push(f32::from_bits(u32::from_be_bytes(
            data[offset + i * 4..offset + i * 4 + 4].try_into().unwrap(),
        )));
    }
    Ok(result)
}

pub fn read_vec_u32(data: &[u8], offset: usize, count: usize) -> Result<Vec<u32>, String> {
    if data.len() < offset + count * 4 {
        return Err("Not enough data to read u32 vec".into());
    }
    let mut result = Vec::with_capacity(count);
    for i in 0..count {
        result.push(u32::from_be_bytes(
            data[offset + i * 4..offset + i * 4 + 4].try_into().unwrap(),
        ));
    }
    Ok(result)
}

pub fn cross(a: [f32; 3], b: [f32; 3]) -> [f32; 3] {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}

pub fn dot(a: [f32; 3], b: [f32; 3]) -> f32 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

pub fn add(a: [f32; 3], b: [f32; 3]) -> [f32; 3] {
    [a[0] + b[0], a[1] + b[1], a[2] + b[2]]
}

pub fn scale(a: [f32; 3], s: f32) -> [f32; 3] {
    [a[0] * s, a[1] * s, a[2] * s]
}