#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // Directives
    ModuleDir, // .MODULE
    EntryDir,  // .ENTRY
    EndDir,    // .END

    // Keywords
    Async,
    Def,
    Let,
    Lin,
    Grad,
    Region,
    ResilientCompute,
    Fallback,
    Return,
    Export,
    As,
    Await,
    Spawn,
    Consume,
    Import,
    From,
    Dollar,
    If,
    Else,
    While,
    For,
    In,
    True,
    False,
    And,
    Or,
    Not,

    ProofContract,
    Invariant,
    Ensures,

    Struct,
    Enum,
    Match,
    Type,
    Mut,
    Inline,
    Module,
    Trait,
    Impl,

    // Cognitive DSL Keywords
    Brain,
    Fork,
    Simulate,
    Abort,
    Then,
    With,

    // Identifiers & Literals
    Ident(String),
    IntLit(i64),
    HexLit(u64),
    FloatLit(f64),
    StringLit(String),

    // Axes and Special Identifiers
    Axis(String), // X+, X-, Y+, Y-, Z+, Z-, W+, W-

    // Delimiters & Operators
    OpenParen,    // (
    CloseParen,   // )
    OpenBrace,    // {
    CloseBrace,   // }
    OpenBracket,  // [
    CloseBracket, // ]
    Comma,        // ,
    Colon,        // :
    Arrow,        // ->
    FatArrow,     // =>
    Assign,       // =
    Underscore,   // _
    Plus,         // +
    Minus,        // -
    Star,         // *
    Slash,        // /
    Percent,      // %
    Dot,          // .
    DotDot,       // ..
    Less,         // <
    LessEqual,    // <=
    Shl,          // <<
    Greater,      // >
    GreaterEqual, // >=
    Shr,          // >>
    EqualEqual,   // ==
    NotEqual,     // !=
    Bang,         // !
    Amp,          // &
    AmpAmp,       // &&
    Pipe,         // |
    PipePipe,     // ||
    Caret,        // ^
    Semicolon,    // ;
    PlusAssign,   // +=
    MinusAssign,  // -=
    StarAssign,   // *=
    SlashAssign,  // /=
    Eof,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Span {
    pub line: usize,
    pub col: usize,
    pub byte_offset: usize,
    pub len: usize,
}

impl Span {
    pub fn new(line: usize, col: usize, byte_offset: usize, len: usize) -> Self {
        Self {
            line,
            col,
            byte_offset,
            len,
        }
    }

    pub fn point(line: usize, col: usize, byte_offset: usize) -> Self {
        Self {
            line,
            col,
            byte_offset,
            len: 1,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Spanned<T> {
    pub value: T,
    pub span: Span,
}

impl<T> Spanned<T> {
    pub fn new(value: T, span: Span) -> Self {
        Self { value, span }
    }
}

