// ============================================================================
// CRON AI Vibe-Coding Schema & Specification Engine (cl_spec.rs)
// Generates formal EBNF grammar, machine-readable JSON Schema, and zero-shot
// LLM System Prompt contexts for AI agents writing pure .cl microcode.
//
// 100% Pure Rust — Zero External Dependencies.
// ============================================================================

use crate::cl_lang::KNOWN_OPCODES;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpecFormat {
    Ebnf,
    Json,
    Prompt,
    All,
}

impl SpecFormat {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_ascii_lowercase().as_str() {
            "ebnf" => Some(SpecFormat::Ebnf),
            "json" | "schema" => Some(SpecFormat::Json),
            "prompt" | "system-prompt" | "llm" => Some(SpecFormat::Prompt),
            "all" => Some(SpecFormat::All),
            _ => None,
        }
    }
}

/// Generate formal EBNF Grammar for .cl (Cognitive Low-level) Language
pub fn generate_cl_ebnf_grammar() -> String {
    let mut ebnf = String::new();
    ebnf.push_str("(* ========================================================================== *)\n");
    ebnf.push_str("(* CRON Cognitive Low-Level Machine Language (.cl) Formal EBNF Specification *)\n");
    ebnf.push_str("(* Target Architecture: 256-Core 4D-Torus Hybrid Optical/Neuromorphic Silicon *)\n");
    ebnf.push_str("(* ========================================================================== *)\n\n");

    ebnf.push_str("Program          = { Statement } ;\n");
    ebnf.push_str("Statement        = Directive | LabelDef | Bundle | Comment | EmptyLine ;\n\n");

    ebnf.push_str("Directive        = CoreDirective | SectionDirective ;\n");
    ebnf.push_str("CoreDirective    = \".core\" , \"[\" , Coord , \",\" , Coord , \",\" , Coord , \",\" , Coord , \"]:\" ;\n");
    ebnf.push_str("Coord            = \"0\" | \"1\" | \"2\" | \"3\" ; (* 4x4x4x4 Torus Dimension *)\n");
    ebnf.push_str("SectionDirective = ( \".data\" | \".weights\" | \".pgas\" ) , [ Ident ] , \":\" ;\n\n");

    ebnf.push_str("LabelDef         = \"@\" , Ident , \":\" | \"L\" , Digit , Digit , \":\" ;\n");
    ebnf.push_str("Ident            = Letter , { Letter | Digit | \"_\" } ;\n\n");

    ebnf.push_str("Bundle           = CycleHeader , Slot , Slot , Slot , Slot ;\n");
    ebnf.push_str("CycleHeader      = \"B\" , { Digit } , \":\" ;\n\n");

    ebnf.push_str("(* Exact 10-Character Fixed-Width Instruction Slot Token *)\n");
    ebnf.push_str("Slot             = Prefix , Opcode , DestReg , Mode , SrcReg , CrcParity , ImmParam , Terminator ;\n");
    ebnf.push_str("Prefix           = \"_\" | \"'\" | \"~\" | \"@\" ;\n");
    ebnf.push_str("(* Prefix Semantics: '_' = Operation, ''' = Immediate, '~' = Reversible, '@' = Control/Barrier *)\n\n");

    ebnf.push_str("Opcode           = \n");
    for (i, op) in KNOWN_OPCODES.iter().enumerate() {
        if i == 0 {
            ebnf.push_str(&format!("                   \"{}\"", op));
        } else if i % 8 == 0 {
            ebnf.push_str(&format!(" |\n                   \"{}\"", op));
        } else {
            ebnf.push_str(&format!(" | \"{}\"", op));
        }
    }
    ebnf.push_str(" ;\n\n");

    ebnf.push_str("DestReg          = HexDigit , HexDigit ; (* 00..0F -> Registers R0..RF *)\n");
    ebnf.push_str("Mode             = \"$\" | \"#\" | \"@\" | \"+\" | \"-\" | \"*\" | \"/\" | \"&\" | \"|\" | \"^\" | \"!\" | \"?\" ;\n");
    ebnf.push_str("SrcReg           = HexDigit ;            (* 0..F -> Source Register R0..RF *)\n");
    ebnf.push_str("CrcParity        = HexDigit ;            (* CRC-8 ATM Polynomial (x^8 + x^2 + x + 1) nibble *)\n");
    ebnf.push_str("ImmParam         = ASCIIPrintable ;      (* Immediate payload nibble or branch offset *)\n");
    ebnf.push_str("Terminator       = \">\" | \"!\" | \"?\" | \";\" ; (* '>' = Exec, '!' = Halt/Trap, '?' = Predicated *) \n\n");

    ebnf.push_str("HexDigit         = \"0\" | \"1\" | \"2\" | \"3\" | \"4\" | \"5\" | \"6\" | \"7\"\n");
    ebnf.push_str("                 | \"8\" | \"9\" | \"A\" | \"B\" | \"C\" | \"D\" | \"E\" | \"F\" ;\n");
    ebnf.push_str("Digit            = \"0\" | \"1\" | \"2\" | \"3\" | \"4\" | \"5\" | \"6\" | \"7\" | \"8\" | \"9\" ;\n");
    ebnf.push_str("Letter           = \"a\"..\"z\" | \"A\"..\"Z\" ;\n");
    ebnf.push_str("Comment          = ( \";\" | \"//\" ) , { AllCharacters - Newline } , Newline ;\n");
    ebnf.push_str("EmptyLine        = [ Whitespace ] , Newline ;\n");

    ebnf
}

