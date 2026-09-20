// ============================================================================
// CRON Compiler: Pure-Rust Zero-Dependency ONNX Protobuf Wire Decoder
// Module: cronc::model_importer::onnx
// (C) 2026 CRON Language Project - SSS+ Tier Industrial Systems
// ============================================================================

use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct OnnxTensor {
    pub name: String,
    pub dims: Vec<usize>,
    pub data_type: i32, // 1 = FLOAT, 2 = UINT8, 3 = INT8, 7 = INT64, 10 = FLOAT16
    pub float_values: Vec<f32>,
}

#[derive(Debug, Clone)]
pub struct OnnxAttribute {
    pub name: String,
    pub f: Option<f32>,
    pub i: Option<i64>,
    pub s: Option<String>,
    pub ints: Vec<i64>,
    pub floats: Vec<f32>,
}

#[derive(Debug, Clone)]
pub struct OnnxNode {
    pub op_type: String,
    pub name: String,
    pub inputs: Vec<String>,
    pub outputs: Vec<String>,
    pub attributes: HashMap<String, OnnxAttribute>,
}

#[derive(Debug, Clone)]
pub struct OnnxValueInfo {
    pub name: String,
    pub dims: Vec<usize>,
}

#[derive(Debug, Clone)]
pub struct OnnxGraph {
    pub name: String,
    pub nodes: Vec<OnnxNode>,
    pub initializers: HashMap<String, OnnxTensor>,
    pub inputs: Vec<OnnxValueInfo>,
    pub outputs: Vec<OnnxValueInfo>,
}

#[derive(Debug, Clone)]
pub struct OnnxModel {
    pub ir_version: i64,
    pub producer_name: String,
    pub graph: OnnxGraph,
}

// ----------------------------------------------------------------------------
// Protobuf Wire Decoding Primitives
// ----------------------------------------------------------------------------

struct ProtoReader<'a> {
    data: &'a [u8],
    pos: usize,
}

impl<'a> ProtoReader<'a> {
    fn new(data: &'a [u8]) -> Self {
        Self { data, pos: 0 }
    }

    fn is_eof(&self) -> bool {
        self.pos >= self.data.len()
    }

    fn read_byte(&mut self) -> Result<u8, String> {
        if self.pos >= self.data.len() {
            return Err("Unexpected EOF reading byte in Protobuf stream".to_string());
        }
        let b = self.data[self.pos];
        self.pos += 1;
        Ok(b)
    }

    fn read_bytes(&mut self, len: usize) -> Result<&'a [u8], String> {
        if self.pos + len > self.data.len() {
            return Err("Unexpected EOF reading slice in Protobuf stream".to_string());
        }
        let slice = &self.data[self.pos..self.pos + len];
        self.pos += len;
        Ok(slice)
    }

    fn read_varint(&mut self) -> Result<u64, String> {
        let mut result = 0u64;
        let mut shift = 0;
        loop {
            let b = self.read_byte()?;
            result |= ((b & 0x7F) as u64) << shift;
            if (b & 0x80) == 0 {
                break;
            }
            shift += 7;
            if shift >= 64 {
                return Err("Varint overflow (>64 bits)".to_string());
            }
        }
        Ok(result)
    }

    fn read_tag(&mut self) -> Result<(u32, u8), String> {
        let tag = self.read_varint()?;
        let field_num = (tag >> 3) as u32;
        let wire_type = (tag & 0x07) as u8;
        Ok((field_num, wire_type))
    }

    fn skip_field(&mut self, wire_type: u8) -> Result<(), String> {
        match wire_type {
            0 => {
                let _ = self.read_varint()?;
            }
            1 => {
                let _ = self.read_bytes(8)?;
            }
            2 => {
                let len = self.read_varint()? as usize;
                let _ = self.read_bytes(len)?;
            }
            5 => {
                let _ = self.read_bytes(4)?;
            }
            other => return Err(format!("Unsupported protobuf wire type: {}", other)),
        }
        Ok(())
    }

    fn read_string(&mut self) -> Result<String, String> {
        let len = self.read_varint()? as usize;
        let bytes = self.read_bytes(len)?;
        String::from_utf8(bytes.to_vec())
            .map_err(|e| format!("Invalid UTF-8 string in protobuf: {}", e))
    }

    fn read_float32(&mut self) -> Result<f32, String> {
        let bytes = self.read_bytes(4)?;
        Ok(f32::from_le_bytes(bytes.try_into().unwrap()))
    }
}

