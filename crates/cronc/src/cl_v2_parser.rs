// ============================================================================
// CRON CL v2.0 SOVEREIGN 5-PILLAR PARSER
// Module: cronc::cl_v2_parser (crates/cronc/src/cl_v2_parser.rs)
//
// Parses the 5 Pillars of the Post-Human AI-Silicon Language:
// [1] Silicon Target Contract:  @kernel, .target, .ipc_target, .cores, @mesh
// [2] I/O Memory Binding:       @io { in, out, wt @sram/@dram/@hbm }
// [3] Mathematical Morphism:    §Block[Dims] { States, X ↦ Equation }
// [4] Ciphered Control-Flow:    §ControlFlow { ⟳, ? ⮞ :, ←, !Δ watchdog }
// [5] 10/16-Slot VLIW Cipher:   B0000: '==... _MD... _DV... _TG...
// ============================================================================

use crate::cl_v2_ast::*;

pub fn parse_cl_v2(input: &str) -> Result<ClV2Document, String> {
    let mut doc = ClV2Document::new();
    let lines: Vec<&str> = input.lines().collect();
    let mut i = 0;

    while i < lines.len() {
        let line = lines[i].trim();

        // Skip comments and blank lines
        if line.is_empty() || line.starts_with("//") || (line.starts_with(';') && !line.starts_with("; B")) {
            i += 1;
            continue;
        }

        // [1] Target Contract: @kernel ...
        // [1] Target Contract: @kernel ... or @⌖ ...
        if line.starts_with("@kernel") || line.starts_with("@⌖") {
            let (target, next_i) = parse_target_contract(&lines, i)?;
            doc.items.push(ProgramItem::TargetContract(target.clone()));
            doc.target_contract = Some(target);
            i = next_i;
            continue;
        }

        // [2] I/O Memory Binding: @io { ... }
        if line.starts_with("@io") {
            let (io, next_i) = parse_io_binding(&lines, i)?;
            doc.items.push(ProgramItem::IoBinding(io.clone()));
            doc.io_binding = Some(io);
            i = next_i;
            continue;
        }

        // [3] Function / Mathematical Tensor Morphism Block: §Name...
        if line.starts_with('§') && !line.contains("§ControlFlow") {
            let (func, next_i) = parse_function_block(&lines, i)?;
            doc.items.push(ProgramItem::Function(func.clone()));
            if doc.math_morphism.is_none() {
                doc.math_morphism = Some(MathMorphismBlock {
                    name: func.name.clone(),
                    tile_dims: if func.tile_dims.is_empty() { vec![16, 16] } else { func.tile_dims.clone() },
                    dtype: func.dtype.clone(),
                    states: func.states.clone(),
                    input_symbol: func.input_symbol.clone(),
                    output_symbol: func.output_symbol.clone(),
                    equation_expr: func.equation_expr.clone(),
                    steps: func.steps.clone(),
                });
            }
            doc.functions.push(func);
            i = next_i;
            continue;
        }

        // [4] Explicit Control-Flow Block: §ControlFlow
        if line.contains("§ControlFlow") {
            let (control, next_i) = parse_control_flow(&lines, i)?;
            doc.items.push(ProgramItem::ControlFlow(control.clone()));
            doc.control_flow = Some(control);
            i = next_i;
            continue;
        }

        // [4b] Standalone Watchdog: !Δ or !delta
        if line.starts_with("!Δ") || line.starts_with("!delta") {
            let watchdog = parse_watchdog_line(line)?;
            doc.items.push(ProgramItem::Watchdog(watchdog.clone()));
            doc.watchdog = Some(watchdog.clone());
            if let Some(ref mut cf) = doc.control_flow {
                cf.watchdog = Some(watchdog);
            } else {
                doc.control_flow = Some(CipherControlFlow {
                    statements: Vec::new(),
                    watchdog: Some(watchdog),
                });
            }
            i += 1;
            continue;
        }

        // [5] VLIW Microcode Bundle: B0000: ... or W000: B0000: ... or ; B0000: ...
        let line_bundle = if line.starts_with("; B") {
            line[2..].trim()
        } else if let Some(b_idx) = line.find('B') {
            if b_idx > 0 && line[..b_idx].trim_end().ends_with(':') {
                line[b_idx..].trim()
            } else {
                line
            }
        } else {
            line
        };

        if line_bundle.starts_with('B') && line_bundle.find(':').map_or(false, |pos| pos >= 2 && pos <= 6) {
            let bundle = parse_vliw_bundle(line_bundle)?;
            doc.items.push(ProgramItem::VliwBundle(bundle.clone()));
            doc.vliw_stream.push(bundle);
            i += 1;
            continue;
        }

        // Direct target contract directives outside explicit @kernel line
        if line.starts_with(".target") || line.starts_with(".ipc_target") || line.starts_with(".cores") || line.starts_with("@mesh") {
            if doc.target_contract.is_none() {
                doc.target_contract = Some(SiliconTargetContract {
                    kernel_name: "sovereign_kernel".to_string(),
                    target: "silicon.generic".to_string(),
                    ipc_target: 4.0,
                    cores: 256,
                    mesh_routing: None,
                });
            }
            if let Some(ref mut contract) = doc.target_contract {
                if line.starts_with(".target") {
                    contract.target = line.trim_start_matches(".target").trim().to_string();
                } else if line.starts_with(".ipc_target") {
                    contract.ipc_target = line.trim_start_matches(".ipc_target").trim().parse().unwrap_or(4.0);
                } else if line.starts_with(".cores") {
                    contract.cores = line.trim_start_matches(".cores").trim().parse().unwrap_or(256);
                } else if line.starts_with("@mesh") {
                    contract.mesh_routing = Some(parse_mesh_line(line)?);
                }
            }
            i += 1;
            continue;
        }

        // [6] Free-form Top-Level Statements:
        // Top-level Loop: ⟳ (cond) { ... }
        if line.starts_with('⟳') || line.starts_with("@loop") {
            let (loop_stmt, next_i) = parse_loop_stmt(&lines, i)?;
            doc.items.push(ProgramItem::Statement(loop_stmt.clone()));
            doc.top_level_statements.push(loop_stmt);
            i = next_i;
            continue;
        }

        // Top-level Conditional: ? (cond) ⮞ { ... } : { ... }
        if line.starts_with('?') {
            let (if_stmt, next_i) = parse_if_stmt(&lines, i)?;
            doc.items.push(ProgramItem::Statement(if_stmt.clone()));
            doc.top_level_statements.push(if_stmt);
            i = next_i;
            continue;
        }

        // Top-level Memory transfer: #R1 ← @dmem[0x1000]
        if line.contains('←') || line.contains("<-") {
            let delim = if line.contains('←') { "←" } else { "<-" };
            let parts: Vec<&str> = line.split(delim).collect();
            if parts.len() == 2 {
                let stmt = CipherStmt::MemoryTransfer {
                    dest: parts[0].trim().to_string(),
                    src: parts[1].trim().to_string(),
                };
                doc.items.push(ProgramItem::Statement(stmt.clone()));
                doc.top_level_statements.push(stmt);
            }
            i += 1;
            continue;
        }

        // Top-level Assignment: x := expr or #R1 := expr
        if line.contains(":=") {
            let parts: Vec<&str> = line.split(":=").collect();
            if parts.len() == 2 {
                let stmt = CipherStmt::Assign {
                    dest: parts[0].trim().to_string(),
                    expr: parts[1].trim().to_string(),
                };
                doc.items.push(ProgramItem::Statement(stmt.clone()));
                doc.top_level_statements.push(stmt);
            }
            i += 1;
            continue;
        }

        // Top-level Return: ⮌ expr
        if line.starts_with('⮌') || line.starts_with("return") {
            let ret_str = if line.starts_with('⮌') {
                line.trim_start_matches('⮌').trim()
            } else {
                line.trim_start_matches("return").trim()
            };
            let stmt = if ret_str.is_empty() {
                CipherStmt::Exit
            } else {
                CipherStmt::Return(Some(ret_str.to_string()))
            };
            doc.items.push(ProgramItem::Statement(stmt.clone()));
            doc.top_level_statements.push(stmt);
            i += 1;
            continue;
        }

        // Unrecognized or metadata line, advance
        i += 1;
    }

    if doc.control_flow.is_none() && !doc.top_level_statements.is_empty() {
        doc.control_flow = Some(CipherControlFlow {
            statements: doc.top_level_statements.clone(),
            watchdog: doc.watchdog.clone(),
        });
    }

    Ok(doc)
}