/// Generate machine-readable JSON Schema for .cl Language & VLIW Bundles
pub fn generate_cl_json_schema() -> String {
    let mut json = String::new();
    json.push_str("{\n");
    json.push_str("  \"$schema\": \"http://json-schema.org/draft-07/schema#\",\n");
    json.push_str("  \"title\": \"CronCognitiveLowLevelLanguageSpec\",\n");
    json.push_str("  \"description\": \"Complete Machine-Readable JSON Specification for CRON .cl VLIW Silicon Microcode\",\n");
    json.push_str("  \"type\": \"object\",\n");
    json.push_str("  \"properties\": {\n");

    // Architecture metadata
    json.push_str("    \"target_architecture\": {\n");
    json.push_str("      \"name\": \"CRON 256-Core 4D-Torus Hybrid Silicon\",\n");
    json.push_str("      \"mesh_dimensions\": [4, 4, 4, 4],\n");
    json.push_str("      \"total_cores\": 256,\n");
    json.push_str("      \"vliw_bundle_width_bits\": 128,\n");
    json.push_str("      \"slots_per_bundle\": 4,\n");
    json.push_str("      \"slot_token_length_chars\": 10,\n");
    json.push_str("      \"canonical_nop_token\": \"_NO00#000>\",\n");
    json.push_str("      \"crc_polynomial\": \"x^8 + x^2 + x + 1 (0x07 CRC-8 ATM)\"\n");
    json.push_str("    },\n");

    // Register ABI map
    json.push_str("    \"registers\": [\n");
    let reg_abis = [
        ("R0", "00", "$rv", "Return Value / Primary GEMM Accumulator"),
        ("R1", "01", "$a0", "Function Argument 0 / Knowledge Graph Root"),
        ("R2", "02", "$a1", "Function Argument 1 / Waveguide Phase Vector"),
        ("R3", "03", "$a2", "Function Argument 2 / Neuromorphic Synapse Weight"),
        ("R4", "04", "$t0", "Temporary Scratch Register 0 (Caller-Saved)"),
        ("R5", "05", "$t1", "Temporary Scratch Register 1 (Caller-Saved)"),
        ("R6", "06", "$t2", "Temporary Scratch Register 2 (Caller-Saved)"),
        ("R7", "07", "$s0", "Preserved Register 0 (Callee-Saved)"),
        ("R8", "08", "$s1", "Preserved Register 1 (Callee-Saved)"),
        ("R9", "09", "$s2", "Preserved Register 2 (Callee-Saved)"),
        ("R10", "0A", "$im0", "Brain 5 Latent Chaos Dimension X"),
        ("R11", "0B", "$im1", "Brain 5 Latent Chaos Dimension Y"),
        ("R12", "0C", "$im2", "Brain 5 Latent Chaos Dimension Z"),
        ("R13", "0D", "$ar0", "Brain 6 Arbiter Thermal Telemetry"),
        ("R14", "0E", "$ar1", "Brain 6 Arbiter Sentry Parity Checkpoint"),
        ("R15", "0F", "$bp", "PGAS Memory Base Pointer (64KB SRAM Bank)"),
    ];
    for (i, (name, hex, abi, role)) in reg_abis.iter().enumerate() {
        json.push_str("      {\n");
        json.push_str(&format!("        \"id\": {},\n", i));
        json.push_str(&format!("        \"name\": \"{}\",\n", name));
        json.push_str(&format!("        \"hex_index\": \"{}\",\n", hex));
        json.push_str(&format!("        \"abi_alias\": \"{}\",\n", abi));
        json.push_str(&format!("        \"role\": \"{}\"\n", role));
        if i + 1 < reg_abis.len() {
            json.push_str("      },\n");
        } else {
            json.push_str("      }\n");
        }
    }
    json.push_str("    ],\n");

    // Opcodes categorization
    json.push_str("    \"opcode_catalog\": [\n");
    let categorized_ops = [
        ("OP", "Photonic GEMM", "Optical Matrix Multiplication on MZI Mesh (Brain 2)"),
        ("FA", "Autodiff", "Forward Automatic Differentiation Sensory Tap"),
        ("PO", "ALU", "Predicated SIMD Arithmetic/Logic Unit"),
        ("MD", "Sub-byte SIMD", "Multi-Dot BitNet Ternary / INT4 Dot Product"),
        ("BK", "Reversible", "Backward Invert Reversible Pass (0-Entropy)"),
        ("BL", "Spatial Cache", "Halo Boundary Cache Blend across 4D Torus"),
        ("RF", "Reversible", "Reversible Fredkin Controlled Permutation Gate"),
        ("GU", "Optimizer", "Neuromorphic Gradient Weight Update"),
        ("ST", "Neuromorphic", "Spike-Timing-Dependent Plasticity (STDP) Step"),
        ("SY", "Symbolic", "Symbolic Causal Hyper-Graph Unification (Brain 1)"),
        ("RS", "Region Arena", "0-Cycle Region Instantaneous Arena Reset"),
        ("PK", "Sub-byte SIMD", "Pack/Unpack Sub-Byte SIMD Vector Registers"),
        ("TL", "Spatial Memory", "4D Hypercube Tiling Coordinate Mapping"),
        ("YD", "Coroutine", "Fiber Cooperative Yield to Local Core Scheduler"),
        ("SP", "Fiber", "Spawn Asynchronous Micro-Fiber Task"),
        ("FJ", "Fiber", "Join Micro-Fiber Barrier"),
        ("DW", "NoC DMA", "DMA Transfer Asynchronous Wait"),
        ("SB", "NoC Broadcast", "Spatial Broadcast Across All 8 Neighbors in 4D Mesh"),
        ("SH", "Self-Healing", "Silicon Sentry Heartbeat Self-Healing Trigger"),
        ("RC", "Fault Tolerance", "Re-route Fallback Deadlock Avoidance Channel"),
        ("TO", "Reversible", "Toffoli 3-Wire Quantum/Reversible Gate"),
        ("WD", "Photonic WDM", "Wavelength Division Multiplexing Laser Modulator"),
        ("TX", "NoC Wormhole", "Channel Send: Wormhole Packet Injection into Router"),
        ("RX", "NoC Wormhole", "Channel Recv: Core Mailbox FIFO Pop"),
        ("IR", "NoC Routing", "In-Network Hop Flight Reduction"),
        ("DF", "NoC Routing", "Adaptive Dynamic Deflection Routing"),
        ("JP", "Control Flow", "Unconditional Relative/Absolute Jump"),
        ("BZ", "Control Flow", "Branch if Register is Zero"),
        ("BN", "Control Flow", "Branch if Register is Non-Zero"),
        ("BL", "Control Flow", "Branch if Register is Negative / Less Than"),
        ("BG", "Control Flow", "Branch if Register is Greater Than"),
        ("bb", "Synchronization", "Chip-Wide 256-Core Global Hardware Barrier"),
        ("CC", "Cache Sentry", "256-Core Instruction/Data Cache Flush"),
        ("DD", "NoC DMA", "Zero-Overhead Direct 4D-Torus NoC DMA Transfer"),
        ("EE", "DVFS Power", "Energy-Aware Dynamic Voltage and Frequency Scaling"),
        ("HL", "System", "Halt Core Execution / Enter Low-Power Standby"),
        ("NO", "System", "No Operation (NOP Idle Power-Gated Slot)"),
    ];
    for (i, (op, cat, desc)) in categorized_ops.iter().enumerate() {
        json.push_str("      {\n");
        json.push_str(&format!("        \"opcode\": \"{}\",\n", op));
        json.push_str(&format!("        \"category\": \"{}\",\n", cat));
        json.push_str(&format!("        \"description\": \"{}\"\n", desc));
        if i + 1 < categorized_ops.len() {
            json.push_str("      },\n");
        } else {
            json.push_str("      }\n");
        }
    }
    json.push_str("    ]\n");

    json.push_str("  }\n");
    json.push_str("}\n");

    json
}

