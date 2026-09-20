// ============================================================================
// CRON Compiler: GGUF Pure-Rust Binary Parser & Importer (gguf.rs)
// Module: cronc::model_importer::gguf
// (C) 2026 CRON Language Project - SSS+ Tier Industrial Systems
// ============================================================================

use std::collections::HashMap;

/// GGUF Magic Constant: "GGUF" in ASCII (0x46554747)
pub const GGUF_MAGIC: u32 = 0x46554747;

/// GGUF Supported Versions
pub const GGUF_VERSION_2: u32 = 2;
pub const GGUF_VERSION_3: u32 = 3;

/// GGUF Value Types
#[derive(Debug, Clone, PartialEq)]
pub enum GgufValue {
    Uint8(u8),
    Int8(i8),
    Uint16(u16),
    Int16(i16),
    Uint32(u32),
    Int32(i32),
    Float32(f32),
    Bool(bool),
    String(String),
    Array(Vec<GgufValue>),
    Uint64(u64),
    Int64(i64),
    Float64(f64),
}

/// GGUF Tensor Types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GgufTensorType {
    F32,
    F16,
    Q4_0,
    Q4_1,
    Q5_0,
    Q5_1,
    Q8_0,
    Q8_1,
    I8,
    I16,
    I32,
    Unknown(u32),
}

impl GgufTensorType {
    pub fn to_u32(&self) -> u32 {
        match self {
            GgufTensorType::F32 => 0,
            GgufTensorType::F16 => 1,
            GgufTensorType::Q4_0 => 2,
            GgufTensorType::Q4_1 => 3,
            GgufTensorType::Q5_0 => 6,
            GgufTensorType::Q5_1 => 7,
            GgufTensorType::Q8_0 => 8,
            GgufTensorType::Q8_1 => 9,
            GgufTensorType::I8 => 16,
            GgufTensorType::I16 => 17,
            GgufTensorType::I32 => 18,
            GgufTensorType::Unknown(u) => *u,
        }
    }
}

impl From<u32> for GgufTensorType {
    fn from(val: u32) -> Self {
        match val {
            0 => GgufTensorType::F32,
            1 => GgufTensorType::F16,
            2 => GgufTensorType::Q4_0,
            3 => GgufTensorType::Q4_1,
            6 => GgufTensorType::Q5_0,
            7 => GgufTensorType::Q5_1,
            8 => GgufTensorType::Q8_0,
            9 => GgufTensorType::Q8_1,
            16 => GgufTensorType::I8,
            17 => GgufTensorType::I16,
            18 => GgufTensorType::I32,
            other => GgufTensorType::Unknown(other),
        }
    }
}

/// GGUF Tensor Info Record
#[derive(Debug, Clone)]
pub struct GgufTensorInfo {
    pub name: String,
    pub dimensions: Vec<u64>,
    pub tensor_type: GgufTensorType,
    pub offset: u64,
}

/// Parsed GGUF Model Header and Tensor Directory
#[derive(Debug, Clone)]
pub struct GgufModel {
    pub version: u32,
    pub tensor_count: u64,
    pub metadata: HashMap<String, GgufValue>,
    pub tensors: HashMap<String, GgufTensorInfo>,
    pub data_offset: u64,
    pub alignment: usize,
}

/// Helper for reading binary data with bounds checks
struct ByteCursor<'a> {
    data: &'a [u8],
    pos: usize,
}

impl<'a> ByteCursor<'a> {
    fn new(data: &'a [u8]) -> Self {
        Self { data, pos: 0 }
    }

    fn read_u8(&mut self) -> Result<u8, String> {
        if self.pos + 1 > self.data.len() {
            return Err("Unexpected EOF reading u8".to_string());
        }
        let val = self.data[self.pos];
        self.pos += 1;
        Ok(val)
    }

    fn read_i8(&mut self) -> Result<i8, String> {
        self.read_u8().map(|b| b as i8)
    }

    fn read_u16(&mut self) -> Result<u16, String> {
        if self.pos + 2 > self.data.len() {
            return Err("Unexpected EOF reading u16".to_string());
        }
        let val = u16::from_le_bytes(self.data[self.pos..self.pos + 2].try_into().unwrap());
        self.pos += 2;
        Ok(val)
    }

    fn read_i16(&mut self) -> Result<i16, String> {
        self.read_u16().map(|v| v as i16)
    }

    fn read_u32(&mut self) -> Result<u32, String> {
        if self.pos + 4 > self.data.len() {
            return Err("Unexpected EOF reading u32".to_string());
        }
        let val = u32::from_le_bytes(self.data[self.pos..self.pos + 4].try_into().unwrap());
        self.pos += 4;
        Ok(val)
    }

    fn read_i32(&mut self) -> Result<i32, String> {
        self.read_u32().map(|v| v as i32)
    }

    fn read_f32(&mut self) -> Result<f32, String> {
        self.read_u32().map(f32::from_bits)
    }