// ----------------------------------------------------------------------------
// ONNX Message Parsers
// ----------------------------------------------------------------------------

pub fn parse_onnx_model(bytes: &[u8]) -> Result<OnnxModel, String> {
    let mut reader = ProtoReader::new(bytes);
    let mut ir_version = 0;
    let mut producer_name = String::new();
    let mut graph = None;

    while !reader.is_eof() {
        let (field_num, wire_type) = reader.read_tag()?;
        match (field_num, wire_type) {
            (1, 0) => {
                ir_version = reader.read_varint()? as i64;
            }
            (3, 2) => {
                producer_name = reader.read_string()?;
            }
            (7, 2) => {
                let len = reader.read_varint()? as usize;
                let g_bytes = reader.read_bytes(len)?;
                graph = Some(parse_onnx_graph(g_bytes)?);
            }
            (_, w) => reader.skip_field(w)?,
        }
    }

    let graph = graph.ok_or_else(|| "ONNX Model missing GraphProto (field 7)".to_string())?;

    Ok(OnnxModel {
        ir_version,
        producer_name,
        graph,
    })
}

fn parse_onnx_graph(bytes: &[u8]) -> Result<OnnxGraph, String> {
    let mut reader = ProtoReader::new(bytes);
    let mut name = String::new();
    let mut nodes = Vec::new();
    let mut initializers = HashMap::new();
    let mut inputs = Vec::new();
    let mut outputs = Vec::new();

    while !reader.is_eof() {
        let (field_num, wire_type) = reader.read_tag()?;
        match (field_num, wire_type) {
            (1, 2) => {
                // Repeated NodeProto
                let len = reader.read_varint()? as usize;
                let n_bytes = reader.read_bytes(len)?;
                nodes.push(parse_onnx_node(n_bytes)?);
            }
            (2, 2) => {
                name = reader.read_string()?;
            }
            (5, 2) => {
                // Repeated TensorProto (initializer)
                let len = reader.read_varint()? as usize;
                let t_bytes = reader.read_bytes(len)?;
                let tensor = parse_onnx_tensor(t_bytes)?;
                initializers.insert(tensor.name.clone(), tensor);
            }
            (11, 2) => {
                // Repeated ValueInfoProto (input)
                let len = reader.read_varint()? as usize;
                let v_bytes = reader.read_bytes(len)?;
                inputs.push(parse_onnx_value_info(v_bytes)?);
            }
            (12, 2) => {
                // Repeated ValueInfoProto (output)
                let len = reader.read_varint()? as usize;
                let v_bytes = reader.read_bytes(len)?;
                outputs.push(parse_onnx_value_info(v_bytes)?);
            }
            (_, w) => reader.skip_field(w)?,
        }
    }

    Ok(OnnxGraph {
        name,
        nodes,
        initializers,
        inputs,
        outputs,
    })
}

fn parse_onnx_node(bytes: &[u8]) -> Result<OnnxNode, String> {
    let mut reader = ProtoReader::new(bytes);
    let mut inputs = Vec::new();
    let mut outputs = Vec::new();
    let mut name = String::new();
    let mut op_type = String::new();
    let mut attributes = HashMap::new();

    while !reader.is_eof() {
        let (field_num, wire_type) = reader.read_tag()?;
        match (field_num, wire_type) {
            (1, 2) => {
                inputs.push(reader.read_string()?);
            }
            (2, 2) => {
                outputs.push(reader.read_string()?);
            }
            (3, 2) => {
                name = reader.read_string()?;
            }
            (4, 2) => {
                op_type = reader.read_string()?;
            }
            (5, 2) => {
                let len = reader.read_varint()? as usize;
                let a_bytes = reader.read_bytes(len)?;
                let attr = parse_onnx_attribute(a_bytes)?;
                attributes.insert(attr.name.clone(), attr);
            }
            (_, w) => reader.skip_field(w)?,
        }
    }

    Ok(OnnxNode {
        op_type,
        name,
        inputs,
        outputs,
        attributes,
    })
}

