// ============================================================================
// CRON Compiler: SafeTensors Pure-Rust Binary Parser
// Module: cronc::model_importer::safetensors
// (C) 2026 CRON Language Project - SSS+ Tier Industrial Systems
// ============================================================================

use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum SafeTensorDType {
    F32,
    F16,
    BF16,
    I8,
    I32,
    U8,
    Unknown(String),
}

#[derive(Debug, Clone)]
pub struct SafeTensorMeta {
    pub name: String,
    pub dtype: SafeTensorDType,
    pub shape: Vec<usize>,
    pub data_offsets: (usize, usize),
}

#[derive(Debug, Clone)]
pub struct SafeTensorData {
    pub name: String,
    pub dtype: SafeTensorDType,
    pub shape: Vec<usize>,
    pub values: Vec<f32>,
}

#[derive(Debug, Clone)]
pub struct SafeTensorsModel {
    pub metadata: HashMap<String, String>,
    pub tensors: HashMap<String, SafeTensorData>,
}

/// Parse a HuggingFace SafeTensors binary byte buffer into structured metadata and f32 tensor buffers.
pub fn parse_safetensors(bytes: &[u8]) -> Result<SafeTensorsModel, String> {
    if bytes.len() < 8 {
        return Err("SafeTensors binary too small: expected at least 8 bytes header length".to_string());
    }

    let header_len = u64::from_le_bytes(bytes[0..8].try_into().unwrap()) as usize;
    if bytes.len() < 8 + header_len {
        return Err(format!(
            "SafeTensors binary truncated: header expects {} bytes, but buffer has {}",
            header_len,
            bytes.len() - 8
        ));
    }

    let header_bytes = &bytes[8..8 + header_len];
    let header_str = std::str::from_utf8(header_bytes)
        .map_err(|e| format!("SafeTensors header is not valid UTF-8: {}", e))?;

    let base_data_offset = 8 + header_len;
    let data_payload = &bytes[base_data_offset..];

    let (metadata, tensor_metas) = parse_safetensors_json(header_str)?;

    let mut tensors = HashMap::new();
    for meta in tensor_metas {
        let (start, end) = meta.data_offsets;
        if end < start || end > data_payload.len() {
            return Err(format!(
                "Tensor '{}' data offsets [{}, {}] exceed payload length {}",
                meta.name, start, end, data_payload.len()
            ));
        }
        let tensor_bytes = &data_payload[start..end];
        let values = decode_tensor_buffer(tensor_bytes, &meta.dtype)?;
        tensors.insert(
            meta.name.clone(),
            SafeTensorData {
                name: meta.name,
                dtype: meta.dtype,
                shape: meta.shape,
                values,
            },
        );
    }

    Ok(SafeTensorsModel { metadata, tensors })
}

/// Helper function to decode raw tensor bytes into f32 values according to its dtype.
fn decode_tensor_buffer(bytes: &[u8], dtype: &SafeTensorDType) -> Result<Vec<f32>, String> {
    match dtype {
        SafeTensorDType::F32 => {
            if !bytes.len().is_multiple_of(4) {
                return Err("F32 tensor byte size not multiple of 4".to_string());
            }
            let count = bytes.len() / 4;
            let mut out = Vec::with_capacity(count);
            for i in 0..count {
                let chunk = &bytes[i * 4..(i + 1) * 4];
                out.push(f32::from_le_bytes(chunk.try_into().unwrap()));
            }
            Ok(out)
        }
        SafeTensorDType::F16 => {
            if !bytes.len().is_multiple_of(2) {
                return Err("F16 tensor byte size not multiple of 2".to_string());
            }
            let count = bytes.len() / 2;
            let mut out = Vec::with_capacity(count);
            for i in 0..count {
                let chunk = &bytes[i * 2..(i + 1) * 2];
                let u = u16::from_le_bytes(chunk.try_into().unwrap());
                out.push(f16_to_f32(u));
            }
            Ok(out)
        }
        SafeTensorDType::BF16 => {
            if !bytes.len().is_multiple_of(2) {
                return Err("BF16 tensor byte size not multiple of 2".to_string());
            }
            let count = bytes.len() / 2;
            let mut out = Vec::with_capacity(count);
            for i in 0..count {
                let chunk = &bytes[i * 2..(i + 1) * 2];
                let u = u16::from_le_bytes(chunk.try_into().unwrap());
                // BF16 is just the top 16 bits of FP32
                let f_bits = (u as u32) << 16;
                out.push(f32::from_bits(f_bits));
            }
            Ok(out)
        }
        SafeTensorDType::I8 => {
            Ok(bytes.iter().map(|&b| (b as i8) as f32).collect())
        }
        SafeTensorDType::U8 => {
            Ok(bytes.iter().map(|&b| b as f32).collect())
        }
        SafeTensorDType::I32 => {
            if !bytes.len().is_multiple_of(4) {
                return Err("I32 tensor byte size not multiple of 4".to_string());
            }
            let count = bytes.len() / 4;
            let mut out = Vec::with_capacity(count);
            for i in 0..count {
                let chunk = &bytes[i * 4..(i + 1) * 4];
                let val = i32::from_le_bytes(chunk.try_into().unwrap());
                out.push(val as f32);
            }
            Ok(out)
        }
        SafeTensorDType::Unknown(s) => Err(format!("Unsupported SafeTensors dtype '{}'", s)),
    }
}

