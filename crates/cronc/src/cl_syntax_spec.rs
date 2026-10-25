// ============================================================================
// CRON CL v2.0 SOVEREIGN PURE-CIPHER SYNTAX SPECIFICATION & CONFORMANCE VERIFIER
// Module: cronc::cl_syntax_spec (crates/cronc/src/cl_syntax_spec.rs)
//
// Formal Non-Verbal Silicon Mathematical Language Specification:
// ZERO-VERBAL CONSTITUTION:
// 1. Prohibits all natural language English keywords ('kernel', 'module', 'fn', 'let', 'query', etc.)
// 2. Strict 5-Pillar Symbolic Form:
//    - Pillar 1 (Silicon Topology):       @⌖ [D₁×D₂×...×Dₖ] : ⮑ (x+1, y, z, w)
//    - Pillar 2 (I/O SRAM/DRAM Map):      @io { X[16×16:FP32] @0x0000, Y[16×16:FP32] @0x0200 }
//    - Pillar 3 (Anonymous Tile Morphism): §[16×16:FP32] { X ↦ Y : S := X ⊗ W, Y := σ(∥S∥) }
//    - Pillar 4 (Zero-Overhead Loops/!Δ): ⟳ (N) { #R1 ← @0x0000, ... } and !Δ { ∥Y∥ < 1e5 }
//    - Pillar 5 (128-bit VLIW Cipher):    B####: [Slot0 Slot1 Slot2 Slot3]
// ============================================================================


pub const SOVEREIGN_SYNTAX_EBNF: &str = r##"
(* ============================================================================
   SOVEREIGN CL v2.0 POST-HUMAN FORMAL EBNF GRAMMAR
   Zero Natural Language Words. Pure Tensor-Silicon Cipher.
   ============================================================================ *)

Program             ::= [ TopologyHeader ] [ IoBlock ] [ MathBlock ] [ ControlBlock ] [ Invariant ] [ VliwStream ] ;

TopologyHeader      ::= "@⌖" DimensionVector ":" "⮑" RoutingFormula ;
DimensionVector     ::= "[" Integer { "×" Integer } "]" ;
RoutingFormula      ::= "(" Identifier { "," Identifier } ")" ;

IoBlock             ::= "@io" "{" { IoBindingEntry } "}" ;
IoBindingEntry      ::= Symbol "[" TensorDimensions ":" DataType "]" Address ;
TensorDimensions    ::= Integer { "×" Integer } ;
DataType            ::= "FP32" | "FP16" | "BF16" | "NVFP4" | "INT8" | "INT32" | "T" ;
Address             ::= "@" [ "sram" | "dram" | "hbm" ] "[" HexLiteral "]" | "@" HexLiteral ;

MathBlock           ::= "§" DimensionVector "{" { StateDecl } MorphismEquation "}" ;
StateDecl           ::= Symbol ":" [ "SS" | "T" ] DimensionVector ;
MorphismEquation    ::= Symbol "↦" Symbol ":" { MathStep } ;
MathStep            ::= Symbol ":=" MathExpr ;

MathExpr            ::= MathTerm { ( "+" | "-" ) MathTerm } ;
MathTerm            ::= MathFactor { ( "*" | "/" | "%" | "⊗" | "<~>" | ">>=" | "<|>" | "<%>" | "/@" | "~*~" | "^" | "." ) MathFactor } ;
MathFactor          ::= [ "-" | "~" ] ( Symbol | FloatLiteral | ActivationCall | NormCall | "(" MathExpr ")" ) ;
ActivationCall      ::= "σ" "(" MathExpr ")" ;
NormCall            ::= "∥" MathExpr "∥" ;

ControlBlock        ::= "⟳" "(" Integer ")" "{" { SiliconStmt } "}" ;
SiliconStmt         ::= RegTransfer | RegMath | RegBranch ;
RegTransfer         ::= Register "←" Address | Address "←" Register ;
RegMath             ::= Register ":=" Register ( "+" | "-" | "*" | "⊗" ) Register ;
RegBranch           ::= "?" "(" Register ")" "⮞" HexLiteral ;

Invariant           ::= "!Δ" "{" InvariantExpr { "," InvariantExpr } "}" ;
InvariantExpr       ::= ( "∥" Symbol "∥" | "|" Symbol "|" ) ( "<" | ">" | "≤" | "≥" ) FloatLiteral ;

VliwStream          ::= { BundleLine } ;
BundleLine          ::= "B" HexLiteral4 ":" Slot Slot Slot Slot ;
Slot                ::= SlotPrefix Opcode DestReg Mode Src1 Src2 ImmNibble CrcToken Terminator ;

Symbol              ::= UpperCaseLetter | GreekLetter ;
Register            ::= "#R" Integer ;
HexLiteral          ::= "0x" { HexDigit } ;
HexLiteral4         ::= HexDigit HexDigit HexDigit HexDigit ;
Integer             ::= Digit { Digit } ;
FloatLiteral        ::= Digit { Digit } [ "." Digit { Digit } ] [ ( "e" | "E" ) [ "+" | "-" ] Digit { Digit } ] ;
"##;