fn parse_onnx_attribute(bytes: &[u8]) -> Result<OnnxAttribute, String> {
    let mut reader = ProtoReader::new(bytes);
    let mut name = String::new();
    let mut f = None;
    let mut i = None;
    let mut s = None;
    let mut ints = Vec::new();
    let mut floats = Vec::new();

    while !reader.is_eof() {
        let (field_num, wire_type) = reader.read_tag()?;
        match (field_num, wire_type) {
            (1, 2) => {
                name = reader.read_string()?;
            }
            (2, 5) => {
                f = Some(reader.read_float32()?);
            }
            (3, 0) => {
                i = Some(reader.read_varint()? as i64);
            }
            (4, 2) => {
                s = Some(reader.read_string()?);
            }
            (7, 2) => {
                // Packed floats
                let len = reader.read_varint()? as usize;
                let mut sub = ProtoReader::new(reader.read_bytes(len)?);
                while !sub.is_eof() {
                    floats.push(sub.read_float32()?);
                }
            }
            (8, 2) => {
                // Packed ints
                let len = reader.read_varint()? as usize;
                let mut sub = ProtoReader::new(reader.read_bytes(len)?);
                while !sub.is_eof() {
                    ints.push(sub.read_varint()? as i64);
                }
            }
            (_, w) => reader.skip_field(w)?,
        }
    }

    Ok(OnnxAttribute {
        name,
        f,
        i,
        s,
        ints,
        floats,
    })
}

fn parse_onnx_tensor(bytes: &[u8]) -> Result<OnnxTensor, String> {
    let mut reader = ProtoReader::new(bytes);
    let mut dims = Vec::new();
    let mut data_type = 1;
    let mut float_values = Vec::new();
    let mut name = String::new();
    let mut raw_data = Vec::new();

    while !reader.is_eof() {
        let (field_num, wire_type) = reader.read_tag()?;
        match (field_num, wire_type) {
            (1, 0) => {
                dims.push(reader.read_varint()? as usize);
            }
            (1, 2) => {
                // Packed dims
                let len = reader.read_varint()? as usize;
                let mut sub = ProtoReader::new(reader.read_bytes(len)?);
                while !sub.is_eof() {
                    dims.push(sub.read_varint()? as usize);
                }
            }
            (2, 0) => {
                data_type = reader.read_varint()? as i32;
            }
            (4, 2) => {
                let len = reader.read_varint()? as usize;
                raw_data = reader.read_bytes(len)?.to_vec();
            }
            (7, 5) => {
                float_values.push(reader.read_float32()?);
            }
            (7, 2) => {
                // Packed floats
                let len = reader.read_varint()? as usize;
                let mut sub = ProtoReader::new(reader.read_bytes(len)?);
                while !sub.is_eof() {
                    float_values.push(sub.read_float32()?);
                }
            }
            (8, 2) => {
                name = reader.read_string()?;
            }
            (_, w) => reader.skip_field(w)?,
        }
    }

    // If float_values is empty but raw_data is populated and data_type is FLOAT (1)
    if float_values.is_empty() && !raw_data.is_empty() && data_type == 1 {
        let count = raw_data.len() / 4;
        for k in 0..count {
            let chunk = &raw_data[k * 4..(k + 1) * 4];
            float_values.push(f32::from_le_bytes(chunk.try_into().unwrap()));
        }
    }

    Ok(OnnxTensor {
        name,
        dims,
        data_type,
        float_values,
    })
}

