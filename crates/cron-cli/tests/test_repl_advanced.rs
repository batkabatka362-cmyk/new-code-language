// ============================================================================
// CRON SSS+ Cognitive REPL Advanced Integration Test Suite
// Validates:
//   - Cumulative variable environment persistence
//   - Multi-line block evaluation
//   - AST inspection (:ast)
//   - Lexer token streaming (:tokens)
//   - Static type inference (:type)
//   - 128-bit VLIW 4-slot bundle disassembly (:disasm, :slots)
//   - 4D-Torus spatial slice (:torus) & 8D Wafer HUD (:wafer)
//   - External .cr file loading (:load)
//   - Direct VLIW injection
// ============================================================================

use cron_cli::repl::ReplSession;

#[test]
fn test_repl_basic_arithmetic_evaluation() {
    let mut session = ReplSession::new();
    assert!(session.eval_line("10 + 25"));
    assert!(session.last_cl.is_some());
    assert!(session.step_count > 0);
}

#[test]
fn test_repl_variable_binding_chain() {
    let mut session = ReplSession::new();
    assert!(session.eval_line("let a = 100"));
    assert!(session.eval_line("let b = 50"));
    assert!(session.eval_line("let c = a - b"));
    assert_eq!(session.statements.len(), 3);
    assert!(session.eval_line("c * 2"));
    assert!(session.last_cl.is_some());
}

#[test]
fn test_repl_function_declaration_and_call() {
    let mut session = ReplSession::new();
    assert!(session.eval_line("def multiply(x: u32, y: u32) -> u32 { return x * y }"));
    assert_eq!(session.declarations.len(), 1);
    assert!(session.eval_line("multiply(6, 7)"));
    assert!(session.last_cl.is_some());
}

#[test]
fn test_repl_struct_declaration() {
    let mut session = ReplSession::new();
    assert!(session.eval_line("struct Point { x: u32, y: u32 }"));
    assert_eq!(session.declarations.len(), 1);
    assert!(session.eval_line("let p = Point { x: 12, y: 34 }"));
    assert_eq!(session.statements.len(), 1);
}

#[test]
fn test_repl_colon_inspections() {
    let mut session = ReplSession::new();
    // Test AST inspector
    assert!(session.eval_line(":ast let x = 42 + 8"));
    // Test Token stream inspector
    assert!(session.eval_line(":tokens let x = 42 + 8"));
    // Test Type inspector
    assert!(session.eval_line(":type 42 + 8"));
    assert!(session.eval_line(":type [1, 2, 3, 4]"));
    assert!(session.eval_line(":type \"hello cron\""));
    assert!(session.eval_line(":type true"));
}

#[test]
fn test_repl_hardware_telemetry_commands() {
    let mut session = ReplSession::new();
    assert!(session.eval_line("let v = 1234"));
    assert!(session.eval_line(":regs"));
    assert!(session.eval_line(":mesh"));
    assert!(session.eval_line(":torus"));
    assert!(session.eval_line(":wafer"));
    assert!(session.eval_line(":cl"));
    assert!(session.eval_line(":disasm"));
    assert!(session.eval_line(":slots"));
}

#[test]
fn test_repl_load_external_cr_file() {
    let mut session = ReplSession::new();
    assert!(session.eval_line(":load examples/jit_fibonacci.cr"));
    assert!(!session.declarations.is_empty());
}

#[test]
fn test_repl_direct_vliw_execution() {
    let mut session = ReplSession::new();
    // VLIW bundle with immediate load 0x0A04 to bank 0, reg 0
    let vliw = "B0000: '=00#0A04> _NO00#000> _NO00#000> _NO00#000!";
    assert!(session.eval_line(vliw));
    assert_eq!(session.sim.cores[0].registers[0], 0x0A04);
}

#[test]
fn test_repl_reset_flow() {
    let mut session = ReplSession::new();
    assert!(session.eval_line("let x = 999"));
    assert_eq!(session.statements.len(), 1);
    assert!(session.eval_line(":reset"));
    assert_eq!(session.statements.len(), 0);
    assert_eq!(session.declarations.len(), 0);
    assert_eq!(session.sim.cores[0].registers[0], 0);
}

#[test]
fn test_repl_exit_commands() {
    let mut session = ReplSession::new();
    assert!(!session.eval_line(":quit"));
    assert!(!session.eval_line(":exit"));
    assert!(!session.eval_line(":q"));
}
