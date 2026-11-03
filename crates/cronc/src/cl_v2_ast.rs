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
    pub expr_ast: Option<MathExpr>,
}

// ----------------------------------------------------------------------------
// [3b] RECURSIVE MATHEMATICAL EXPRESSION AST & SHAPE INFERENCE
// ----------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq)]
pub enum MathExpr {
    Variable(String),
    Immediate(f64),
    Binary {
        op: MathBinaryOp,
        lhs: Box<MathExpr>,
        rhs: Box<MathExpr>,
    },
    Unary {
        op: MathUnaryOp,
        operand: Box<MathExpr>,
    },
    Norm(Box<MathExpr>),
    Activation {
        kind: ActivationKind,
        operand: Box<MathExpr>,
    },
    Primitive {
        name: String,
        args: Vec<MathExpr>,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MathBinaryOp {
    TensorMatMul,        // ⊗
    InterferenceCoupling,// <~>
    AssociativeScan,     // >>=
    LogSpaceGating,      // <|>
    RingReduction,       // <%>
    RadixGraft,          // /@
    PeriodicResonance,   // ~*~
    ExteriorWedge,       // ^
    InnerScalar,         // .
    Add,                 // +
    Sub,                 // -
    Mul,                 // *
    Div,                 // /
    Mod,                 // %
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MathUnaryOp {
    Neg,
    RotorReverse,        // ~
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActivationKind {
    Sigmoid,
    Silu,
    Gelu,
    Relu,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TensorShape {
    pub dims: Vec<usize>,
    pub dtype: String,
}

impl TensorShape {
    pub fn new(dims: Vec<usize>, dtype: &str) -> Self {
        Self {
            dims,
            dtype: dtype.to_string(),
        }
    }

    /// Check and infer matrix multiplication: (M, K) ⊗ (K, N) -> (M, N)
    pub fn matmul(&self, other: &TensorShape) -> Result<TensorShape, String> {
        if self.dims.len() < 2 || other.dims.len() < 2 {
            return Ok(self.clone());
        }
        let k1 = self.dims[self.dims.len() - 1];
        let k2 = other.dims[other.dims.len() - 2];
        if k1 != k2 {
            return Err(format!(
                "Compile-time Dimension Mismatch in Tensor Contraction (⊗): {:?} cannot contract with {:?} ({} != {})",
                self.dims, other.dims, k1, k2
            ));
        }
        let mut out_dims = self.dims[..self.dims.len() - 1].to_vec();
        out_dims.push(other.dims[other.dims.len() - 1]);
        Ok(TensorShape {
            dims: out_dims,
            dtype: self.dtype.clone(),
        })
    }

    /// Check and infer elementwise binary operation: shapes must broadcast or match
    pub fn elementwise(&self, other: &TensorShape, op: MathBinaryOp) -> Result<TensorShape, String> {
        if self.dims == other.dims || other.dims.is_empty() || self.dims.is_empty() {
            return Ok(self.clone());
        }
        // Scalar broadcasting: [1] broadcasts to any tensor shape
        if other.dims == [1] || other.dims.iter().product::<usize>() == 1 {
            return Ok(self.clone());
        }
        if self.dims == [1] || self.dims.iter().product::<usize>() == 1 {
            return Ok(other.clone());
        }
        if self.dims.iter().product::<usize>() == other.dims.iter().product::<usize>() {
            return Ok(self.clone());
        }
        Err(format!(
            "Compile-time Shape Mismatch in Elementwise {:?}: {:?} vs {:?}",
            op, self.dims, other.dims
        ))
    }

    /// Check and infer Associative Scan (>>=): input dimension must match state dimension
    pub fn associative_scan(&self, state: &TensorShape) -> Result<TensorShape, String> {
        if self.dims.is_empty() || state.dims.is_empty() {
            return Ok(self.clone());
        }
        let in_d = self.dims[self.dims.len() - 1];
        let state_d = state.dims[0];
        if in_d != state_d {
            return Err(format!(
                "Compile-time Dimension Mismatch in Associative Scan (>>=): Input dim {} does not match State dim {} ({:?} vs {:?})",
                in_d, state_d, self.dims, state.dims
            ));
        }
        Ok(self.clone())
    }

    /// Check and infer Photonic Interference Coupling (<~>): MZI grid size parity
    pub fn photonic_coupling(&self, other: &TensorShape) -> Result<TensorShape, String> {
        if self.dims.is_empty() || other.dims.is_empty() {
            return Ok(self.clone());
        }
        let d1 = self.dims[self.dims.len() - 1];
        let d2 = other.dims[other.dims.len() - 1];
        if d1 != d2 && self.dims != other.dims {
            return Err(format!(
                "Compile-time Phase Mismatch in Photonic Interference Coupling (<~>): MZI grid size {} != {} ({:?} vs {:?})",
                d1, d2, self.dims, other.dims
            ));
        }
        Ok(self.clone())
    }

    /// Check and infer Exterior Wedge Product (^): Clifford multivector space consistency
    pub fn exterior_wedge(&self, other: &TensorShape) -> Result<TensorShape, String> {
        if self.dims.is_empty() || other.dims.is_empty() {
            return Ok(self.clone());
        }
        let d1 = self.dims[self.dims.len() - 1];
        let d2 = other.dims[other.dims.len() - 1];
        if d1 != d2 {
            return Err(format!(
                "Compile-time Multivector Space Mismatch in Exterior Wedge (^): Cl(n) dimension {} != {} ({:?} vs {:?})",
                d1, d2, self.dims, other.dims
            ));
        }
        Ok(self.clone())
    }

    /// Check and infer Polynomial NTT Crypto-Ring Reduction (<%>)
    pub fn ring_reduction(&self, other: &TensorShape) -> Result<TensorShape, String> {
        if self.dims.is_empty() || other.dims.is_empty() {
            return Ok(self.clone());
        }
        let deg1 = self.dims[0];
        let deg2 = other.dims[0];
        if deg1 != deg2 {
            return Err(format!(
                "Compile-time Cyclotomic Degree Mismatch in Crypto-Ring Reduction (<%>): NTT degree {} != {} ({:?} vs {:?})",
                deg1, deg2, self.dims, other.dims
            ));
        }
        Ok(self.clone())
    }
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