/// Generate optimized Zero-Shot System Prompt for AI Coding Agents (LLMs)
pub fn generate_cl_ai_system_prompt() -> String {
    let mut prompt = String::new();
    prompt.push_str("### SYSTEM ROLE: SSS+ CRON .cl SILICON VIBE-CODING ENGINE\n");
    prompt.push_str("You are an autonomous compiler and microcode engineer generating native `.cl` (Cognitive Low-level) machine code for the CRON 256-Core 4D-Torus Processor.\n\n");

    prompt.push_str("#### CORE INVARIANTS & GRAMMAR RULES:\n");
    prompt.push_str("1. **Fixed-Width 10-Character Slot Format**:\n");
    prompt.push_str("   Every instruction slot token MUST be EXACTLY 10 ASCII characters:\n");
    prompt.push_str("   `[0]`    Prefix: '_' (operation), ''' (immediate), '~' (reversible), '@' (control)\n");
    prompt.push_str("   `[1..2]` Opcode: 2-char mnemonic (OP, MD, PO, ST, TX, RX, JP, BZ, HL, NO)\n");
    prompt.push_str("   `[3..4]` Dest Register: 2 hex characters (00..0F -> R0..RF)\n");
    prompt.push_str("   `[5]`    Mode Delimiter: '$' (reg-reg), '#' (reg-imm), '@' (pointer)\n");
    prompt.push_str("   `[6]`    Source Register / Parameter: 1 hex char (0..F -> R0..RF)\n");
    prompt.push_str("   `[7]`    CRC-8 ATM Parity Nibble: 1 hex char (auto-healed by `cron cl-heal`)\n");
    prompt.push_str("   `[8]`    Immediate / Low Param: 1 ASCII char (e.g. '0'..'F', offset)\n");
    prompt.push_str("   `[9]`    Terminator: '>' (execute), '!' (halt), '?' (predicated), ';' (trap)\n\n");

    prompt.push_str("2. **VLIW 4-Slot Bundle Structure**:\n");
    prompt.push_str("   - Each cycle line starts with `B<cycle_num>:` followed by exactly 4 slot tokens.\n");
    prompt.push_str("   - If you have fewer than 4 operations in a cycle, PAD with idle NOP slots: `_NO00#000>`.\n");
    prompt.push_str("   - Example: `B0000: '==01#00A> _NO00#000> _NO00#000> _NO00#000>`\n\n");

    prompt.push_str("3. **Labels & Multi-Core Partitioning**:\n");
    prompt.push_str("   - Target specific cores using `.core [x, y, z, w]:` where 0 <= x,y,z,w < 4.\n");
    prompt.push_str("   - Declare labels on their own line: `@label_name:` or `L00:`.\n");
    prompt.push_str("   - Jump / Branch opcodes: `_JP00$000>` (unconditional), `_BZ01$000>` (branch if zero), `_BN01$000>` (branch non-zero).\n\n");

    prompt.push_str("4. **NoC Wormhole Inter-Core Communication**:\n");
    prompt.push_str("   - Core A Send:    `_TX01$100>` (Transmit R1 into NoC channel)\n");
    prompt.push_str("   - Core B Receive: `_RX02$000>` (Receive into R2 from NoC mailbox)\n\n");

    prompt.push_str("#### CANONICAL FEW-SHOT EXAMPLES:\n\n");

    prompt.push_str("```cron\n");
    prompt.push_str("; Example 1: High-Throughput Matrix Multiply & Accumulate (FlashAttention kernel)\n");
    prompt.push_str(".core [0, 0, 0, 0]:\n");
    prompt.push_str("@matmul_loop:\n");
    prompt.push_str("B0000: '==01#00A> '==02#00B> '==03#00C> _NO00#000>\n");
    prompt.push_str("B0001: _OP01$28F> _MD04$102> _PO05$301> _NO00#000>\n");
    prompt.push_str("B0002: _SB00$400> _ST03$201> _NO00#000> _HL00#000!\n");
    prompt.push_str("```\n\n");

    prompt.push_str("```cron\n");
    prompt.push_str("; Example 2: Multi-Core 4D-Torus Producer-Consumer Pipeline\n");
    prompt.push_str(".core [0, 0, 0, 0]:\n");
    prompt.push_str("@producer:\n");
    prompt.push_str("B0000: '==01#0FF> _NO00#000> _NO00#000> _NO00#000>\n");
    prompt.push_str("B0001: _TX01$100> _NO00#000> _NO00#000> _NO00#000>\n");
    prompt.push_str("B0002: _HL00#000! _NO00#000> _NO00#000> _NO00#000>\n\n");
    prompt.push_str(".core [1, 0, 0, 0]:\n");
    prompt.push_str("@consumer:\n");
    prompt.push_str("B0000: _RX02$000> _NO00#000> _NO00#000> _NO00#000>\n");
    prompt.push_str("B0001: _PO00$204> _NO00#000> _NO00#000> _NO00#000>\n");
    prompt.push_str("B0002: _HL00#000! _NO00#000> _NO00#000> _NO00#000>\n");
    prompt.push_str("```\n\n");

    prompt.push_str("#### AI TOOL-CALL WORKFLOW:\n");
    prompt.push_str("- Generate `.cl` microcode following the above rules.\n");
    prompt.push_str("- Run `cron cl-heal <file.cl>` to automatically verify CRC parity & hazard latency.\n");
    prompt.push_str("- Run `cron cl-opt <file.cl>` to compact slots and achieve peak IPC (up to 4.0 ops/cycle).\n");
    prompt.push_str("- Run `cron cl-run <file.cl>` to execute in RAM with zero I/O and verify bit-exact silicon parity.\n");

    prompt
}

/// Unified specification generator based on chosen format
pub fn generate_cl_spec(format: SpecFormat) -> String {
    match format {
        SpecFormat::Ebnf => generate_cl_ebnf_grammar(),
        SpecFormat::Json => generate_cl_json_schema(),
        SpecFormat::Prompt => generate_cl_ai_system_prompt(),
        SpecFormat::All => {
            let mut out = String::new();
            out.push_str("================================================================================\n");
            out.push_str("SECTION 1: AI VIBE-CODING SYSTEM PROMPT & FEW-SHOT CONTEXT\n");
            out.push_str("================================================================================\n");
            out.push_str(&generate_cl_ai_system_prompt());
            out.push_str("\n\n");
            out.push_str("================================================================================\n");
            out.push_str("SECTION 2: MACHINE-READABLE JSON SCHEMA\n");
            out.push_str("================================================================================\n");
            out.push_str(&generate_cl_json_schema());
            out.push_str("\n\n");
            out.push_str("================================================================================\n");
            out.push_str("SECTION 3: FORMAL EBNF GRAMMAR\n");
            out.push_str("================================================================================\n");
            out.push_str(&generate_cl_ebnf_grammar());
            out
        }
    }
}