/// Convert IEEE 754 half-precision float (u16) to standard single-precision f32.
pub fn f16_to_f32(h: u16) -> f32 {
    let sign = ((h >> 15) & 1) as u32;
    let exp = ((h >> 10) & 0x1F) as u32;
    let frac = (h & 0x3FF) as u32;

    if exp == 0 {
        if frac == 0 {
            // Signed zero
            f32::from_bits(sign << 31)
        } else {
            // Subnormal
            let mut m = frac;
            let mut e = 0;
            while (m & 0x400) == 0 {
                m <<= 1;
                e += 1;
            }
            m &= 0x3FF;
            let f_exp = (127 - 15 - e + 1) as u32;
            let f_frac = m << 13;
            f32::from_bits((sign << 31) | (f_exp << 23) | f_frac)
        }
    } else if exp == 31 {
        // Inf or NaN
        let f_exp = 0xFFu32;
        let f_frac = frac << 13;
        f32::from_bits((sign << 31) | (f_exp << 23) | f_frac)
    } else {
        // Normalized
        let f_exp = exp + (127 - 15);
        let f_frac = frac << 13;
        f32::from_bits((sign << 31) | (f_exp << 23) | f_frac)
    }
}

/// Pure-Rust zero-dependency lightweight JSON parser specifically tailored for SafeTensors header format.
pub fn parse_safetensors_json(
    json: &str,
) -> Result<(HashMap<String, String>, Vec<SafeTensorMeta>), String> {
    let mut metadata = HashMap::new();
    let mut tensors = Vec::new();

    let trimmed = json.trim();
    if !trimmed.starts_with('{') || !trimmed.ends_with('}') {
        return Err("SafeTensors JSON header must be an object enclosing with { ... }".to_string());
    }

    let inner = &trimmed[1..trimmed.len() - 1].trim();
    let mut chars = inner.char_indices().peekable();

    while chars.peek().is_some() {
        skip_ws_and_commas(&mut chars);
        if chars.peek().is_none() {
            break;
        }

        // Read Key
        let key = match read_json_string(&mut chars)? {
            Some(k) => k,
            None => break,
        };

        skip_ws(&mut chars);
        match chars.next() {
            Some((_, ':')) => {}
            other => return Err(format!("Expected ':' after key '{}', found {:?}", key, other)),
        }
        skip_ws(&mut chars);

        if key == "__metadata__" {
            // Skip or parse metadata sub-object
            let meta_obj = read_json_object_raw(&mut chars)?;
            metadata.insert("__metadata__".to_string(), meta_obj);
        } else {
            // Tensor entry: { "dtype": "...", "shape": [...], "data_offsets": [...] }
            let meta = parse_single_tensor_entry(&key, &mut chars)?;
            tensors.push(meta);
        }
    }

    Ok((metadata, tensors))
}

fn skip_ws<I: Iterator<Item = (usize, char)>>(chars: &mut std::iter::Peekable<I>) {
    while let Some(&(_, c)) = chars.peek() {
        if c.is_whitespace() {
            chars.next();
        } else {
            break;
        }
    }
}

fn skip_ws_and_commas<I: Iterator<Item = (usize, char)>>(chars: &mut std::iter::Peekable<I>) {
    while let Some(&(_, c)) = chars.peek() {
        if c.is_whitespace() || c == ',' {
            chars.next();
        } else {
            break;
        }
    }
}