// ----------------------------------------------------------------------------
// [1] Target Contract Parser
// ----------------------------------------------------------------------------
fn parse_target_contract(lines: &[&str], mut i: usize) -> Result<(SiliconTargetContract, usize), String> {
    let first = lines[i].trim();
    let mut kernel_name = if first.starts_with("@kernel") {
        first.trim_start_matches("@kernel").trim().to_string()
    } else {
        "sovereign_kernel".to_string()
    };
    if kernel_name.is_empty() {
        kernel_name = "sovereign_kernel".to_string();
    }
    let mut target = "silicon.generic".to_string();
    let mut ipc_target = 4.0f32;
    let mut cores = 256usize;
    let mut mesh_routing = None;

    if first.starts_with("@⌖") || first.starts_with("@mesh") {
        mesh_routing = Some(parse_mesh_line(first)?);
    }

    i += 1;
    while i < lines.len() {
        let line = lines[i].trim();
        if line.starts_with(".target") {
            target = line.trim_start_matches(".target").trim().to_string();
        } else if line.starts_with(".ipc_target") {
            ipc_target = line.trim_start_matches(".ipc_target").trim().parse().unwrap_or(4.0);
        } else if line.starts_with(".cores") {
            cores = line.trim_start_matches(".cores").trim().parse().unwrap_or(256);
        } else if line.starts_with("@mesh") || line.starts_with("@⌖") {
            mesh_routing = Some(parse_mesh_line(line)?);
        } else if line.is_empty() || line.starts_with("//") || line.starts_with(';') {
            // Keep going if comment or blank
        } else {
            // Reached next section
            break;
        }
        i += 1;
    }

    Ok((
        SiliconTargetContract {
            kernel_name,
            target,
            ipc_target,
            cores,
            mesh_routing,
        },
        i,
    ))
}

