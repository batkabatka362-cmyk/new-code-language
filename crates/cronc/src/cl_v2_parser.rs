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
        if line.starts_with("@kernel") {
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

        // [5] VLIW Microcode Bundle: B0000: ...
        if line.starts_with('B') && line.len() >= 5 && line.chars().nth(5) == Some(':') {
            let bundle = parse_vliw_bundle(line)?;
            doc.items.push(ProgramItem::VliwBundle(bundle.clone()));
            doc.vliw_stream.push(bundle);
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
    let kernel_name = first.trim_start_matches("@kernel").trim().to_string();
    let mut target = "silicon.generic".to_string();
    let mut ipc_target = 4.0f32;
    let mut cores = 256usize;
    let mut mesh_routing = None;

    i += 1;
    while i < lines.len() {
        let line = lines[i].trim();
        if line.starts_with(".target") {
            target = line.trim_start_matches(".target").trim().to_string();
        } else if line.starts_with(".ipc_target") {
            ipc_target = line.trim_start_matches(".ipc_target").trim().parse().unwrap_or(4.0);
        } else if line.starts_with(".cores") {
            cores = line.trim_start_matches(".cores").trim().parse().unwrap_or(256);
        } else if line.starts_with("@mesh") {
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
    // Format: @mesh [4x4x4x4] : ⮑ (x+1, y, z, w)
    let parts: Vec<&str> = line.split(':').collect();
    if parts.is_empty() {
        return Err("Invalid @mesh line".to_string());
    }
    let dims_part = parts[0].trim_start_matches("@mesh").trim();
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

        // Example: in: X[16x16:NVFP4] @sram[0x0000]
        let colon_idx = item_line.find(':').ok_or_else(|| format!("Expected ':' in io item: {}", item_line))?;
        let kind_str = item_line[..colon_idx].trim();
        let rest = item_line[colon_idx + 1..].trim();

        let kind = match kind_str {
            "in" => IoKind::Input,
            "out" => IoKind::Output,
            "wt" | "weight" => IoKind::Weight,
            "state" => IoKind::State,
            _ => IoKind::Input,
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

        // Parse mem_part: sram[0x0000]
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
    let name = after_section[..name_end].trim().to_string();

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
                steps.push(MathStep {
                    target_var: dest.clone(),
                    expr: expr.clone(),
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