fn parse_onnx_value_info(bytes: &[u8]) -> Result<OnnxValueInfo, String> {
    let mut reader = ProtoReader::new(bytes);
    let mut name = String::new();
    let mut dims = Vec::new();

    while !reader.is_eof() {
        let (field_num, wire_type) = reader.read_tag()?;
        match (field_num, wire_type) {
            (1, 2) => {
                name = reader.read_string()?;
            }
            (2, 2) => {
                // TypeProto: parse dimensions if present
                let len = reader.read_varint()? as usize;
                let type_bytes = reader.read_bytes(len)?;
                dims = parse_type_proto_dims(type_bytes);
            }
            (_, w) => reader.skip_field(w)?,
        }
    }

    Ok(OnnxValueInfo { name, dims })
}

fn parse_type_proto_dims(bytes: &[u8]) -> Vec<usize> {
    let mut reader = ProtoReader::new(bytes);
    let mut dims = Vec::new();

    while !reader.is_eof() {
        if let Ok((field_num, wire_type)) = reader.read_tag() {
            if field_num == 1 && wire_type == 2 {
                // TensorTypeProto
                let len = reader.read_varint().unwrap_or(0) as usize;
                if let Ok(t_bytes) = reader.read_bytes(len) {
                    dims = parse_tensor_shape(t_bytes);
                }
            } else {
                let _ = reader.skip_field(wire_type);
            }
        } else {
            break;
        }
    }
    dims
}

fn parse_tensor_shape(bytes: &[u8]) -> Vec<usize> {
    let mut reader = ProtoReader::new(bytes);
    let mut dims = Vec::new();

    while !reader.is_eof() {
        if let Ok((field_num, wire_type)) = reader.read_tag() {
            if field_num == 2 && wire_type == 2 {
                // TensorShapeProto
                let len = reader.read_varint().unwrap_or(0) as usize;
                if let Ok(s_bytes) = reader.read_bytes(len) {
                    dims = parse_shape_dims(s_bytes);
                }
            } else {
                let _ = reader.skip_field(wire_type);
            }
        } else {
            break;
        }
    }
    dims
}

fn parse_shape_dims(bytes: &[u8]) -> Vec<usize> {
    let mut reader = ProtoReader::new(bytes);
    let mut dims = Vec::new();

    while !reader.is_eof() {
        if let Ok((field_num, wire_type)) = reader.read_tag() {
            if field_num == 1 && wire_type == 2 {
                // Dimension
                let len = reader.read_varint().unwrap_or(0) as usize;
                if let Ok(d_bytes) = reader.read_bytes(len) {
                    let mut d_reader = ProtoReader::new(d_bytes);
                    while !d_reader.is_eof() {
                        if let Ok((df, dw)) = d_reader.read_tag() {
                            if df == 1 && dw == 0 {
                                if let Ok(val) = d_reader.read_varint() {
                                    dims.push(val as usize);
                                }
                            } else {
                                let _ = d_reader.skip_field(dw);
                            }
                        } else {
                            break;
                        }
                    }
                }
            } else {
                let _ = reader.skip_field(wire_type);
            }
        } else {
            break;
        }
    }
    dims
}

// ----------------------------------------------------------------------------
// Synthetic ONNX Model Serialization Helper for Unit Testing
// ----------------------------------------------------------------------------