fn parse_mesh_line(line: &str) -> Result<MeshRouting, String> {
    // Format: @mesh [4x4x4x4] : ⮑ (x+1, y, z, w) or @⌖ [4×4×4×4] : ⮑ (x+1, y, z, w)
    let parts: Vec<&str> = line.split(':').collect();
    if parts.is_empty() {
        return Err("Invalid @mesh or @⌖ line".to_string());
    }
    let dims_part = parts[0]
        .trim_start_matches("@mesh")
        .trim_start_matches("@⌖")
        .trim();
    let inner_dims = dims_part.trim_matches(|c| c == '[' || c == ']');
    let dims: Vec<usize> = inner_dims
        .split(|c| c == 'x' || c == '×' || c == ',')
        .filter_map(|s| s.trim().parse().ok())
        .collect();

    let formula = if parts.len() > 1 {
        parts[1]
            .trim()
            .trim_start_matches(|c| c == '⮑' || c == '-' || c == '>')
            .trim()
            .to_string()
    } else {
        "(x+1, y, z, w)".to_string()
    };

    Ok(MeshRouting {
        dimensions: dims,
        vector_formula: formula,
    })
}

// ----------------------------------------------------------------------------
// [2] I/O Binding Parser
// ----------------------------------------------------------------------------
fn parse_io_binding(lines: &[&str], mut i: usize) -> Result<(IoMemoryBinding, usize), String> {
    let mut items = Vec::new();
    let mut block_text = String::new();

    // Collect entire @io { ... } block
    while i < lines.len() {
        let line = lines[i].trim();
        block_text.push_str(line);
        block_text.push('\n');
        if line.contains('}') {
            i += 1;
            break;
        }
        i += 1;
    }

    let inner = match (block_text.find('{'), block_text.rfind('}')) {
        (Some(start), Some(end)) if start < end => &block_text[start + 1..end],
        _ => return Err("Malformed @io block, missing braces".to_string()),
    };

    for raw_item in inner.lines() {
        let item_line = raw_item.trim();
        if item_line.is_empty() || item_line.starts_with("//") || item_line.starts_with(';') {
            continue;
        }

        // Support both:
        // 1. in: X[16x16:NVFP4] @sram[0x0000]
        // 2. Q[16×16:FP32] @0x0000 (Pure non-verbal)
        let (kind, rest) = if let Some(colon_idx) = item_line.find(':') {
            let b_open = item_line.find('[');
            let b_close = item_line.find(']');
            let is_inside_brackets = match (b_open, b_close) {
                (Some(o), Some(c)) => colon_idx > o && colon_idx < c,
                _ => false,
            };
            if !is_inside_brackets {
                let kind_str = item_line[..colon_idx].trim();
                let k = match kind_str {
                    "in" => IoKind::Input,
                    "out" => IoKind::Output,
                    "wt" | "weight" => IoKind::Weight,
                    "state" => IoKind::State,
                    _ => IoKind::Input,
                };
                (k, item_line[colon_idx + 1..].trim())
            } else {
                (IoKind::Input, item_line)
            }
        } else {
            (IoKind::Input, item_line)
        };

        // Split name/type from memory address (delimited by '@')
        let at_idx = rest.find('@').ok_or_else(|| format!("Expected '@' memory tier in io item: {}", item_line))?;
        let tensor_part = rest[..at_idx].trim();
        let mem_part = rest[at_idx + 1..].trim();

        // Parse tensor_part: X[16x16:NVFP4]
        let bracket_open = tensor_part.find('[').ok_or_else(|| "Missing '[' in tensor shape".to_string())?;
        let bracket_close = tensor_part.find(']').ok_or_else(|| "Missing ']' in tensor shape".to_string())?;
        let name = tensor_part[..bracket_open].trim().to_string();
        let shape_type_str = &tensor_part[bracket_open + 1..bracket_close];

        let (shape_str, dtype) = if let Some(c_idx) = shape_type_str.find(':') {
            (&shape_type_str[..c_idx], shape_type_str[c_idx + 1..].trim().to_string())
        } else {
            (shape_type_str, "FP32".to_string())
        };

        let shape: Vec<usize> = shape_str
            .split(|c| c == 'x' || c == '×' || c == ',')
            .filter_map(|s| s.trim().parse().ok())
            .collect();

        // Parse mem_part: sram[0x0000] or 0x0000
        let (tier, addr) = parse_mem_address(mem_part)?;

        items.push(IoBindingItem {
            kind,
            name,
            shape,
            dtype,
            tier,
            address: addr,
        });
    }

    Ok((IoMemoryBinding { items }, i))
}

fn parse_mem_address(s: &str) -> Result<(MemoryTier, u64), String> {
    let s_clean = s.trim();
    if s_clean.starts_with("0x") || s_clean.starts_with("0X") {
        let addr = u64::from_str_radix(&s_clean[2..], 16).unwrap_or(0);
        return Ok((MemoryTier::Sram, addr));
    }
    let bracket_open = s.find('[').unwrap_or(0);
    let tier_str = s[..bracket_open].trim().to_lowercase();
    let tier = match tier_str.as_str() {
        "sram" => MemoryTier::Sram,
        "dram" => MemoryTier::Dram,
        "hbm" => MemoryTier::Hbm,
        _ => MemoryTier::Dmem,
    };

    let inner = s
        .trim_matches(|c| c == '[' || c == ']')
        .trim_start_matches(|c: char| c.is_alphabetic())
        .trim_matches(|c| c == '[' || c == ']');

    let addr = if inner.starts_with("0x") || inner.starts_with("0X") {
        u64::from_str_radix(&inner[2..], 16).unwrap_or(0)
    } else {
        inner.parse().unwrap_or(0)
    };

    Ok((tier, addr))
}