    fn read_u64(&mut self) -> Result<u64, String> {
        if self.pos + 8 > self.data.len() {
            return Err("Unexpected EOF reading u64".to_string());
        }
        let val = u64::from_le_bytes(self.data[self.pos..self.pos + 8].try_into().unwrap());
        self.pos += 8;
        Ok(val)
    }

    fn read_i64(&mut self) -> Result<i64, String> {
        self.read_u64().map(|v| v as i64)
    }

    fn read_f64(&mut self) -> Result<f64, String> {
        self.read_u64().map(f64::from_bits)
    }

    fn read_string(&mut self) -> Result<String, String> {
        let len = self.read_u64()? as usize;
        if self.pos + len > self.data.len() {
            return Err(format!("Unexpected EOF reading string of length {}", len));
        }
        let s = std::str::from_utf8(&self.data[self.pos..self.pos + len])
            .map_err(|e| format!("Invalid UTF-8 string: {}", e))?
            .to_string();
        self.pos += len;
        Ok(s)
    }

    fn read_value(&mut self, type_id: u32) -> Result<GgufValue, String> {
        match type_id {
            0 => self.read_u8().map(GgufValue::Uint8),
            1 => self.read_i8().map(GgufValue::Int8),
            2 => self.read_u16().map(GgufValue::Uint16),
            3 => self.read_i16().map(GgufValue::Int16),
            4 => self.read_u32().map(GgufValue::Uint32),
            5 => self.read_i32().map(GgufValue::Int32),
            6 => self.read_f32().map(GgufValue::Float32),
            7 => self.read_u8().map(|b| GgufValue::Bool(b != 0)),
            8 => self.read_string().map(GgufValue::String),
            9 => {
                let elem_type = self.read_u32()?;
                let elem_count = self.read_u64()? as usize;
                let mut arr = Vec::with_capacity(elem_count.min(100_000));
                for _ in 0..elem_count {
                    arr.push(self.read_value(elem_type)?);
                }
                Ok(GgufValue::Array(arr))
            }
            10 => self.read_u64().map(GgufValue::Uint64),
            11 => self.read_i64().map(GgufValue::Int64),
            12 => self.read_f64().map(GgufValue::Float64),
            other => Err(format!("Unsupported GGUF value type ID {}", other)),
        }
    }
}

/// Parse a GGUF binary buffer (header + metadata + tensor descriptors).
/// Does not copy or load full tensor weight buffers into RAM.
pub fn parse_gguf_header(bytes: &[u8]) -> Result<GgufModel, String> {
    if bytes.len() < 16 {
        return Err("GGUF binary too short: expected at least 16 bytes for header".to_string());
    }

    let mut cursor = ByteCursor::new(bytes);

    let magic = cursor.read_u32()?;
    if magic != GGUF_MAGIC {
        return Err(format!(
            "Invalid GGUF magic: expected 0x{:08X} ('GGUF'), got 0x{:08X}",
            GGUF_MAGIC, magic
        ));
    }

    let version = cursor.read_u32()?;
    if version != GGUF_VERSION_2 && version != GGUF_VERSION_3 {
        return Err(format!(
            "Unsupported GGUF version: {}. Supported versions: 2, 3",
            version
        ));
    }

    let tensor_count = cursor.read_u64()?;
    let metadata_kv_count = cursor.read_u64()?;

    // 1. Read metadata key-values
    let mut metadata = HashMap::with_capacity(metadata_kv_count as usize);
    let mut alignment = 32usize;

    for _ in 0..metadata_kv_count {
        let key = cursor.read_string()?;
        let type_id = cursor.read_u32()?;
        let val = cursor.read_value(type_id)?;

        if key == "general.alignment" {
            if let GgufValue::Uint32(align) = val {
                alignment = align as usize;
            } else if let GgufValue::Uint64(align) = val {
                alignment = align as usize;
            }
        }

        metadata.insert(key, val);
    }

    // 2. Read tensor descriptors
    let mut tensors = HashMap::with_capacity(tensor_count as usize);
    for _ in 0..tensor_count {
        let name = cursor.read_string()?;
        let n_dims = cursor.read_u32()? as usize;
        let mut dimensions = Vec::with_capacity(n_dims);
        for _ in 0..n_dims {
            dimensions.push(cursor.read_u64()?);
        }
        let type_id = cursor.read_u32()?;
        let offset = cursor.read_u64()?;

        tensors.insert(
            name.clone(),
            GgufTensorInfo {
                name,
                dimensions,
                tensor_type: GgufTensorType::from(type_id),
                offset,
            },
        );
    }

    // 3. Align cursor to data section
    let header_end_pos = cursor.pos;
    let padding = (alignment - (header_end_pos % alignment)) % alignment;
    let data_offset = (header_end_pos + padding) as u64;

    Ok(GgufModel {
        version,
        tensor_count,
        metadata,
        tensors,
        data_offset,
        alignment,
    })
}