fn read_json_string<I: Iterator<Item = (usize, char)>>(
    chars: &mut std::iter::Peekable<I>,
) -> Result<Option<String>, String> {
    skip_ws(chars);
    match chars.next() {
        Some((_, '"')) => {}
        None => return Ok(None),
        Some((_, c)) => return Err(format!("Expected string starting with '\"', got '{}'", c)),
    }

    let mut s = String::new();
    let mut escaped = false;
    for (_, c) in chars.by_ref() {
        if escaped {
            match c {
                '"' => s.push('"'),
                '\\' => s.push('\\'),
                '/' => s.push('/'),
                'b' => s.push('\x08'),
                'f' => s.push('\x0C'),
                'n' => s.push('\n'),
                'r' => s.push('\r'),
                't' => s.push('\t'),
                _ => s.push(c),
            }
            escaped = false;
        } else if c == '\\' {
            escaped = true;
        } else if c == '"' {
            return Ok(Some(s));
        } else {
            s.push(c);
        }
    }
    Err("Unterminated string in JSON header".to_string())
}

fn read_json_object_raw<I: Iterator<Item = (usize, char)>>(
    chars: &mut std::iter::Peekable<I>,
) -> Result<String, String> {
    skip_ws(chars);
    match chars.next() {
        Some((_, '{')) => {}
        other => return Err(format!("Expected '{{' for object, got {:?}", other)),
    }
    let mut depth = 1;
    let mut out = String::from("{");
    let mut in_str = false;
    let mut escaped = false;

    for (_, c) in chars.by_ref() {
        out.push(c);
        if in_str {
            if escaped {
                escaped = false;
            } else if c == '\\' {
                escaped = true;
            } else if c == '"' {
                in_str = false;
            }
        } else if c == '"' {
            in_str = true;
        } else if c == '{' {
            depth += 1;
        } else if c == '}' {
            depth -= 1;
            if depth == 0 {
                return Ok(out);
            }
        }
    }
    Err("Unclosed object in SafeTensors JSON header".to_string())
}

fn parse_single_tensor_entry<I: Iterator<Item = (usize, char)>>(
    tensor_name: &str,
    chars: &mut std::iter::Peekable<I>,
) -> Result<SafeTensorMeta, String> {
    skip_ws(chars);
    match chars.next() {
        Some((_, '{')) => {}
        other => return Err(format!("Expected '{{' starting tensor spec, got {:?}", other)),
    }

    let mut dtype = None;
    let mut shape = None;
    let mut data_offsets = None;

    while let Some(&(_, c)) = chars.peek() {
        if c == '}' {
            chars.next();
            break;
        }
        skip_ws_and_commas(chars);
        if chars.peek().map(|&(_, c)| c == '}').unwrap_or(false) {
            chars.next();
            break;
        }

        let field = read_json_string(chars)?
            .ok_or_else(|| "Expected field name in tensor definition".to_string())?;

        skip_ws(chars);
        match chars.next() {
            Some((_, ':')) => {}
            other => return Err(format!("Expected ':' after '{}', got {:?}", field, other)),
        }
        skip_ws(chars);

        match field.as_str() {
            "dtype" => {
                let dt_str = read_json_string(chars)?
                    .ok_or_else(|| "Expected string for dtype".to_string())?;
                let dt = match dt_str.to_uppercase().as_str() {
                    "F32" => SafeTensorDType::F32,
                    "F16" => SafeTensorDType::F16,
                    "BF16" => SafeTensorDType::BF16,
                    "I8" => SafeTensorDType::I8,
                    "U8" => SafeTensorDType::U8,
                    "I32" => SafeTensorDType::I32,
                    other => SafeTensorDType::Unknown(other.to_string()),
                };
                dtype = Some(dt);
            }
            "shape" => {
                shape = Some(read_json_int_array(chars)?);
            }
            "data_offsets" => {
                let offsets = read_json_int_array(chars)?;
                if offsets.len() != 2 {
                    return Err(format!(
                        "Tensor '{}' data_offsets must have exactly 2 elements, found {:?}",
                        tensor_name, offsets
                    ));
                }
                data_offsets = Some((offsets[0], offsets[1]));
            }
            _ => {
                // Skip unknown value
                skip_json_value(chars)?;
            }
        }
    }

    let dtype = dtype.ok_or_else(|| format!("Tensor '{}' missing 'dtype'", tensor_name))?;
    let shape = shape.ok_or_else(|| format!("Tensor '{}' missing 'shape'", tensor_name))?;
    let data_offsets = data_offsets
        .ok_or_else(|| format!("Tensor '{}' missing 'data_offsets'", tensor_name))?;

    Ok(SafeTensorMeta {
        name: tensor_name.to_string(),
        dtype,
        shape,
        data_offsets,
    })
}