pub fn create_synthetic_onnx_model(
    model_name: &str,
    nodes: &[(&str, &str, &[&str], &[&str])], // (op_type, node_name, inputs, outputs)
    initializers: &[(&str, &[usize], &[f32])], // (tensor_name, dims, values)
    inputs: &[(&str, &[usize])],
    outputs: &[(&str, &[usize])],
) -> Vec<u8> {
    let mut graph_buf = Vec::new();

    // 1. Nodes (field 1)
    for &(op_type, node_name, in_names, out_names) in nodes {
        let mut node_buf = Vec::new();
        // inputs (field 1)
        for &inp in in_names {
            encode_string_field(1, inp, &mut node_buf);
        }
        // outputs (field 2)
        for &outp in out_names {
            encode_string_field(2, outp, &mut node_buf);
        }
        // name (field 3)
        encode_string_field(3, node_name, &mut node_buf);
        // op_type (field 4)
        encode_string_field(4, op_type, &mut node_buf);

        encode_length_delimited(1, &node_buf, &mut graph_buf);
    }

    // 2. Name (field 2)
    encode_string_field(2, model_name, &mut graph_buf);

    // 3. Initializers (field 5)
    for &(t_name, dims, vals) in initializers {
        let mut t_buf = Vec::new();
        // dims (field 1 packed)
        let mut dims_buf = Vec::new();
        for &d in dims {
            encode_varint_raw(d as u64, &mut dims_buf);
        }
        encode_length_delimited(1, &dims_buf, &mut t_buf);
        // data_type (field 2: 1 = FLOAT)
        encode_varint_field(2, 1, &mut t_buf);
        // raw_data (field 4)
        let mut raw_bytes = Vec::new();
        for &v in vals {
            raw_bytes.extend_from_slice(&v.to_le_bytes());
        }
        encode_length_delimited(4, &raw_bytes, &mut t_buf);
        // name (field 8)
        encode_string_field(8, t_name, &mut t_buf);

        encode_length_delimited(5, &t_buf, &mut graph_buf);
    }

    // 4. Inputs (field 11)
    for &(inp_name, dims) in inputs {
        let mut vi_buf = Vec::new();
        encode_string_field(1, inp_name, &mut vi_buf);
        let type_proto = encode_type_proto(dims);
        encode_length_delimited(2, &type_proto, &mut vi_buf);
        encode_length_delimited(11, &vi_buf, &mut graph_buf);
    }

    // 5. Outputs (field 12)
    for &(out_name, dims) in outputs {
        let mut vi_buf = Vec::new();
        encode_string_field(1, out_name, &mut vi_buf);
        let type_proto = encode_type_proto(dims);
        encode_length_delimited(2, &type_proto, &mut vi_buf);
        encode_length_delimited(12, &vi_buf, &mut graph_buf);
    }

    // Wrap into ModelProto
    let mut model_buf = Vec::new();
    encode_varint_field(1, 8, &mut model_buf); // ir_version = 8
    encode_string_field(3, "CRON-Pure-Rust-ONNX-Synthesizer", &mut model_buf);
    encode_length_delimited(7, &graph_buf, &mut model_buf);

    model_buf
}

fn encode_type_proto(dims: &[usize]) -> Vec<u8> {
    let mut shape_buf = Vec::new();
    for &d in dims {
        let mut dim_buf = Vec::new();
        encode_varint_field(1, d as u64, &mut dim_buf);
        encode_length_delimited(1, &dim_buf, &mut shape_buf);
    }

    let mut tensor_type_buf = Vec::new();
    encode_varint_field(1, 1, &mut tensor_type_buf); // elem_type = FLOAT
    encode_length_delimited(2, &shape_buf, &mut tensor_type_buf);

    let mut type_buf = Vec::new();
    encode_length_delimited(1, &tensor_type_buf, &mut type_buf);
    type_buf
}

fn encode_varint_raw(mut val: u64, out: &mut Vec<u8>) {
    loop {
        let b = (val & 0x7F) as u8;
        val >>= 7;
        if val != 0 {
            out.push(b | 0x80);
        } else {
            out.push(b);
            break;
        }
    }
}

fn encode_varint_field(field_num: u32, val: u64, out: &mut Vec<u8>) {
    let tag = (field_num << 3) as u64;
    encode_varint_raw(tag, out);
    encode_varint_raw(val, out);
}

fn encode_length_delimited(field_num: u32, bytes: &[u8], out: &mut Vec<u8>) {
    let tag = ((field_num << 3) | 2) as u64;
    encode_varint_raw(tag, out);
    encode_varint_raw(bytes.len() as u64, out);
    out.extend_from_slice(bytes);
}

fn encode_string_field(field_num: u32, s: &str, out: &mut Vec<u8>) {
    encode_length_delimited(field_num, s.as_bytes(), out);
}