// ----------------------------------------------------------------------------
// [3] Universal Function & Math Morphism Parser
// ----------------------------------------------------------------------------
fn parse_function_block(lines: &[&str], mut i: usize) -> Result<(FunctionBlock, usize), String> {
    let header = lines[i].trim();
    // Header format: §Name[Dims:DType](params) {  or  §Name(params) {  or  §Name {
    let after_section = header.trim_start_matches('§').trim_start();
    let name_end = after_section.find(|c| c == '[' || c == '(' || c == '{' || c == ' ' || c == '\t').unwrap_or(after_section.len());
    let mut name = after_section[..name_end].trim().to_string();
    if name.is_empty() {
        name = "Block".to_string();
    }

    let mut tile_dims = Vec::new();
    let mut dtype = "FP32".to_string();
    let mut params = Vec::new();

    if let (Some(b_open), Some(b_close)) = (after_section.find('['), after_section.find(']')) {
        let inside = &after_section[b_open + 1..b_close];
        if let Some(c_idx) = inside.find(':') {
            dtype = inside[c_idx + 1..].trim().to_string();
            tile_dims = inside[..c_idx]
                .split(|c| c == 'x' || c == '×' || c == ',')
                .filter_map(|s| s.trim().parse().ok())
                .collect();
        } else {
            tile_dims = inside
                .split(|c| c == 'x' || c == '×' || c == ',')
                .filter_map(|s| s.trim().parse().ok())
                .collect();
        }
    }

    if let (Some(p_open), Some(p_close)) = (after_section.find('('), after_section.find(')')) {
        let inside = &after_section[p_open + 1..p_close];
        params = inside.split(',').map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).collect();
    }

    let mut states = Vec::new();
    let mut input_symbol = if !params.is_empty() { params[0].clone() } else { "X".to_string() };
    let mut output_symbol = None;
    let mut equation_expr = String::new();
    let mut steps = Vec::new();
    let mut body = Vec::new();

    i += 1;
    while i < lines.len() {
        let line = lines[i].trim();
        if line.starts_with('}') {
            i += 1;
            break;
        }

        if line.is_empty() || line.starts_with("//") || line.starts_with(';') {
            i += 1;
            continue;
        }

        // Loop inside function: ⟳ (cond) { ... }
        if line.starts_with('⟳') || line.starts_with("@loop") {
            let (loop_stmt, next_i) = parse_loop_stmt(lines, i)?;
            body.push(loop_stmt);
            i = next_i;
            continue;
        }

        // Conditional inside function: ? (cond) ⮞ { ... } : { ... }
        if line.starts_with('?') {
            let (if_stmt, next_i) = parse_if_stmt(lines, i)?;
            body.push(if_stmt);
            i = next_i;
            continue;
        }

        // Memory transfer: #R1 ← @dmem[0x1000]
        if line.contains('←') || line.contains("<-") {
            let delim = if line.contains('←') { "←" } else { "<-" };
            let parts: Vec<&str> = line.split(delim).collect();
            if parts.len() == 2 {
                body.push(CipherStmt::MemoryTransfer {
                    dest: parts[0].trim().to_string(),
                    src: parts[1].trim().to_string(),
                });
            }
            i += 1;
            continue;
        }

        // State declarations: H: SS[64x256], W: T[16x16:NVFP4]
        if line.contains(':') && !line.contains(":=") && !line.contains('↦') && !line.contains("->") && !line.contains('?') {
            let decls = line.split(',');
            for d in decls {
                if let Some((s_name, s_tag)) = d.split_once(':') {
                    states.push(MathStateDecl {
                        name: s_name.trim().to_string(),
                        type_tag: s_tag.trim().to_string(),
                        dims: vec![16, 16],
                        subtype: None,
                    });
                }
            }
            i += 1;
            continue;
        }

        // Direct lambda mapping: X ↦ silu(...)
        if line.contains('↦') || line.contains("->") {
            let delim = if line.contains('↦') { "↦" } else { "->" };
            let parts: Vec<&str> = line.split(delim).collect();
            if parts.len() >= 2 {
                input_symbol = parts[0].trim().to_string();
                let rhs = parts[1].trim();
                if rhs.ends_with(':') {
                    output_symbol = Some(rhs.trim_end_matches(':').trim().to_string());
                } else {
                    equation_expr = rhs.to_string();
                }
            }
            i += 1;
            continue;
        }

        // Assignment: N := rmsnorm(X) or dest := expr
        if line.contains(":=") {
            let parts: Vec<&str> = line.split(":=").collect();
            if parts.len() == 2 {
                let dest = parts[0].trim().to_string();
                let expr = parts[1].trim().to_string();
                let expr_ast = parse_math_expr_str(&expr).ok();
                steps.push(MathStep {
                    target_var: dest.clone(),
                    expr: expr.clone(),
                    expr_ast,
                });
                body.push(CipherStmt::Assign { dest, expr });
            }
            i += 1;
            continue;
        }

        // Return / Exit: ⮌ expr
        if line.starts_with('⮌') || line.starts_with("return") {
            let ret_str = if line.starts_with('⮌') {
                line.trim_start_matches('⮌').trim()
            } else {
                line.trim_start_matches("return").trim()
            };
            if ret_str.is_empty() {
                body.push(CipherStmt::Exit);
            } else {
                body.push(CipherStmt::Return(Some(ret_str.to_string())));
            }
            i += 1;
            continue;
        }

        // Standalone expression or call
        body.push(CipherStmt::Expr(line.to_string()));
        i += 1;
    }

    Ok((
        FunctionBlock {
            name,
            tile_dims,
            dtype,
            params,
            states,
            input_symbol,
            output_symbol,
            equation_expr,
            steps,
            body,
        },
        i,
    ))
}