#[derive(Debug, Clone, PartialEq)]
pub struct SyntaxViolation {
    pub line: usize,
    pub token: String,
    pub reason: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SovereignSyntaxReport {
    pub is_pure_cipher: bool,
    pub has_topology: bool,
    pub has_io_binding: bool,
    pub has_math_morphism: bool,
    pub has_control_flow: bool,
    pub has_invariant: bool,
    pub has_vliw: bool,
    pub violations: Vec<SyntaxViolation>,
}

/// Blacklisted English Verbal Keywords that violate the Sovereign Non-Verbal Constitution
const FORBIDDEN_VERBAL_KEYWORDS: &[&str] = &[
    "kernel", "target", "ipc_target", "cores", "module", "import", "stage",
    "arch", "state", "fn", "function", "let", "var", "def", "class", "struct",
    "return", "as", "query", "entry_node", "out_node", "controlflow", "block",
    "weights", "input", "output", "delta", "sqdist", "proximity", "nearestneighbor",
    "currentcentroid", "queryvector", "rmsnorm", "silu", "gelu", "relu",
    "sigmoid", "softmax", "layer", "forward", "backward"
];

/// Legal Allowed Type Tags & Registered Symbols
pub fn is_allowed_type_tag(s: &str) -> bool {
    matches!(s, "FP32" | "FP16" | "BF16" | "NVFP4" | "INT8" | "INT16" | "INT32" | "INT64" | "UINT8" | "UINT32" | "T" | "SS")
}

/// Verify if a CL source code conforms to the Pure Sovereign Non-Verbal Syntax Standard
pub fn verify_sovereign_pure_cipher_syntax(source: &str) -> Result<SovereignSyntaxReport, SovereignSyntaxReport> {
    let mut violations = Vec::new();
    let mut has_topology = false;
    let mut has_io_binding = false;
    let mut has_math_morphism = false;
    let mut has_control_flow = false;
    let mut has_invariant = false;
    let mut has_vliw = false;

    let lines: Vec<&str> = source.lines().collect();
    for (line_idx, raw_line) in lines.iter().enumerate() {
        let line_num = line_idx + 1;
        let trimmed = raw_line.trim();

        // Skip comments and blank lines
        if trimmed.is_empty() || trimmed.starts_with("//") || trimmed.starts_with('#') {
            continue;
        }

        // Structural Pillar Detection
        if trimmed.starts_with("@⌖") {
            has_topology = true;
        }
        if trimmed.starts_with("@io") {
            has_io_binding = true;
        }
        if trimmed.starts_with('§') {
            has_math_morphism = true;
            // Verify section is anonymous tile e.g. §[16×16:FP32], not named §NamedBlock
            let section_head = trimmed.trim_start_matches('§').trim();
            if !section_head.starts_with('[') && !section_head.is_empty() {
                let name_part = section_head.split('[').next().unwrap_or("").trim();
                if !name_part.is_empty() && name_part.len() > 2 {
                    violations.push(SyntaxViolation {
                        line: line_num,
                        token: name_part.to_string(),
                        reason: format!(
                            "Named section '§{}' violates pure cipher standard. Must use anonymous tensor tile '§[Dims:Type]'.",
                            name_part
                        ),
                    });
                }
            }
        }
        if trimmed.starts_with('⟳') {
            has_control_flow = true;
        }
        if trimmed.starts_with("!Δ") {
            has_invariant = true;
        }
        if trimmed.starts_with('B') && trimmed.find(':').map_or(false, |p| p >= 2 && p <= 5) {
            has_vliw = true;
        }

        // Check for forbidden verbal keywords
        let lower = trimmed.to_lowercase();
        for &kw in FORBIDDEN_VERBAL_KEYWORDS {
            // Check word boundary or directive
            let matches_directive = lower.contains(&format!("@{}", kw)) || lower.contains(&format!(".{}", kw));
            let matches_colon = lower.contains(&format!("{}:", kw));
            let matches_call = lower.contains(&format!("{}(", kw));
            let matches_word = lower.split(|c: char| !c.is_alphanumeric() && c != '_').any(|word| word == kw);

            if matches_directive || matches_colon || matches_call || matches_word {
                violations.push(SyntaxViolation {
                    line: line_num,
                    token: kw.to_string(),
                    reason: format!(
                        "Forbidden English verbal token '{}' detected. Sovereign CL v2.0 requires pure mathematical cipher.",
                        kw
                    ),
                });
            }
        }

        // Check variable length in morphism equations (must be pure mathematical symbols, length <= 2)
        if trimmed.contains(":=") {
            let parts: Vec<&str> = trimmed.split(":=").collect();
            if parts.len() == 2 {
                let lhs = parts[0].trim();
                // Valid symbols: single capital letter A-Z or register #R1 or Greek letter
                if lhs.len() > 2 && !lhs.starts_with("#R") && !lhs.starts_with('@') {
                    violations.push(SyntaxViolation {
                        line: line_num,
                        token: lhs.to_string(),
                        reason: format!(
                            "Multi-character verbal variable '{}' in assignment. Must use pure mathematical tensor symbol (e.g., 'Y', 'S', 'D').",
                            lhs
                        ),
                    });
                }
            }
        }
    }

    let report = SovereignSyntaxReport {
        is_pure_cipher: violations.is_empty(),
        has_topology,
        has_io_binding,
        has_math_morphism,
        has_control_flow,
        has_invariant,
        has_vliw,
        violations,
    };

    if report.is_pure_cipher {
        Ok(report)
    } else {
        Err(report)
    }
}
