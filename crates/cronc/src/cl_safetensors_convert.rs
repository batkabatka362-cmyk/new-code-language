// ============================================================================
// CRON Safetensors -> DPSTA 1.58-bit Ternary Converter (cl_safetensors_convert.rs)
//
// 100% Pure Rust - Zero Python, Zero PyTorch dependencies.
//
// Converts standard HuggingFace Safetensors Float32/BFloat16/Float16 checkpoints
// into CRON DPSTA 1.58-bit sub-byte packed binary format (.cldb / .dpdata)
// achieving ~19.9x RAM savings and zero-multiplication Popcount MAC execution.
// ============================================================================

use std::fs::File;
use std::io::Write;
use std::path::Path;

use crate::cl_dpsta::pack_ternary_slice;

/// Summary report of the Safetensors conversion process
#[derive(Debug, Clone)]
pub struct SafetensorsConversionSummary {
    pub input_file: String,
    pub output_cldb: String,
    pub output_cl: Option<String>,
    pub total_tensors: usize,
    pub total_elements: usize,
    pub original_bytes: usize,
    pub converted_bytes: usize,
    pub compression_ratio: f32,
    pub average_sparsity_pct: f32,
    pub tensor_names: Vec<String>,
}

impl SafetensorsConversionSummary {
    pub fn render_ascii_hud(&self) -> String {
        let mut hud = String::new();
        hud.push_str("════════════════════════════════════════════════════════════════════════════════════════\n");
        hud.push_str("          CRON SAFETENSORS -> DPSTA 1.58-BIT TERNARY SILICON CONVERTER                  \n");
        hud.push_str("════════════════════════════════════════════════════════════════════════════════════════\n");
        hud.push_str(&format!(" • Input Safetensors:  {}\n", self.input_file));
        hud.push_str(&format!(" • Output CLDB Binary: {}\n", self.output_cldb));
        if let Some(ref cl_path) = self.output_cl {
            hud.push_str(&format!(" • Output .CL Model:   {}\n", cl_path));
        }
        hud.push_str(&format!(" • Converted Tensors:  {} tensors ({} total parameters)\n", self.total_tensors, self.total_elements));
        hud.push_str(&format!(
            " • Memory Footprint:   {:.2} MB -> {:.2} MB ({:.1}x In-Memory RAM Savings)\n",
            self.original_bytes as f64 / 1_048_576.0,
            self.converted_bytes as f64 / 1_048_576.0,
            self.compression_ratio
        ));
        hud.push_str(&format!(" • Average Sparsity:   {:.2}% zero weights (Multiplier-Free Skip)\n", self.average_sparsity_pct));
        hud.push_str("────────────────────────────────────────────────────────────────────────────────────────\n");
        hud.push_str("  ✓ 100% In-Silicon Ready: Exported with zero Python/PyTorch runtime overhead.\n");
        hud.push_str("════════════════════════════════════════════════════════════════════════════════════════\n");
        hud
    }
}

/// Quantizes a float slice to 1.58-bit ternary {-1, 0, +1} and per-tensor scale gamma
pub fn quantize_to_ternary_158(data: &[f32]) -> (Vec<i8>, f32, f32) {
    if data.is_empty() {
        return (Vec::new(), 1.0, 0.0);
    }

    // 1. Compute scale gamma = mean(|W|)
    let sum_abs: f64 = data.iter().map(|&x| (x as f64).abs()).sum();
    let gamma = ((sum_abs / data.len() as f64) as f32).max(1e-7);

    // 2. Quantize: w_t = round(clamp(w / gamma, -1, 1))
    let mut ternary = Vec::with_capacity(data.len());
    let mut zero_count = 0usize;

    for &val in data {
        let scaled = val / gamma;
        let t = if scaled > 0.5 {
            1i8
        } else if scaled < -0.5 {
            -1i8
        } else {
            0i8
        };
        if t == 0 {
            zero_count += 1;
        }
        ternary.push(t);
    }

    let sparsity_pct = (zero_count as f32 / data.len() as f32) * 100.0;
    (ternary, gamma, sparsity_pct)
}