// ----------------------------------------------------------------------------
// [4] Ciphered Control-Flow Parser
// ----------------------------------------------------------------------------
fn parse_control_flow(lines: &[&str], mut i: usize) -> Result<(CipherControlFlow, usize), String> {
    let mut stmts = Vec::new();
    let mut watchdog = None;

    i += 1;
    while i < lines.len() {
        let line = lines[i].trim();
        if line.starts_with('}') {
            i += 1;
            break;
        }

        if line.is_empty() || line.starts_with("//") || line.starts_with(';') {
            i += 1;
            continue;
        }

        // Loop: ⟳ (100) { ... }
        if line.starts_with('⟳') || line.starts_with("@loop") {
            let (loop_stmt, next_i) = parse_loop_stmt(lines, i)?;
            stmts.push(loop_stmt);
            i = next_i;
            continue;
        }

        // Conditional: ? (cond) ⮞ { ... } : { ... }
        if line.starts_with('?') {
            let (if_stmt, next_i) = parse_if_stmt(lines, i)?;
            stmts.push(if_stmt);
            i = next_i;
            continue;
        }

        // Memory transfer: #R1 ← @dmem[0x1000]
        if line.contains('←') || line.contains("<-") {
            let delim = if line.contains('←') { "←" } else { "<-" };
            let parts: Vec<&str> = line.split(delim).collect();
            if parts.len() == 2 {
                stmts.push(CipherStmt::MemoryTransfer {
                    dest: parts[0].trim().to_string(),
                    src: parts[1].trim().to_string(),
                });
            }
            i += 1;
            continue;
        }

        // Assignment: #R2 := #R1 * 0.95
        if line.contains(":=") {
            let parts: Vec<&str> = line.split(":=").collect();
            if parts.len() == 2 {
                stmts.push(CipherStmt::Assign {
                    dest: parts[0].trim().to_string(),
                    expr: parts[1].trim().to_string(),
                });
            }
            i += 1;
            continue;
        }

        // Break: ⮌
        if line.contains('⮌') || line.starts_with("break") {
            stmts.push(CipherStmt::Exit);
            i += 1;
            continue;
        }

        // Continue: ⮍
        if line.contains('⮍') || line.starts_with("continue") {
            stmts.push(CipherStmt::Next);
            i += 1;
            continue;
        }

        // Watchdog: !Δ { ... }
        if line.starts_with("!Δ") || line.starts_with("!delta") {
            watchdog = Some(parse_watchdog_line(line)?);
            i += 1;
            continue;
        }

        i += 1;
    }

    Ok((
        CipherControlFlow {
            statements: stmts,
            watchdog,
        },
        i,
    ))
}

fn parse_loop_stmt(lines: &[&str], mut i: usize) -> Result<(CipherStmt, usize), String> {
    let header = lines[i].trim();
    let cond = match (header.find('('), header.find(')')) {
        (Some(s), Some(e)) if s < e => header[s + 1..e].trim().to_string(),
        _ => "true".to_string(),
    };

    let mut body = Vec::new();
    i += 1;
    while i < lines.len() {
        let line = lines[i].trim();
        if line.starts_with('}') {
            i += 1;
            break;
        }
        if line.contains('⮌') {
            body.push(CipherStmt::Exit);
        } else if line.contains('←') || line.contains("<-") {
            let delim = if line.contains('←') { "←" } else { "<-" };
            let parts: Vec<&str> = line.split(delim).collect();
            if parts.len() == 2 {
                body.push(CipherStmt::MemoryTransfer {
                    dest: parts[0].trim().to_string(),
                    src: parts[1].trim().to_string(),
                });
            }
        } else if line.contains(":=") {
            let parts: Vec<&str> = line.split(":=").collect();
            if parts.len() == 2 {
                body.push(CipherStmt::Assign {
                    dest: parts[0].trim().to_string(),
                    expr: parts[1].trim().to_string(),
                });
            }
        }
        i += 1;
    }

    Ok((CipherStmt::Loop { condition: cond, body }, i))
}

fn parse_if_stmt(lines: &[&str], mut i: usize) -> Result<(CipherStmt, usize), String> {
    let header = lines[i].trim();
    let cond = match (header.find('('), header.find(')')) {
        (Some(s), Some(e)) if s < e => header[s + 1..e].trim().to_string(),
        _ => "true".to_string(),
    };

    let mut then_branch = Vec::new();
    let mut else_branch = Vec::new();
    let mut in_else = false;

    i += 1;
    while i < lines.len() {
        let line = lines[i].trim();
        if line.starts_with('}') && !line.contains(':') {
            i += 1;
            break;
        }
        if line.contains(':') && line.contains('{') {
            in_else = true;
            i += 1;
            continue;
        }
        if line.contains('⮌') {
            if in_else { else_branch.push(CipherStmt::Exit); } else { then_branch.push(CipherStmt::Exit); }
        } else if line.contains(":=") {
            let parts: Vec<&str> = line.split(":=").collect();
            if parts.len() == 2 {
                let stmt = CipherStmt::Assign {
                    dest: parts[0].trim().to_string(),
                    expr: parts[1].trim().to_string(),
                };
                if in_else { else_branch.push(stmt); } else { then_branch.push(stmt); }
            }
        } else if line.contains('←') || line.contains("<-") {
            let delim = if line.contains('←') { "←" } else { "<-" };
            let parts: Vec<&str> = line.split(delim).collect();
            if parts.len() == 2 {
                let stmt = CipherStmt::MemoryTransfer {
                    dest: parts[0].trim().to_string(),
                    src: parts[1].trim().to_string(),
                };
                if in_else { else_branch.push(stmt); } else { then_branch.push(stmt); }
            }
        }
        i += 1;
    }

    Ok((CipherStmt::IfElse { condition: cond, then_branch, else_branch }, i))
}

