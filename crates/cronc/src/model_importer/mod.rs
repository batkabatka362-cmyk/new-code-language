// ============================================================================
// CRON Compiler: Universal AI Model Importer & Silicon Graph Lowering Engine
// Module: cronc::model_importer
// (C) 2026 CRON Language Project - SSS+ Tier Industrial Systems
// ============================================================================

pub mod gguf;
pub mod graph_lowering;
pub mod onnx;
pub mod quantize;
pub mod safetensors;
pub mod ternary;

pub use gguf::{create_synthetic_gguf, parse_gguf_header, GgufModel, GgufTensorInfo, GgufTensorType, GgufValue};
pub use graph_lowering::{lower_gguf_model, lower_onnx_graph, lower_safetensors_model, LoweredCode, LoweringOptions};
pub use onnx::{parse_onnx_model, OnnxGraph, OnnxModel, OnnxNode, OnnxTensor};
pub use quantize::{
    quantize_int4, quantize_ternary_158b, QuantizationMode, QuantizationReport, QuantizedTensor,
};
pub use safetensors::{
    f16_to_f32, parse_safetensors, parse_safetensors_json, SafeTensorDType, SafeTensorData, SafeTensorMeta, SafeTensorsModel,
};
pub use ternary::{
    pack_ternary_2bit, quantize_to_ternary, ternary_dot_product, ternary_gemv, unpack_ternary_2bit,
    TernaryQuantParams,
};

use std::fs;
use std::path::Path;

#[derive(Debug, Clone)]
pub struct ImportOptions {
    pub output_path: Option<String>,
    pub quantize_mode: QuantizationMode,
    pub tile_size: (usize, usize),
    pub enable_kernel_fusion: bool,
    pub target_cores: usize,
}

impl Default for ImportOptions {
    fn default() -> Self {
        Self {
            output_path: None,
            quantize_mode: QuantizationMode::Ternary158b,
            tile_size: (4, 4),
            enable_kernel_fusion: true,
            target_cores: 256,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ImportedModel {
    pub source_format: String, // "ONNX", "SafeTensors", or "GGUF"
    pub model_name: String,
    pub lowered_code: LoweredCode,
}

/// Import an ONNX, SafeTensors, or GGUF model file directly from disk and lower it to CRON (.cr).
pub fn import_model_file<P: AsRef<Path>>(
    file_path: P,
    options: &ImportOptions,
) -> Result<ImportedModel, String> {
    let path = file_path.as_ref();
    let bytes = fs::read(path)
        .map_err(|e| format!("Failed to read model file '{}': {}", path.display(), e))?;

    let file_name = path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("imported_model");

    let ext = path
        .extension()
        .and_then(|s| s.to_str())
        .unwrap_or("")
        .to_lowercase();

    let lowering_opts = LoweringOptions {
        module_name: format!("Imported_{}", file_name),
        quantize_mode: options.quantize_mode,
        tile_size: options.tile_size,
        enable_kernel_fusion: options.enable_kernel_fusion,
        target_cores: options.target_cores,
    };

    if ext == "onnx" || ext == "pb" {
        let onnx_model = parse_onnx_model(&bytes)?;
        let lowered = lower_onnx_graph(&onnx_model.graph, &lowering_opts)?;
        Ok(ImportedModel {
            source_format: "ONNX".to_string(),
            model_name: onnx_model.graph.name,
            lowered_code: lowered,
        })
    } else if ext == "safetensors" || ext == "bin" {
        let st_model = parse_safetensors(&bytes)?;
        let lowered = lower_safetensors_model(&st_model, &lowering_opts)?;
        Ok(ImportedModel {
            source_format: "SafeTensors".to_string(),
            model_name: file_name.to_string(),
            lowered_code: lowered,
        })
    } else if ext == "gguf" {
        let gguf_model = parse_gguf_header(&bytes)?;
        let lowered = lower_gguf_model(&gguf_model, &lowering_opts)?;
        Ok(ImportedModel {
            source_format: "GGUF".to_string(),
            model_name: file_name.to_string(),
            lowered_code: lowered,
        })
    } else {
        // Try parsing as SafeTensors first, then GGUF, then ONNX
        if let Ok(st_model) = parse_safetensors(&bytes) {
            let lowered = lower_safetensors_model(&st_model, &lowering_opts)?;
            return Ok(ImportedModel {
                source_format: "SafeTensors".to_string(),
                model_name: file_name.to_string(),
                lowered_code: lowered,
            });
        }
        if let Ok(gguf_model) = parse_gguf_header(&bytes) {
            let lowered = lower_gguf_model(&gguf_model, &lowering_opts)?;
            return Ok(ImportedModel {
                source_format: "GGUF".to_string(),
                model_name: file_name.to_string(),
                lowered_code: lowered,
            });
        }
        if let Ok(onnx_model) = parse_onnx_model(&bytes) {
            let lowered = lower_onnx_graph(&onnx_model.graph, &lowering_opts)?;
            return Ok(ImportedModel {
                source_format: "ONNX".to_string(),
                model_name: onnx_model.graph.name,
                lowered_code: lowered,
            });
        }
        Err(format!(
            "Unrecognized model format for '{}'. Expected .onnx, .safetensors, or .gguf",
            path.display()
        ))
    }
}
