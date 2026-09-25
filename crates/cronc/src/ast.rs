// ============================================================================
// CRON Abstract Syntax Tree (AST)
// Complete representation of both .cr high-level and .cl low-level structures.
// Target: 256-Core 4D-Torus Neuromorphic Photonic Processor
// ============================================================================

use crate::token::Span;

#[derive(Debug, Clone, PartialEq)]
pub struct Program {
    pub module_name: String,
    pub entry_name: Option<String>,
    pub imports: Vec<ImportDecl>,
    pub type_aliases: Vec<TypeAliasDecl>,
    pub structs: Vec<StructDecl>,
    pub enums: Vec<EnumDecl>,
    pub traits: Vec<TraitDecl>,
    pub impls: Vec<ImplDecl>,
    pub functions: Vec<FunctionDecl>,
    pub schedules: Vec<ScheduleDecl>,
    pub brains: Vec<BrainDecl>,
    pub main_statements: Vec<Statement>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ScheduleDecl {
    pub target_fn: String,
    pub target_arch: String,
    pub directives: Vec<ScheduleDirective>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ScheduleDirective {
    TileSize(usize, usize),
    PrefetchTo(String),
    Unroll(usize),
    Distribute4D { axis: String, cores: usize },
    Vectorize(usize),
    Autotune {
        tile_sizes: Vec<(usize, usize)>,
        unrolls: Vec<usize>,
        vectorize_widths: Vec<usize>,
        metric: String,
    },
    Custom { name: String, args: Vec<String> },
}

#[derive(Debug, Clone, PartialEq)]
pub struct EnumDecl {
    pub name: String,
    pub generic_params: Vec<String>,
    pub variants: Vec<EnumVariant>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct EnumVariant {
    pub name: String,
    pub payload: Option<Vec<String>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ImportDecl {
    pub items: Vec<String>,
    pub path: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TypeAliasDecl {
    pub name: String,
    pub target_type: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct StructDecl {
    pub name: String,
    pub generic_params: Vec<String>,
    pub fields: Vec<(String, String)>,
}

/// Zero-vtable trait declaration (Milestone #005: no dynamic dispatch on 256-core processor)
#[derive(Debug, Clone, PartialEq)]
pub struct TraitDecl {
    pub name: String,
    pub generic_params: Vec<String>,
    pub methods: Vec<TraitMethodSig>,
}

/// Trait method signature (body-less declaration)
#[derive(Debug, Clone, PartialEq)]
pub struct TraitMethodSig {
    pub name: String,
    pub params: Vec<Param>,
    pub return_type: Option<String>,
}

/// Monomorphized impl block — maps trait methods to concrete struct implementations
#[derive(Debug, Clone, PartialEq)]
pub struct ImplDecl {
    pub trait_name: String,
    pub target_struct: String,
    pub generic_params: Vec<String>,
    pub methods: Vec<FunctionDecl>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionDecl {
    pub is_async: bool,
    pub is_export: bool,
    pub is_inline: bool,
    pub name: String,
    pub generic_params: Vec<String>,
    pub params: Vec<Param>,
    pub return_type: Option<String>,
    pub body: Vec<Statement>,
}

/// Cognitive DSL: High-level Brain specification (Milestone: Cognitive DSL / Page 320)
#[derive(Debug, Clone, PartialEq)]
pub struct BrainDecl {
    pub name: String,
    pub attrs: Vec<(String, String)>,
    pub body: Vec<Statement>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Param {
    pub is_lin: bool,
    pub is_grad: bool,
    pub name: String,
    pub param_type: String,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    Let {
        is_lin: bool,
        is_grad: bool,
        is_mut: bool,
        name: String,
        type_annot: Option<String>,
        value: Expr,
        extra_vars: Vec<(bool, bool, String, Option<String>)>,
        span: Span,
    },
    Assign {
        target: String,
        value: Expr,
        span: Span,
    },
    Region {
        name: String,
        attrs: Vec<(String, String)>,
        body: Vec<Statement>,
        span: Span,
    },
    Resilient {
        attrs: Vec<(String, String)>,
        body: Vec<Statement>,
        fallback: Option<Vec<Statement>>,
        span: Span,
    },
    /// Automated Kernel Fusion & Zero-Allocation Streaming Block (Milestone #012)
    Fuse {
        attrs: Vec<(String, String)>,
        body: Vec<Statement>,
        span: Span,
    },
    If {
        condition: Expr,
        then_body: Vec<Statement>,
        else_body: Option<Vec<Statement>>,
        span: Span,
    },
    While {
        condition: Expr,
        body: Vec<Statement>,
        span: Span,
    },
    For {
        var_name: String,
        iterable: Expr,
        body: Vec<Statement>,
        span: Span,
    },
    Superposition {
        params: Vec<CallArg>,
        branches: Vec<SuperpositionBranch>,
        collapse_args: Option<Vec<CallArg>>,
        span: Span,
    },
    ProofContract(Vec<Statement>),
    Return(Option<Expr>),
    Export {
        source_name: String,
        exported_name: String,
        span: Span,
    },
    Brain {
        name: String,
        attrs: Vec<(String, String)>,
        body: Vec<Statement>,
        span: Span,
    },
    Fork {
        target: Expr,
        span: Span,
    },
    Simulate {
        action: Expr,
        with_arg: Option<Expr>,
        span: Span,
    },
    Abort(Span),
    Expr(Expr),
    Match {
        expr: Expr,
        arms: Vec<MatchArm>,
        span: Span,
    },
    /// Compile-Time Metaprogramming execution block
    Comptime {
        body: Vec<Statement>,
        span: Span,
    },
    // Milestone #022: Esolang-Inspired Language Statements
    /// Assembly: Inline 4-way VLIW slot machine assembly block
    InlineVliw {
        raw_bundles: Vec<String>,
        span: Span,
    },
    /// Brainfuck: Hardware auto-advancing tape ring buffer declaration
    TapeDecl {
        name: String,
        elem_type: String,
        capacity: usize,
        span: Span,
    },
    /// Brainfuck: Tape stream write (tape << val) or stream read (val = >> tape)
    TapeStream {
        target_tape: String,
        value: Expr,
        is_read: bool,
        span: Span,
    },
    /// Befunge: Spatial 2D/4D wavefront systolic array block
    SystolicBlock {
        mesh_rows: usize,
        mesh_cols: usize,
        topology: String,
        flows: Vec<SystolicFlowDecl>,
        body: Vec<Statement>,
        span: Span,
    },
    /// Prolog: Horn-clause neuro-symbolic rule declaration
    RuleDecl {
        name: String,
        head_params: Vec<String>,
        body_exprs: Vec<Expr>,
        span: Span,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub struct SystolicFlowDecl {
    pub tensor_name: String,
    pub direction: String,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MatchArm {
    pub pattern: MatchPattern,
    pub guard: Option<Expr>,
    pub body: Vec<Statement>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub enum MatchPattern {
    Variant {
        enum_name: Option<String>,
        variant_name: String,
        bindings: Vec<String>,
    },
    Literal(Expr),
    Tuple(Vec<MatchPattern>),
    Or(Vec<MatchPattern>),
    Wildcard,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SuperpositionBranch {
    pub name: String,
    pub body: Vec<Statement>,
}

impl Statement {
    pub fn span(&self) -> Span {
        match self {
            Statement::Let { span, .. } => *span,
            Statement::Assign { span, .. } => *span,
            Statement::Region { span, .. } => *span,
            Statement::Resilient { span, .. } => *span,
            Statement::Fuse { span, .. } => *span,
            Statement::If { span, .. } => *span,
            Statement::While { span, .. } => *span,
            Statement::For { span, .. } => *span,
            Statement::Superposition { span, .. } => *span,
            Statement::Export { span, .. } => *span,
            Statement::Brain { span, .. } => *span,
            Statement::Fork { span, .. } => *span,
            Statement::Simulate { span, .. } => *span,
            Statement::Abort(span) => *span,
            Statement::ProofContract(_) => Span::default(),
            Statement::Return(_) => Span::default(),
            Statement::Expr(e) => e.span(),
            Statement::Match { span, .. } => *span,
            Statement::Comptime { span, .. } => *span,
            Statement::InlineVliw { span, .. } => *span,
            Statement::TapeDecl { span, .. } => *span,
            Statement::TapeStream { span, .. } => *span,
            Statement::SystolicBlock { span, .. } => *span,
            Statement::RuleDecl { span, .. } => *span,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    LiteralInt(i64),
    LiteralHex(u64),
    LiteralFloat(f64),
    LiteralAxis(String),
    LiteralString(String),
    LiteralBool(bool),
    Ident(String, Span),
    Array(Vec<Expr>),
    ArrayRepeat {
        value: Box<Expr>,
        count: Box<Expr>,
    },
    Tuple(Vec<Expr>),
    StructInit {
        struct_name: String,
        fields: Vec<(String, Expr)>,
    },
    IfExpr {
        condition: Box<Expr>,
        then_branch: Box<Expr>,
        else_branch: Box<Expr>,
    },
    Unary {
        op: String,
        operand: Box<Expr>,
    },
    Binary {
        op: String,
        left: Box<Expr>,
        right: Box<Expr>,
    },
    Cast {
        expr: Box<Expr>,
        target_type: String,
    },
    Call {
        callee: String,
        args: Vec<CallArg>,
    },
    /// Monomorphized method call: object.method(args) resolved at compile-time
    MethodCall {
        object: Box<Expr>,
        method: String,
        args: Vec<CallArg>,
    },
    FieldAccess {
        object: Box<Expr>,
        field: String,
    },
    Index {
        object: Box<Expr>,
        index: Box<Expr>,
    },
    Consume(String, Span),
    Await(Box<Expr>),
    Spawn(Box<Expr>),
    SpawnAt {
        core_id: Box<Expr>,
        target: Box<Expr>,
        span: Span,
    },
    ChannelSend {
        channel: Box<Expr>,
        value: Box<Expr>,
        span: Span,
    },
    ChannelRecv {
        channel: Box<Expr>,
        span: Span,
    },
    /// Language-level automatic differentiation: grad(f) or grad(f, wrt: "x")
    Grad {
        callee: Box<Expr>,
        wrt: Option<String>,
        span: Span,
    },
    /// Direct evaluation of a differentiated function: grad(f)(x, y, ...)
    GradCall {
        callee: Box<Expr>,
        wrt: Option<String>,
        args: Vec<CallArg>,
        span: Span,
    },
    /// Compile-time metaprogramming block: comptime { ... }
    Comptime {
        body: Vec<Statement>,
        result: Option<Box<Expr>>,
        span: Span,
    },
    /// Immutable reference expression: &expr
    Ref(Box<Expr>),
    /// Mutable reference expression: &mut expr
    RefMut(Box<Expr>),
    /// Pattern match expression: match target { arm => val, ... }
    Match {
        expr: Box<Expr>,
        arms: Vec<MatchArm>,
        span: Span,
    },
    /// First-class zero-vtable closure: |params| -> body
    Closure {
        params: Vec<Param>,
        return_type: Option<String>,
        body: Box<Expr>,
        span: Span,
    },
}

impl Expr {
    pub fn span(&self) -> Span {
        match self {
            Expr::Ident(_, span) => *span,
            Expr::Consume(_, span) => *span,
            Expr::SpawnAt { span, .. } => *span,
            Expr::ChannelSend { span, .. } => *span,
            Expr::ChannelRecv { span, .. } => *span,
            Expr::Grad { span, .. } => *span,
            Expr::GradCall { span, .. } => *span,
            Expr::Comptime { span, .. } => *span,
            Expr::Ref(inner) | Expr::RefMut(inner) => inner.span(),
            Expr::Match { span, .. } => *span,
            Expr::Closure { span, .. } => *span,
            _ => Span::default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct CallArg {
    pub name: Option<String>,
    pub value: Expr,
}