fn parse_watchdog_line(line: &str) -> Result<WatchdogInvariant, String> {
    // Format: !Δ { |Y| < 1e4, latency < 15ns }
    let mut max_magnitude = None;
    let mut max_latency_ns = None;

    if let (Some(s), Some(e)) = (line.find('{'), line.rfind('}')) {
        let inside = &line[s + 1..e];
        for clause in inside.split(',') {
            let c = clause.trim();
            if c.contains('|') && c.contains('<') {
                let val_str = c.split('<').nth(1).unwrap_or("").trim();
                max_magnitude = val_str.parse().ok();
            } else if c.to_lowercase().contains("latency") && c.contains('<') {
                let val_str = c.split('<').nth(1).unwrap_or("").trim().trim_end_matches("ns").trim();
                max_latency_ns = val_str.parse().ok();
            }
        }
    }

    Ok(WatchdogInvariant {
        max_magnitude,
        max_latency_ns,
        custom_clause: Some(line.to_string()),
    })
}

// ----------------------------------------------------------------------------
// [5] VLIW Cipher Bundle Parser
// ----------------------------------------------------------------------------
fn parse_vliw_bundle(line: &str) -> Result<VliwBundle, String> {
    let colon_idx = line.find(':').ok_or_else(|| "Missing ':' in VLIW bundle".to_string())?;
    let cycle_str = line[1..colon_idx].trim();
    let cycle = usize::from_str_radix(cycle_str, 16).unwrap_or(0);

    let slots_str = line[colon_idx + 1..].trim();
    let mut slots = Vec::new();

    for raw_slot in slots_str.split_whitespace() {
        if raw_slot.starts_with(';') {
            break; // Start of comment
        }
        let is_16_char = raw_slot.len() >= 16;
        let prefix = raw_slot.chars().next().unwrap_or('_');
        let opcode = if raw_slot.len() >= 3 { raw_slot[1..3].to_string() } else { "NO".to_string() };
        let dst = if raw_slot.len() >= 5 {
            u8::from_str_radix(&raw_slot[3..5], 16).unwrap_or(0)
        } else {
            0
        };
        let op_math = raw_slot.chars().nth(5).unwrap_or('#');
        let src1 = if raw_slot.len() >= 7 {
            u8::from_str_radix(&raw_slot[6..7], 16).unwrap_or(0)
        } else {
            0
        };
        let src2 = if raw_slot.len() >= 9 {
            Some(u8::from_str_radix(&raw_slot[7..8], 16).unwrap_or(0))
        } else {
            None
        };
        let crc_token = if !raw_slot.is_empty() {
            raw_slot.chars().last().unwrap().to_string()
        } else {
            ">".to_string()
        };

        slots.push(VliwSlot {
            raw: raw_slot.to_string(),
            is_16_char_tile: is_16_char,
            prefix,
            opcode,
            dst,
            op_math,
            src1,
            src2,
            crc_token,
        });
    }

    Ok(VliwBundle { cycle, slots })
}

// ----------------------------------------------------------------------------
// [6] Recursive Mathematical Expression Parser & Precedence Climber
// ----------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq)]
pub enum MathToken {
    Ident(String),
    Num(f64),
    LParen,
    RParen,
    Comma,
    NormBar, // ∥
    Sigma,   // σ
    TensorMatMul,         // ⊗
    InterferenceCoupling, // <~>
    AssociativeScan,      // >>=
    LogSpaceGating,       // <|>
    RingReduction,        // <%>
    RadixGraft,           // /@
    PeriodicResonance,    // ~*~
    ExteriorWedge,        // ^
    Dot,                  // .
    Add,                  // +
    Sub,                  // -
    Mul,                  // *
    Div,                  // /
    Mod,                  // %
    Tilde,                // ~
}

pub fn parse_math_expr_str(input: &str) -> Result<MathExpr, String> {
    let tokens = tokenize_math_expr(input)?;
    if tokens.is_empty() {
        return Err("Empty mathematical expression".to_string());
    }
    let mut parser = MathExprParser::new(tokens);
    let expr = parser.parse_expr()?;
    if parser.pos < parser.tokens.len() {
        return Err(format!("Unexpected trailing token at position {}: {:?}", parser.pos, parser.tokens[parser.pos]));
    }
    Ok(expr)
}