/// Helper function to create a synthetic GGUF binary buffer for testing and benchmarking
pub fn create_synthetic_gguf(
    metadata_kvs: &[(&str, GgufValue)],
    tensors: &[(&str, Vec<u64>, GgufTensorType, &[u8])],
    alignment: usize,
) -> Vec<u8> {
    let mut buf = Vec::new();

    // 1. Header
    buf.extend_from_slice(&GGUF_MAGIC.to_le_bytes());
    buf.extend_from_slice(&GGUF_VERSION_3.to_le_bytes());
    buf.extend_from_slice(&(tensors.len() as u64).to_le_bytes());
    buf.extend_from_slice(&(metadata_kvs.len() as u64).to_le_bytes());

    // Helper closure to write string
    fn write_str(buf: &mut Vec<u8>, s: &str) {
        buf.extend_from_slice(&(s.len() as u64).to_le_bytes());
        buf.extend_from_slice(s.as_bytes());
    }

    // Helper closure to write value
    fn write_val(buf: &mut Vec<u8>, val: &GgufValue) {
        match val {
            GgufValue::Uint8(v) => {
                buf.extend_from_slice(&0u32.to_le_bytes());
                buf.push(*v);
            }
            GgufValue::Int8(v) => {
                buf.extend_from_slice(&1u32.to_le_bytes());
                buf.push(*v as u8);
            }
            GgufValue::Uint16(v) => {
                buf.extend_from_slice(&2u32.to_le_bytes());
                buf.extend_from_slice(&v.to_le_bytes());
            }
            GgufValue::Int16(v) => {
                buf.extend_from_slice(&3u32.to_le_bytes());
                buf.extend_from_slice(&v.to_le_bytes());
            }
            GgufValue::Uint32(v) => {
                buf.extend_from_slice(&4u32.to_le_bytes());
                buf.extend_from_slice(&v.to_le_bytes());
            }
            GgufValue::Int32(v) => {
                buf.extend_from_slice(&5u32.to_le_bytes());
                buf.extend_from_slice(&v.to_le_bytes());
            }
            GgufValue::Float32(v) => {
                buf.extend_from_slice(&6u32.to_le_bytes());
                buf.extend_from_slice(&v.to_bits().to_le_bytes());
            }
            GgufValue::Bool(v) => {
                buf.extend_from_slice(&7u32.to_le_bytes());
                buf.push(if *v { 1 } else { 0 });
            }
            GgufValue::String(s) => {
                buf.extend_from_slice(&8u32.to_le_bytes());
                write_str(buf, s);
            }
            GgufValue::Array(arr) => {
                buf.extend_from_slice(&9u32.to_le_bytes());
                let elem_type = if let Some(first) = arr.first() {
                    match first {
                        GgufValue::Uint32(_) => 4u32,
                        GgufValue::Float32(_) => 6u32,
                        GgufValue::String(_) => 8u32,
                        _ => 0u32,
                    }
                } else {
                    0u32
                };
                buf.extend_from_slice(&elem_type.to_le_bytes());
                buf.extend_from_slice(&(arr.len() as u64).to_le_bytes());
                for elem in arr {
                    match elem {
                        GgufValue::Uint32(x) => buf.extend_from_slice(&x.to_le_bytes()),
                        GgufValue::Float32(x) => buf.extend_from_slice(&x.to_bits().to_le_bytes()),
                        GgufValue::String(x) => write_str(buf, x),
                        _ => {}
                    }
                }
            }
            GgufValue::Uint64(v) => {
                buf.extend_from_slice(&10u32.to_le_bytes());
                buf.extend_from_slice(&v.to_le_bytes());
            }
            GgufValue::Int64(v) => {
                buf.extend_from_slice(&11u32.to_le_bytes());
                buf.extend_from_slice(&v.to_le_bytes());
            }
            GgufValue::Float64(v) => {
                buf.extend_from_slice(&12u32.to_le_bytes());
                buf.extend_from_slice(&v.to_bits().to_le_bytes());
            }
        }
    }

    // 2. Metadata KVs
    for (k, v) in metadata_kvs {
        write_str(&mut buf, k);
        write_val(&mut buf, v);
    }

    // 3. Tensor infos
    let mut current_offset = 0u64;
    for (name, dims, t_type, data) in tensors {
        write_str(&mut buf, name);
        buf.extend_from_slice(&(dims.len() as u32).to_le_bytes());
        for d in dims {
            buf.extend_from_slice(&d.to_le_bytes());
        }
        let type_id = t_type.to_u32();
        buf.extend_from_slice(&type_id.to_le_bytes());
        buf.extend_from_slice(&current_offset.to_le_bytes());

        current_offset += data.len() as u64;
        let pad = (alignment - (data.len() % alignment)) % alignment;
        current_offset += pad as u64;
    }

    // 4. Alignment padding to data section
    let pad = (alignment - (buf.len() % alignment)) % alignment;
    buf.resize(buf.len() + pad, 0);

    // 5. Tensor payloads
    for (_, _, _, data) in tensors {
        buf.extend_from_slice(data);
        let pad = (alignment - (data.len() % alignment)) % alignment;
        buf.resize(buf.len() + pad, 0);
    }

    buf
}