fn read_json_int_array<I: Iterator<Item = (usize, char)>>(
    chars: &mut std::iter::Peekable<I>,
) -> Result<Vec<usize>, String> {
    skip_ws(chars);
    match chars.next() {
        Some((_, '[')) => {}
        other => return Err(format!("Expected '[' for array, found {:?}", other)),
    }

    let mut arr = Vec::new();
    while let Some(&(_, c)) = chars.peek() {
        if c == ']' {
            chars.next();
            break;
        }
        skip_ws_and_commas(chars);
        if chars.peek().map(|&(_, c)| c == ']').unwrap_or(false) {
            chars.next();
            break;
        }

        let mut num_str = String::new();
        while let Some(&(_, ch)) = chars.peek() {
            if ch.is_ascii_digit() {
                num_str.push(ch);
                chars.next();
            } else {
                break;
            }
        }

        if num_str.is_empty() {
            return Err("Expected integer in array".to_string());
        }

        let val = num_str
            .parse::<usize>()
            .map_err(|e| format!("Failed parsing int '{}': {}", num_str, e))?;
        arr.push(val);
    }

    Ok(arr)
}

fn skip_json_value<I: Iterator<Item = (usize, char)>>(
    chars: &mut std::iter::Peekable<I>,
) -> Result<(), String> {
    skip_ws(chars);
    match chars.peek() {
        Some(&(_, '"')) => {
            read_json_string(chars)?;
        }
        Some(&(_, '{')) => {
            read_json_object_raw(chars)?;
        }
        Some(&(_, '[')) => {
            read_json_int_array(chars)?;
        }
        _ => {
            while let Some(&(_, c)) = chars.peek() {
                if c == ',' || c == '}' || c == ']' || c.is_whitespace() {
                    break;
                }
                chars.next();
            }
        }
    }
    Ok(())
}

/// Helper function to create a synthetic SafeTensors byte buffer for unit tests and benchmarks.
pub fn create_synthetic_safetensors(
    tensors: &[(&str, SafeTensorDType, &[usize], &[f32])],
) -> Vec<u8> {
    let mut header_map = String::from("{");
    let mut payload = Vec::new();

    for (i, &(name, ref dtype, shape, values)) in tensors.iter().enumerate() {
        if i > 0 {
            header_map.push(',');
        }
        let dt_str = match dtype {
            SafeTensorDType::F32 => "F32",
            SafeTensorDType::F16 => "F16",
            SafeTensorDType::BF16 => "BF16",
            SafeTensorDType::I8 => "I8",
            SafeTensorDType::U8 => "U8",
            SafeTensorDType::I32 => "I32",
            SafeTensorDType::Unknown(s) => s.as_str(),
        };

        let start_offset = payload.len();
        match dtype {
            SafeTensorDType::F32 => {
                for &v in values {
                    payload.extend_from_slice(&v.to_le_bytes());
                }
            }
            SafeTensorDType::F16 => {
                for &v in values {
                    let bits = v.to_bits();
                    let sign = (bits >> 31) & 1;
                    let exp = (bits >> 23) & 0xFF;
                    let frac = bits & 0x7FFFFF;
                    let h_exp = if exp >= 112 { (exp - 112).min(31) } else { 0 };
                    let h_frac = frac >> 13;
                    let h = ((sign << 15) | (h_exp << 10) | (h_frac & 0x3FF)) as u16;
                    payload.extend_from_slice(&h.to_le_bytes());
                }
            }
            SafeTensorDType::BF16 => {
                for &v in values {
                    let top16 = (v.to_bits() >> 16) as u16;
                    payload.extend_from_slice(&top16.to_le_bytes());
                }
            }
            SafeTensorDType::I8 => {
                for &v in values {
                    payload.push(v as i8 as u8);
                }
            }
            SafeTensorDType::U8 => {
                for &v in values {
                    payload.push(v as u8);
                }
            }
            SafeTensorDType::I32 => {
                for &v in values {
                    payload.extend_from_slice(&(v as i32).to_le_bytes());
                }
            }
            SafeTensorDType::Unknown(_) => {}
        }
        let end_offset = payload.len();

        let shape_str = shape
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<_>>()
            .join(",");

        header_map.push_str(&format!(
            "\"{}\":{{\"dtype\":\"{}\",\"shape\":[{}],\"data_offsets\":[{},{}]}}",
            name, dt_str, shape_str, start_offset, end_offset
        ));
    }
    header_map.push('}');

    let header_bytes = header_map.as_bytes();
    let header_len = header_bytes.len() as u64;

    let mut out = Vec::with_capacity(8 + header_bytes.len() + payload.len());
    out.extend_from_slice(&header_len.to_le_bytes());
    out.extend_from_slice(header_bytes);
    out.extend_from_slice(&payload);
    out
}