fn tokenize_math_expr(input: &str) -> Result<Vec<MathToken>, String> {
    let mut tokens = Vec::new();
    let chars: Vec<char> = input.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        let c = chars[i];
        if c.is_whitespace() {
            i += 1;
            continue;
        }

        // Multi-char operators
        if i + 2 < chars.len() {
            let s3: String = chars[i..i + 3].iter().collect();
            if s3 == "<~>" {
                tokens.push(MathToken::InterferenceCoupling);
                i += 3;
                continue;
            } else if s3 == ">>=" {
                tokens.push(MathToken::AssociativeScan);
                i += 3;
                continue;
            } else if s3 == "<|>" {
                tokens.push(MathToken::LogSpaceGating);
                i += 3;
                continue;
            } else if s3 == "<%>" {
                tokens.push(MathToken::RingReduction);
                i += 3;
                continue;
            } else if s3 == "~*~" {
                tokens.push(MathToken::PeriodicResonance);
                i += 3;
                continue;
            }
        }
        if i + 1 < chars.len() {
            let s2: String = chars[i..i + 2].iter().collect();
            if s2 == "/@" {
                tokens.push(MathToken::RadixGraft);
                i += 2;
                continue;
            }
        }

        // Single-char symbols
        match c {
            '⊗' => { tokens.push(MathToken::TensorMatMul); i += 1; continue; }
            '∥' => { tokens.push(MathToken::NormBar); i += 1; continue; }
            'σ' => { tokens.push(MathToken::Sigma); i += 1; continue; }
            '(' => { tokens.push(MathToken::LParen); i += 1; continue; }
            ')' => { tokens.push(MathToken::RParen); i += 1; continue; }
            ',' => { tokens.push(MathToken::Comma); i += 1; continue; }
            '^' => { tokens.push(MathToken::ExteriorWedge); i += 1; continue; }
            '+' => { tokens.push(MathToken::Add); i += 1; continue; }
            '-' => { tokens.push(MathToken::Sub); i += 1; continue; }
            '*' => { tokens.push(MathToken::Mul); i += 1; continue; }
            '/' => { tokens.push(MathToken::Div); i += 1; continue; }
            '%' => { tokens.push(MathToken::Mod); i += 1; continue; }
            '~' => { tokens.push(MathToken::Tilde); i += 1; continue; }
            _ => {}
        }

        // Numbers: digits
        if c.is_ascii_digit() {
            let start = i;
            while i < chars.len() && (chars[i].is_ascii_digit() || chars[i] == '.' || chars[i] == 'e' || chars[i] == 'E' || (chars[i] == '-' && (chars[i - 1] == 'e' || chars[i - 1] == 'E'))) {
                i += 1;
            }
            let num_str: String = chars[start..i].iter().collect();
            let val = num_str.parse::<f64>().map_err(|e| format!("Invalid number '{}': {}", num_str, e))?;
            tokens.push(MathToken::Num(val));
            continue;
        }

        // Identifiers: variables, function names, registers (#R1, alpha, etc.)
        if c.is_alphabetic() || c == '_' || c == '#' || c == '.' || (!c.is_ascii() && c != '⊗' && c != '∥' && c != 'σ') {
            let start = i;
            while i < chars.len() {
                let ch = chars[i];
                if ch.is_whitespace() || ch == '(' || ch == ')' || ch == ',' || ch == '+' || ch == '-' 
                    || ch == '*' || ch == '/' || ch == '%' || ch == '^' || ch == '⊗' || ch == '∥' 
                    || ch == 'σ' || ch == '~' || ch == '<' || ch == '>' {
                    break;
                }
                i += 1;
            }
            let ident: String = chars[start..i].iter().collect();
            tokens.push(MathToken::Ident(ident));
            continue;
        }

        return Err(format!("Unrecognized character in math expression: '{}'", c));
    }

    Ok(tokens)
}

struct MathExprParser {
    tokens: Vec<MathToken>,
    pos: usize,
}

impl MathExprParser {
    fn new(tokens: Vec<MathToken>) -> Self {
        Self { tokens, pos: 0 }
    }

    fn peek(&self) -> Option<&MathToken> {
        self.tokens.get(self.pos)
    }

    fn advance(&mut self) -> Option<MathToken> {
        if self.pos < self.tokens.len() {
            let t = self.tokens[self.pos].clone();
            self.pos += 1;
            Some(t)
        } else {
            None
        }
    }

    fn parse_expr(&mut self) -> Result<MathExpr, String> {
        self.parse_binary(0)
    }

    fn get_binop_info(token: &MathToken) -> Option<(MathBinaryOp, u8)> {
        match token {
            // Level 1: Add, Sub (prec 10)
            MathToken::Add => Some((MathBinaryOp::Add, 10)),
            MathToken::Sub => Some((MathBinaryOp::Sub, 10)),
            // Level 2: Mul, Div, Mod (prec 20)
            MathToken::Mul => Some((MathBinaryOp::Mul, 20)),
            MathToken::Div => Some((MathBinaryOp::Div, 20)),
            MathToken::Mod => Some((MathBinaryOp::Mod, 20)),
            // Level 3: Advanced Tensor / Morphism Ops (prec 30)
            MathToken::TensorMatMul => Some((MathBinaryOp::TensorMatMul, 30)),
            MathToken::InterferenceCoupling => Some((MathBinaryOp::InterferenceCoupling, 30)),
            MathToken::AssociativeScan => Some((MathBinaryOp::AssociativeScan, 30)),
            MathToken::LogSpaceGating => Some((MathBinaryOp::LogSpaceGating, 30)),
            MathToken::RingReduction => Some((MathBinaryOp::RingReduction, 30)),
            MathToken::RadixGraft => Some((MathBinaryOp::RadixGraft, 30)),
            MathToken::PeriodicResonance => Some((MathBinaryOp::PeriodicResonance, 30)),
            MathToken::ExteriorWedge => Some((MathBinaryOp::ExteriorWedge, 30)),
            MathToken::Dot => Some((MathBinaryOp::InnerScalar, 30)),
            _ => None,
        }
    }