/// Parsed Raw Tensor from Safetensors
struct RawSafetensor {
    name: String,
    _dtype: String,
    shape: Vec<usize>,
    data: Vec<f32>,
}

/// Native zero-dependency Safetensors parser supporting F32, F16, and BF16
fn parse_raw_safetensors(bytes: &[u8]) -> Result<Vec<RawSafetensor>, String> {
    if bytes.len() < 8 {
        return Err("Safetensors file is too small (less than 8 bytes)".to_string());
    }

    let header_len = u64::from_le_bytes(bytes[0..8].try_into().unwrap()) as usize;
    if bytes.len() < 8 + header_len {
        return Err("Safetensors header length exceeds file size".to_string());
    }

    let header_bytes = &bytes[8..8 + header_len];
    let header_str = std::str::from_utf8(header_bytes)
        .map_err(|e| format!("Invalid UTF-8 in Safetensors header: {}", e))?;
    let payload = &bytes[8 + header_len..];

    let mut result = Vec::new();
    let mut idx = 0;

    while let Some(q_start) = header_str[idx..].find('"') {
        let key_start = idx + q_start + 1;
        if let Some(q_end) = header_str[key_start..].find('"') {
            let key = &header_str[key_start..key_start + q_end];
            idx = key_start + q_end + 1;

            if key == "__metadata__" || key.is_empty() {
                continue;
            }

            // Find matching object block: { ... }
            if let Some(obj_start) = header_str[idx..].find('{') {
                let actual_obj_start = idx + obj_start;
                let mut brace_count = 1;
                let mut obj_end = actual_obj_start + 1;
                for ch in header_str[actual_obj_start + 1..].chars() {
                    if ch == '{' {
                        brace_count += 1;
                    } else if ch == '}' {
                        brace_count -= 1;
                        if brace_count == 0 {
                            break;
                        }
                    }
                    obj_end += ch.len_utf8();
                }

                let obj_str = &header_str[actual_obj_start..=obj_end];
                idx = obj_end + 1;

                // Extract dtype
                let dtype = if let Some(dt_idx) = obj_str.find("\"dtype\"") {
                    let rest = &obj_str[dt_idx + 7..];
                    if let Some(c1) = rest.find('"') {
                        let c2 = rest[c1 + 1..].find('"').unwrap_or(0);
                        rest[c1 + 1..c1 + 1 + c2].to_string()
                    } else {
                        "F32".to_string()
                    }
                } else {
                    "F32".to_string()
                };

                // Extract shape
                let mut shape = Vec::new();
                if let Some(sh_idx) = obj_str.find("\"shape\"") {
                    let rest = &obj_str[sh_idx + 7..];
                    if let Some(b1) = rest.find('[') {
                        if let Some(b2) = rest[b1 + 1..].find(']') {
                            let list_str = &rest[b1 + 1..b1 + 1 + b2];
                            for part in list_str.split(',') {
                                let trimmed = part.trim();
                                if let Ok(val) = trimmed.parse::<usize>() {
                                    shape.push(val);
                                }
                            }
                        }
                    }
                }

                // Extract data_offsets: [start, end]
                let mut offsets = (0usize, 0usize);
                if let Some(off_idx) = obj_str.find("\"data_offsets\"") {
                    let rest = &obj_str[off_idx + 14..];
                    if let Some(b1) = rest.find('[') {
                        if let Some(b2) = rest[b1 + 1..].find(']') {
                            let list_str = &rest[b1 + 1..b1 + 1 + b2];
                            let parts: Vec<&str> = list_str.split(',').collect();
                            if parts.len() == 2 {
                                let s = parts[0].trim().parse::<usize>().unwrap_or(0);
                                let e = parts[1].trim().parse::<usize>().unwrap_or(0);
                                offsets = (s, e);
                            }
                        }
                    }
                }

                if offsets.1 > offsets.0 && offsets.1 <= payload.len() {
                    let tensor_bytes = &payload[offsets.0..offsets.1];
                    let mut floats = Vec::new();

                    match dtype.to_uppercase().as_str() {
                        "F32" => {
                            for chunk in tensor_bytes.chunks_exact(4) {
                                floats.push(f32::from_le_bytes(chunk.try_into().unwrap()));
                            }
                        }
                        "BF16" => {
                            for chunk in tensor_bytes.chunks_exact(2) {
                                let bits = u16::from_le_bytes(chunk.try_into().unwrap()) as u32;
                                floats.push(f32::from_bits(bits << 16));
                            }
                        }
                        "F16" => {
                            for chunk in tensor_bytes.chunks_exact(2) {
                                let bits = u16::from_le_bytes(chunk.try_into().unwrap());
                                // Float16 to Float32 conversion
                                let sign = ((bits >> 15) & 1) as u32;
                                let exp = ((bits >> 10) & 0x1F) as u32;
                                let mant = (bits & 0x3FF) as u32;
                                let f32_bits = if exp == 0 {
                                    sign << 31
                                } else if exp == 31 {
                                    (sign << 31) | (0xFF << 23) | (mant << 13)
                                } else {
                                    let new_exp = exp + 127 - 15;
                                    (sign << 31) | (new_exp << 23) | (mant << 13)
                                };
                                floats.push(f32::from_bits(f32_bits));
                            }
                        }
                        _ => {
                            // Default to F32
                            for chunk in tensor_bytes.chunks_exact(4) {
                                floats.push(f32::from_le_bytes(chunk.try_into().unwrap()));
                            }
                        }
                    }

                    result.push(RawSafetensor {
                        name: key.to_string(),
                        _dtype: dtype,
                        shape,
                        data: floats,
                    });
                }
            } else {
                break;
            }
        } else {
            break;
        }
    }

    Ok(result)
}

