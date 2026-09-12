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
    pub traits: Vec<TraitDecl>,
    pub impls: Vec<ImplDecl>,
    pub functions: Vec<FunctionDecl>,
    pub brains: Vec<BrainDecl>,
    pub main_statements: Vec<Statement>,
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
    pub fields: Vec<(String, String)>,
}

/// Zero-vtable trait declaration (Milestone #005: no dynamic dispatch on 256-core processor)
#[derive(Debug, Clone, PartialEq)]
pub struct TraitDecl {
    pub name: String,
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
    pub methods: Vec<FunctionDecl>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionDecl {
    pub is_async: bool,
    pub is_export: bool,
    pub is_inline: bool,
    pub name: String,
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
}

impl Expr {
    pub fn span(&self) -> Span {
        match self {
            Expr::Ident(_, span) => *span,
            Expr::Consume(_, span) => *span,
            _ => Span::default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct CallArg {
    pub name: Option<String>,
    pub value: Expr,
}