    fn parse_binary(&mut self, min_prec: u8) -> Result<MathExpr, String> {
        let mut lhs = self.parse_prefix()?;

        while let Some(tok) = self.peek() {
            if let Some((op, prec)) = Self::get_binop_info(tok) {
                if prec < min_prec {
                    break;
                }
                self.advance();
                let rhs = self.parse_binary(prec + 1)?;
                lhs = MathExpr::Binary {
                    op,
                    lhs: Box::new(lhs),
                    rhs: Box::new(rhs),
                };
            } else {
                break;
            }
        }

        Ok(lhs)
    }

    fn parse_prefix(&mut self) -> Result<MathExpr, String> {
        match self.peek() {
            Some(MathToken::Sub) => {
                self.advance();
                let operand = self.parse_prefix()?;
                Ok(MathExpr::Unary {
                    op: MathUnaryOp::Neg,
                    operand: Box::new(operand),
                })
            }
            Some(MathToken::Tilde) => {
                self.advance();
                let operand = self.parse_prefix()?;
                Ok(MathExpr::Unary {
                    op: MathUnaryOp::RotorReverse,
                    operand: Box::new(operand),
                })
            }
            Some(MathToken::NormBar) => {
                self.advance(); // consume ∥
                let inner = self.parse_expr()?;
                match self.advance() {
                    Some(MathToken::NormBar) => Ok(MathExpr::Norm(Box::new(inner))),
                    other => Err(format!("Expected closing '∥', found {:?}", other)),
                }
            }
            Some(MathToken::Sigma) => {
                self.advance(); // consume σ
                if let Some(MathToken::LParen) = self.peek() {
                    self.advance();
                    let inner = self.parse_expr()?;
                    match self.advance() {
                        Some(MathToken::RParen) => Ok(MathExpr::Activation {
                            kind: ActivationKind::Silu,
                            operand: Box::new(inner),
                        }),
                        other => Err(format!("Expected ')' after σ activation, found {:?}", other)),
                    }
                } else {
                    let operand = self.parse_prefix()?;
                    Ok(MathExpr::Activation {
                        kind: ActivationKind::Silu,
                        operand: Box::new(operand),
                    })
                }
            }
            Some(MathToken::LParen) => {
                self.advance(); // consume (
                let inner = self.parse_expr()?;
                match self.advance() {
                    Some(MathToken::RParen) => Ok(inner),
                    other => Err(format!("Expected ')', found {:?}", other)),
                }
            }
            Some(MathToken::Num(val)) => {
                let v = *val;
                self.advance();
                Ok(MathExpr::Immediate(v))
            }
            Some(MathToken::Ident(name)) => {
                let name = name.clone();
                self.advance();
                if let Some(MathToken::LParen) = self.peek() {
                    self.advance(); // consume (
                    let mut args = Vec::new();
                    if let Some(MathToken::RParen) = self.peek() {
                        self.advance();
                    } else {
                        loop {
                            let arg = self.parse_expr()?;
                            args.push(arg);
                            match self.peek() {
                                Some(MathToken::Comma) => {
                                    self.advance();
                                }
                                Some(MathToken::RParen) => {
                                    self.advance();
                                    break;
                                }
                                other => return Err(format!("Expected ',' or ')' in argument list, found {:?}", other)),
                            }
                        }
                    }

                    match name.as_str() {
                        "silu" | ".op.silu" => {
                            let operand = args.into_iter().next().unwrap_or(MathExpr::Immediate(0.0));
                            Ok(MathExpr::Activation {
                                kind: ActivationKind::Silu,
                                operand: Box::new(operand),
                            })
                        }
                        "sigmoid" | ".op.sigmoid" => {
                            let operand = args.into_iter().next().unwrap_or(MathExpr::Immediate(0.0));
                            Ok(MathExpr::Activation {
                                kind: ActivationKind::Sigmoid,
                                operand: Box::new(operand),
                            })
                        }
                        "gelu" | ".op.gelu" => {
                            let operand = args.into_iter().next().unwrap_or(MathExpr::Immediate(0.0));
                            Ok(MathExpr::Activation {
                                kind: ActivationKind::Gelu,
                                operand: Box::new(operand),
                            })
                        }
                        "relu" | ".op.relu" => {
                            let operand = args.into_iter().next().unwrap_or(MathExpr::Immediate(0.0));
                            Ok(MathExpr::Activation {
                                kind: ActivationKind::Relu,
                                operand: Box::new(operand),
                            })
                        }
                        "rmsnorm" | "norm" | ".op.rmsnorm" => {
                            let operand = args.into_iter().next().unwrap_or(MathExpr::Immediate(0.0));
                            Ok(MathExpr::Norm(Box::new(operand)))
                        }
                        _ => Ok(MathExpr::Primitive { name, args }),
                    }
                } else {
                    Ok(MathExpr::Variable(name))
                }
            }
            other => Err(format!("Unexpected token in prefix position: {:?}", other)),
        }
    }
}

