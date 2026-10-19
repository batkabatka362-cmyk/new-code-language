// ============================================================================
// CRON CL v2.0 SOVEREIGN 5-PILLAR AST SPECIFICATION
// Module: cronc::cl_v2_ast (crates/cronc/src/cl_v2_ast.rs)
//
// Defines the 5 Pillars of the Post-Human AI-Silicon Language:
// [1] Silicon Target Contract:  @kernel, .target, .ipc_target, .cores, @mesh
// [2] I/O Memory Binding:       @io { in, out, wt @sram/@dram/@hbm }
// [3] Mathematical Morphism:    §Block[Dims] { States, X ↦ Equation }
// [4] Ciphered Control-Flow:    §ControlFlow { ⟳, ? ⮞ :, ←, !Δ watchdog }
// [5] 10/16-Slot VLIW Cipher:   B0000: '==... _MD... _DV... _TG...
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
pub enum ProgramItem {
    TargetContract(SiliconTargetContract),
    IoBinding(IoMemoryBinding),
    Function(FunctionBlock),
    ControlFlow(CipherControlFlow),
    Statement(CipherStmt),
    VliwBundle(VliwBundle),
    Watchdog(WatchdogInvariant),
}

#[derive(Debug, Clone, PartialEq)]
pub struct ClV2Document {
    pub items: Vec<ProgramItem>,
    pub target_contract: Option<SiliconTargetContract>,
    pub io_binding: Option<IoMemoryBinding>,
    pub math_morphism: Option<MathMorphismBlock>,
    pub functions: Vec<FunctionBlock>,
    pub control_flow: Option<CipherControlFlow>,
    pub top_level_statements: Vec<CipherStmt>,
    pub vliw_stream: Vec<VliwBundle>,
    pub watchdog: Option<WatchdogInvariant>,
}

impl ClV2Document {
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            target_contract: None,
            io_binding: None,
            math_morphism: None,
            functions: Vec::new(),
            control_flow: None,
            top_level_statements: Vec::new(),
            vliw_stream: Vec::new(),
            watchdog: None,
        }
    }
}

impl Default for ClV2Document {
    fn default() -> Self {
        Self::new()
    }
}

// ----------------------------------------------------------------------------
// [1] SILICON TARGET CONTRACT
// ----------------------------------------------------------------------------
#[derive(Debug, Clone, PartialEq)]
pub struct SiliconTargetContract {
    pub kernel_name: String,
    pub target: String,
    pub ipc_target: f32,
    pub cores: usize,
    pub mesh_routing: Option<MeshRouting>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MeshRouting {
    pub dimensions: Vec<usize>, // e.g. [4, 4, 4, 4]
    pub vector_formula: String, // e.g. "(x+1, y, z, w)"
}

// ----------------------------------------------------------------------------
// [2] I/O MEMORY BINDING
// ----------------------------------------------------------------------------
#[derive(Debug, Clone, PartialEq)]
pub struct IoMemoryBinding {
    pub items: Vec<IoBindingItem>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum IoKind {
    Input,
    Output,
    Weight,
    State,
}

#[derive(Debug, Clone, PartialEq)]
pub enum MemoryTier {
    Sram,
    Dram,
    Hbm,
    Dmem,
}

#[derive(Debug, Clone, PartialEq)]
pub struct IoBindingItem {
    pub kind: IoKind,
    pub name: String,
    pub shape: Vec<usize>,
    pub dtype: String,
    pub tier: MemoryTier,
    pub address: u64,
}

// ----------------------------------------------------------------------------
// [3] MATHEMATICAL TENSOR MORPHISM
// ----------------------------------------------------------------------------
#[derive(Debug, Clone, PartialEq)]
pub struct MathMorphismBlock {
    pub name: String,
    pub tile_dims: Vec<usize>,
    pub dtype: String,
    pub states: Vec<MathStateDecl>,
    pub input_symbol: String,
    pub output_symbol: Option<String>,
    pub equation_expr: String,
    pub steps: Vec<MathStep>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MathStateDecl {
    pub name: String,
    pub type_tag: String, // "SS[64x256]", "T[16x16:NVFP4]"
    pub dims: Vec<usize>,
    pub subtype: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MathStep {
    pub target_var: String,
    pub expr: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionBlock {
    pub name: String,
    pub tile_dims: Vec<usize>,
    pub dtype: String,
    pub params: Vec<String>,
    pub states: Vec<MathStateDecl>,
    pub input_symbol: String,
    pub output_symbol: Option<String>,
    pub equation_expr: String,
    pub steps: Vec<MathStep>,
    pub body: Vec<CipherStmt>,
}

// ----------------------------------------------------------------------------
// [4] CIPHERED CONTROL-FLOW & SAFETY SLA
// ----------------------------------------------------------------------------
#[derive(Debug, Clone, PartialEq)]
pub struct CipherControlFlow {
    pub statements: Vec<CipherStmt>,
    pub watchdog: Option<WatchdogInvariant>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum CipherStmt {
    /// ⟳ (count_or_cond) { body }
    Loop {
        condition: String,
        body: Vec<CipherStmt>,
    },
    /// ? (cond) ⮞ { then } : { else }
    IfElse {
        condition: String,
        then_branch: Vec<CipherStmt>,
        else_branch: Vec<CipherStmt>,
    },
    /// #Dest := Expression or variable := expression
    Assign {
        dest: String,
        expr: String,
    },
    /// #Reg ← @addr  or  @addr ← #Reg
    MemoryTransfer {
        dest: String,
        src: String,
    },
    /// ⮌ (Exit / Break)
    Exit,
    /// ⮌ expression (Return)
    Return(Option<String>),
    /// ⮍ (Continue / Next)
    Next,
    /// ⮑ target (Jump)
    Jump(String),
    /// Function Call
    Call {
        target: String,
        args: Vec<String>,
    },
    /// Standalone expression
    Expr(String),
}

#[derive(Debug, Clone, PartialEq)]
pub struct WatchdogInvariant {
    pub max_magnitude: Option<f64>,
    pub max_latency_ns: Option<f64>,
    pub custom_clause: Option<String>,
}

// ----------------------------------------------------------------------------
// [5] 10/16-CHAR VLIW SILICON CIPHER
// ----------------------------------------------------------------------------
#[derive(Debug, Clone, PartialEq)]
pub struct VliwBundle {
    pub cycle: usize,
    pub slots: Vec<VliwSlot>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct VliwSlot {
    pub raw: String,
    pub is_16_char_tile: bool,
    pub prefix: char,
    pub opcode: String,
    pub dst: u8,
    pub op_math: char,
    pub src1: u8,
    pub src2: Option<u8>,
    pub crc_token: String,
}