/// Converts a Safetensors checkpoint into DPSTA 1.58-bit sub-byte packed format and .cl model
pub fn convert_safetensors_to_dpsta(
    input_path: &str,
    output_cldb: &str,
    output_cl: Option<&str>,
    quantize_ternary: bool,
) -> Result<SafetensorsConversionSummary, String> {
    if !Path::new(input_path).exists() {
        return Err(format!("Input safetensors file '{}' does not exist", input_path));
    }

    let file_bytes = std::fs::read(input_path)
        .map_err(|e| format!("Failed to read safetensors file '{}': {}", input_path, e))?;

    let raw_tensors = parse_raw_safetensors(&file_bytes)?;
    if raw_tensors.is_empty() {
        return Err(format!("No tensors found in Safetensors file '{}'", input_path));
    }

    let mut total_elements = 0usize;
    let mut original_bytes = 0usize;
    let mut converted_bytes = 0usize;
    let mut total_sparsity_sum = 0.0f32;
    let mut tensor_names = Vec::new();

    // Prepare binary CLDB container:
    // [Magic: "CLDB\x01\x00"][NumTensors: u32]
    // For each tensor:
    //   [NameLen: u16][Name: UTF-8][Rank: u8][Shape: Vec<u64>][Scale: f32][Flags: u8 (0x01 = packed ternary)][DataLen: u64][Data: bytes]
    let mut cldb_bytes = Vec::new();
    cldb_bytes.extend_from_slice(b"CLDB\x01\x00");
    cldb_bytes.extend_from_slice(&(raw_tensors.len() as u32).to_le_bytes());

    for tensor in &raw_tensors {
        total_elements += tensor.data.len();
        original_bytes += tensor.data.len() * 4;
        tensor_names.push(tensor.name.clone());

        let name_bytes = tensor.name.as_bytes();
        cldb_bytes.extend_from_slice(&(name_bytes.len() as u16).to_le_bytes());
        cldb_bytes.extend_from_slice(name_bytes);
        cldb_bytes.push(tensor.shape.len() as u8);
        for &dim in &tensor.shape {
            cldb_bytes.extend_from_slice(&(dim as u64).to_le_bytes());
        }

        if quantize_ternary {
            let (ternary, gamma, sparsity) = quantize_to_ternary_158(&tensor.data);
            total_sparsity_sum += sparsity;
            let packed = pack_ternary_slice(&ternary);

            cldb_bytes.extend_from_slice(&gamma.to_le_bytes());
            cldb_bytes.push(0x01); // 0x01 = 5-in-1 packed ternary
            cldb_bytes.extend_from_slice(&(packed.len() as u64).to_le_bytes());
            cldb_bytes.extend_from_slice(&packed);
            converted_bytes += packed.len() + 4; // packed bytes + scale
        } else {
            cldb_bytes.extend_from_slice(&1.0f32.to_le_bytes());
            cldb_bytes.push(0x00); // 0x00 = raw float32
            let byte_len = tensor.data.len() * 4;
            cldb_bytes.extend_from_slice(&(byte_len as u64).to_le_bytes());
            for &f in &tensor.data {
                cldb_bytes.extend_from_slice(&f.to_le_bytes());
            }
            converted_bytes += byte_len;
        }
    }

    // Write CLDB binary file
    let mut out_file = File::create(output_cldb)
        .map_err(|e| format!("Failed to create output file '{}': {}", output_cldb, e))?;
    out_file
        .write_all(&cldb_bytes)
        .map_err(|e| format!("Failed to write to '{}': {}", output_cldb, e))?;

    let num_tensors = raw_tensors.len().max(1);
    let avg_sparsity = if quantize_ternary {
        total_sparsity_sum / num_tensors as f32
    } else {
        0.0
    };
    let ratio = if converted_bytes > 0 {
        original_bytes as f32 / converted_bytes as f32
    } else {
        1.0
    };

    // Optionally generate canonical Zero-Verbal .cl file
    if let Some(cl_path) = output_cl {
        let mut cl_code = String::new();
        cl_code.push_str("// ════════════════════════════════════════════════════════════════════════════\n");
        cl_code.push_str("// CRON Post-Human AI Architecture: Converted 1.58-Bit DPSTA Silicon Model\n");
        cl_code.push_str("// Level 1A: Zero-Verbal Mathematical Calculus\n");
        cl_code.push_str("// ════════════════════════════════════════════════════════════════════════════\n\n");

        // Block 1: Base weight bindings
        for name in &tensor_names {
            let binding_var = if name.contains("embed_tokens") {
                "E"
            } else if name.contains("q_proj") {
                "W_q"
            } else if name.contains("k_proj") {
                "W_k"
            } else if name.contains("v_proj") {
                "W_v"
            } else if name.contains("o_proj") {
                "W_o"
            } else if name.contains("gate_proj") {
                "W_g"
            } else if name.contains("up_proj") {
                "W_u"
            } else if name.contains("down_proj") {
                "W_d"
            } else if name.contains("lm_head") {
                "W_h"
            } else if name.contains("input_layernorm") {
                "γ_1"
            } else if name.contains("post_attention_layernorm") {
                "γ_2"
            } else if name.contains("norm.weight") {
                "γ_f"
            } else {
                "W"
            };
            cl_code.push_str(&format!("{:<4} = @\"{}:{}\"\n", binding_var, output_cldb, name));
        }

        cl_code.push_str("\n// ════════════════════════════════════════════════════════════════════════════\n");
        cl_code.push_str("// Functional Composition (Post-Human Calculus)\n");
        cl_code.push_str("// ════════════════════════════════════════════════════════════════════════════\n");
        cl_code.push_str("N(x, γ) = (x / √(μ(x^2) + 1e-5)) * γ\n");
        cl_code.push_str("S(x, W_g, W_u, W_d) = ((x @ W_g * σ(x @ W_g)) * (x @ W_u)) @ W_d\n");
        cl_code.push_str("F(x) = x + S(N(x, γ_1), W_g, W_u, W_d)\n\n");

        cl_code.push_str("// ════════════════════════════════════════════════════════════════════════════\n");
        cl_code.push_str("// Target Evaluation\n");
        cl_code.push_str("// ════════════════════════════════════════════════════════════════════════════\n");
        cl_code.push_str("t = [1.0, 2.0, 3.0, 4.0]\n");
        cl_code.push_str("ρ(N(F(E[t]), γ_f) @ W_h^T)\n");

        std::fs::write(cl_path, cl_code)
            .map_err(|e| format!("Failed to write .cl file '{}': {}", cl_path, e))?;
    }

    Ok(SafetensorsConversionSummary {
        input_file: input_path.to_string(),
        output_cldb: output_cldb.to_string(),
        output_cl: output_cl.map(|s| s.to_string()),
        total_tensors: tensor_names.len(),
        total_elements,
        original_bytes,
        converted_bytes,
        compression_ratio: ratio,
        average_sparsity_pct: avg_sparsity,
        tensor_names,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Helper to create a valid minimal Safetensors file in memory
    fn make_test_safetensors() -> Vec<u8> {
        let mut data = Vec::new();
        // 2 tensors: "w1" shape [2, 2] F32, "w2" shape [2] F32
        let t1_bytes = [
            0.5f32.to_le_bytes(),
            (-0.8f32).to_le_bytes(),
            0.0f32.to_le_bytes(),
            1.2f32.to_le_bytes(),
        ]
        .concat();
        let t2_bytes = [0.1f32.to_le_bytes(), (-0.2f32).to_le_bytes()].concat();

        let header = format!(
            r#"{{"w1":{{"dtype":"F32","shape":[2,2],"data_offsets":[0,{}]}},"w2":{{"dtype":"F32","shape":[2],"data_offsets":[{},{}]}}}}"#,
            t1_bytes.len(),
            t1_bytes.len(),
            t1_bytes.len() + t2_bytes.len()
        );

        let header_len = header.len() as u64;
        data.extend_from_slice(&header_len.to_le_bytes());
        data.extend_from_slice(header.as_bytes());
        data.extend_from_slice(&t1_bytes);
        data.extend_from_slice(&t2_bytes);
        data
    }

    #[test]
    fn test_quantize_to_ternary_158_properties() {
        let weights = vec![0.8, -0.9, 0.05, -0.02, 1.2, -1.1, 0.0, 0.1];
        let (ternary, gamma, sparsity) = quantize_to_ternary_158(&weights);
        assert_eq!(ternary.len(), weights.len());
        assert!(gamma > 0.0);
        assert!(sparsity >= 25.0);
        for &t in &ternary {
            assert!(t == -1 || t == 0 || t == 1);
        }
    }

    #[test]
    fn test_e2e_safetensors_conversion() {
        let temp_st = "temp_test_model.safetensors";
        let temp_cldb = "temp_test_model.cldb";
        let temp_cl = "temp_test_model.cl";

        let sf_bytes = make_test_safetensors();
        std::fs::write(temp_st, &sf_bytes).expect("Failed to write test safetensors");

        let summary = convert_safetensors_to_dpsta(temp_st, temp_cldb, Some(temp_cl), true)
            .expect("Conversion must succeed");

        assert_eq!(summary.total_tensors, 2);
        assert!(summary.compression_ratio > 1.0);
        assert!(Path::new(temp_cldb).exists());
        assert!(Path::new(temp_cl).exists());

        // Verify generated .cl is Zero-Verbal compliant
        let cl_content = std::fs::read_to_string(temp_cl).unwrap();
        let report = crate::cl_zero_verbal_lint::ZeroVerbalLinter::audit_source(&cl_content, temp_cl);
        assert!(report.is_constitution_compliant);

        // Cleanup
        let _ = std::fs::remove_file(temp_st);
        let _ = std::fs::remove_file(temp_cldb);
        let _ = std::fs::remove_file(temp_cl);
    }
}
