// ============================================================================
// CRON & CL HIGH-LEVEL AI-NATIVE ABSTRACT SYNTAX TREE (AST)
// Level 1 Mathematical & Multi-Physics Declarative Specification
// Supporting the 7 Beyond-Addition Operators & 525 Universal Primitives
// ============================================================================

use std::collections::HashMap;

/// High-Level CL Module containing stages, imports, and kernels.
#[derive(Debug, Clone, PartialEq)]
pub struct HighModule {
    pub name: String,
    pub imports: Vec<ImportDecl>,
    pub stages: Vec<StageDecl>,
    pub architectures: Vec<ArchDecl>,
    pub kernels: Vec<KernelDecl>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ImportDecl {
    pub path: String, // e.g., "cl.tensor", "cl.recurrent", "cl.silicon.primitives"
    pub alias: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct StageDecl {
    pub name: String,
    pub params: HashMap<String, String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ArchDecl {
    pub name: String,
    pub states: Vec<StateField>,
    pub methods: Vec<KernelDecl>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct StateField {
    pub name: String,
    pub ty: HighType,
}

#[derive(Debug, Clone, PartialEq)]
pub struct KernelDecl {
    pub name: String,
    pub params: Vec<KernelParam>,
    pub return_type: Option<HighType>,
    pub attributes: Vec<Attribute>,
    pub body: Vec<Stmt>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct KernelParam {
    pub name: String,
    pub ty: HighType,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Attribute {
    pub name: String, // e.g. "realtime", "async_task"
    pub args: HashMap<String, String>,
}

/// AI-Native Strict Type System for CL
#[derive(Debug, Clone, PartialEq)]
pub enum HighType {
    Tensor {
        dtype: DType,
        dims: Vec<DimExpr>,
    },
    RegTile {
        rows: usize,
        cols: usize,
        dtype: DType,
    },
    Multivector {
        algebra: String, // e.g. "Cl3", "Cl(3,0)"
        dim: usize,
    },
    Spikes {
        width: usize,
        height: usize,
        time_steps: usize,
    },
    StateSpace {
        d_state: usize,
        d_inner: usize,
    },
    RadixTree {
        key_type: String,
        val_type: String,
        max_depth: usize,
    },
    CryptoRing {
        n_degree: usize,
        modulus: u64,
    },
    Custom(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum DType {
    NVFP4,
    MXFP4,
    MXFP6,
    FP8_E4M3,
    FP8_E5M2,
    BF16,
    FP16,
    FP32,
    FP64,
    Trit16, // 1.58-bit Ternary {-1, 0, +1}
    Int4,
    Int8,
    Int16,
    Int32,
    Int64,
    Bool,
    Custom(String),
}

#[derive(Debug, Clone, PartialEq)]
pub enum DimExpr {
    Static(usize),
    Symbolic(String), // e.g. "Batch", "Seq", "Dim", "B", "S", "D"
}

/// High-Level Statements
#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
    Let {
        name: String,
        ty: Option<HighType>,
        expr: Expr,
    },
    Assign {
        target: Expr,
        expr: Expr,
    },
    Return(Option<Expr>),
    ExprStmt(Expr),
    RealtimeBlock {
        clock_hz: u64,
        max_jitter_ns: u64,
        body: Vec<Stmt>,
    },
    AsyncTask {
        priority: String,
        body: Vec<Stmt>,
    },
}

/// High-Level Expressions
#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Literal(Literal),
    Ident(String),
    BinaryOp {
        op: HighBinaryOp,
        left: Box<Expr>,
        right: Box<Expr>,
    },
    UnaryOp {
        op: HighUnaryOp,
        operand: Box<Expr>,
    },
    /// Universal Machine Primitive Call: `.op.<name>(arg1, key=val)`
    PrimitiveCall {
        op_name: String,
        positional_args: Vec<Expr>,
        named_args: Vec<(String, Expr)>,
    },
    TensorAccess {
        target: Box<Expr>,
        indices: Vec<Expr>,
    },
    Tuple(Vec<Expr>),
    MethodCall {
        target: Box<Expr>,
        method: String,
        args: Vec<Expr>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    Int(i64),
    Float(f64),
    String(String),
    Bool(bool),
}

/// The 7 Beyond-Addition Multi-Physics Operators + Classical Operations
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HighBinaryOp {
    // --- The 7 Beyond-Addition Operators ---
    /// `<~>` Photonic Phase Coupling (MZI Phase Shift & Unitary Matrix Multiply)
    InterferenceCoupling,
    /// `>>=` Continuous State-Space Associative Scan (Mamba-2 SSD, 1D ODE Integration)
    AssociativeScan,
    /// `<|>` Log-Space Entropy Reduction (LogSumExp, Softmax, MoE Gating)
    LogSpaceGating,
    /// `<%>` Polynomial Crypto-Ring Reduction (FHE Modulo, ZK-ML, NTT)
    RingReduction,
    /// `/@` Radix Tree Subtree Graft & Eviction (Zero-Copy KV Cache Branching)
    RadixGraft,
    /// `~*~` Periodic Harmonic Resonance (Snake Activation, Waveform Physics)
    PeriodicResonance,
    /// `^` Clifford Exterior Wedge Product (Volume & Bivector Planes)
    ExteriorWedge,
    /// `.` Clifford Inner Scalar Product
    InnerScalar,

    // --- Classical Arithmetic ---
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Eq,
    Neq,
    Lt,
    Gt,
    Lte,
    Gte,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HighUnaryOp {
    Neg,
    Not,
    /// `~` Clifford Rotor Reversion / Inverse Transformation
    RotorReverse,
}
