// ============================================================================
// CRON WebAssembly Bridge (crates/cron-wasm)
// Exposes the native Rust CRON compiler (cronc), cycle-accurate 256-core
// 4D-Torus VM simulator (cron-vm), and decompiler (cron-decompile) to browsers.
// ============================================================================

#![allow(clippy::not_unsafe_ptr_arg_deref)]
#![allow(clippy::missing_const_for_thread_local)]

use std::cell::RefCell;

thread_local! {
    static LAST_RESULT: RefCell<Vec<u8>> = const { RefCell::new(Vec::new()) };
    static LAST_IS_ERR: RefCell<u32> = const { RefCell::new(0) };
}

/// Allocate memory inside the WebAssembly linear memory heap for host string transfers.
#[no_mangle]
pub extern "C" fn cron_wasm_alloc(len: usize) -> *mut u8 {
    let mut buf = Vec::with_capacity(len);
    let ptr = buf.as_mut_ptr();
    std::mem::forget(buf);
    ptr
}

/// Free memory previously allocated by `cron_wasm_alloc`.
#[no_mangle]
pub extern "C" fn cron_wasm_dealloc(ptr: *mut u8, len: usize) {
    if !ptr.is_null() && len > 0 {
        unsafe {
            let _ = Vec::from_raw_parts(ptr, 0, len);
        }
    }
}

fn set_result(content: String, is_err: bool) -> *const u8 {
    let bytes = content.into_bytes();
    let ptr = bytes.as_ptr();
    LAST_IS_ERR.with(|flag| *flag.borrow_mut() = if is_err { 1 } else { 0 });
    LAST_RESULT.with(|buf| *buf.borrow_mut() = bytes);
    ptr
}

#[no_mangle]
pub extern "C" fn cron_wasm_get_result_len() -> usize {
    LAST_RESULT.with(|buf| buf.borrow().len())
}

#[no_mangle]
pub extern "C" fn cron_wasm_get_result_ptr() -> *const u8 {
    LAST_RESULT.with(|buf| buf.borrow().as_ptr())
}

#[no_mangle]
pub extern "C" fn cron_wasm_get_is_err() -> u32 {
    LAST_IS_ERR.with(|flag| *flag.borrow())
}

/// Compile high-level CRON source (.cr) into 128-bit VLIW machine code (.cl)
/// using the native Rust `cronc` compiler.
#[no_mangle]
pub extern "C" fn cron_wasm_compile(ptr: *const u8, len: usize) -> *const u8 {
    let slice = unsafe { std::slice::from_raw_parts(ptr, len) };
    let source = match std::str::from_utf8(slice) {
        Ok(s) => s,
        Err(e) => return set_result(format!("UTF-8 Decoding Error: {}", e), true),
    };

    match cronc::compile_source(source) {
        Ok(cl) => set_result(cl, false),
        Err(err) => set_result(err, true),
    }
}

/// Compile high-level CRON source (.cr) directly into synthesizable IEEE 1364-2001
/// Verilog HDL RTL using the native Rust `cronc::compile_to_verilog`.
#[no_mangle]
pub extern "C" fn cron_wasm_compile_to_verilog(ptr: *const u8, len: usize) -> *const u8 {
    let slice = unsafe { std::slice::from_raw_parts(ptr, len) };
    let source = match std::str::from_utf8(slice) {
        Ok(s) => s,
        Err(e) => return set_result(format!("UTF-8 Decoding Error: {}", e), true),
    };

    match cronc::compile_to_verilog(source, "cksl_core") {
        Ok(v) => set_result(v, false),
        Err(err) => set_result(err, true),
    }
}

/// Decompile 128-bit VLIW machine code (.cl) back into high-level CRON (.cr)
/// using the native Rust `cron_decompile` reverse engineering engine.
#[no_mangle]
pub extern "C" fn cron_wasm_decompile(ptr: *const u8, len: usize) -> *const u8 {
    let slice = unsafe { std::slice::from_raw_parts(ptr, len) };
    let cl_code = match std::str::from_utf8(slice) {
        Ok(s) => s,
        Err(e) => return set_result(format!("UTF-8 Decoding Error: {}", e), true),
    };

    match cron_decompile::decompile_cl(cl_code) {
        Ok(cr) => set_result(cr, false),
        Err(err) => set_result(err, true),
    }
}

/// Run machine code in the cycle-accurate 256-Core 4D-Torus Hardware Simulator
/// from `cron-vm` and return execution statistics, registers, and active cores as JSON.
#[no_mangle]
pub extern "C" fn cron_wasm_run_simulation(ptr: *const u8, len: usize) -> *const u8 {
    let slice = unsafe { std::slice::from_raw_parts(ptr, len) };
    let cl_code = match std::str::from_utf8(slice) {
        Ok(s) => s,
        Err(e) => return set_result(format!("UTF-8 Decoding Error: {}", e), true),
    };

    let mut sim = cron_vm::Simulator::new();
    sim.load_machine_code(cl_code);
    sim.run();

    let regs: Vec<String> = sim.cores[0]
        .registers
        .iter()
        .map(|r| format!("0x{:08X}", r))
        .collect();
    let stats = &sim.stats;

    let mut active_cores = Vec::new();
    for core in &sim.cores {
        if core.cycle_count > 0
            || core.optical_gemm_count > 0
            || core.reversible_ops_count > 0
            || core.stdp_updates_count > 0
        {
            active_cores.push(core.id);
        }
    }
    if active_cores.is_empty() {
        active_cores.push(0);
    }

    let json = format!(
        r#"{{"total_cycles":{},"optical_gemm_ops":{},"reversible_gate_ops":{},"stdp_synapse_updates":{},"mesh_packets_routed":{},"peak_temperature_c":{},"dram_bandwidth_saved_mb":{:.2},"total_traps":{},"pred_exec_ops":{},"registers":{:?},"active_cores":{:?}}}"#,
        stats.total_cycles,
        stats.optical_gemm_ops,
        stats.reversible_gate_ops,
        stats.stdp_synapse_updates,
        stats.mesh_packets_routed,
        stats.peak_temperature_c,
        stats.dram_bandwidth_saved_mb,
        stats.total_traps,
        stats.pred_exec_ops,
        regs,
        active_cores
    );

    set_result(json, false)
}

/// Run the Esolang-Inspired AI Silicon Coprocessor simulation (Brainfuck Tape,
/// Malbolge Balanced Ternary BitNet b1.58, Befunge 2D Systolic Wavefront, Prolog Unifier)
/// and return structured JSON telemetry.
#[no_mangle]
pub extern "C" fn cron_wasm_esoteric_sim(mode_ptr: *const u8, mode_len: usize) -> *const u8 {
    let mode = if !mode_ptr.is_null() && mode_len > 0 {
        let slice = unsafe { std::slice::from_raw_parts(mode_ptr, mode_len) };
        std::str::from_utf8(slice).unwrap_or("demo")
    } else {
        "demo"
    };

    let mut coproc = cronc::EsotericCoprocessor::new();
    match mode {
        "tape" => {
            for i in 0..16 {
                coproc.tape.tape_memory[i] = (i as u32) * 0x1111;
            }
            coproc.tape.tp0 = 4;
            let _ = coproc.exec_tape_read();
            coproc.exec_tape_write(0xBEEF);
        }
        "trit" => {
            let trits = [
                cronc::Trit::Pos, cronc::Trit::Zero, cronc::Trit::Neg, cronc::Trit::Pos,
                cronc::Trit::Pos, cronc::Trit::Zero, cronc::Trit::Neg, cronc::Trit::Zero,
                cronc::Trit::Pos, cronc::Trit::Neg, cronc::Trit::Zero, cronc::Trit::Pos,
                cronc::Trit::Zero, cronc::Trit::Zero, cronc::Trit::Neg, cronc::Trit::Pos,
            ];
            let w = cronc::TritWord::from_trits(&trits);
            let acts: [i8; 16] = [12, -4, 8, 15, -2, 0, 7, -9, 10, -5, 3, 11, -8, 6, -1, 4];
            let _ = coproc.exec_trit_mac(w, &acts);
        }
        "systolic" => {
            coproc.exec_systolic_push(cronc::SystolicDirection::EastX, 0x42);
            coproc.exec_systolic_push(cronc::SystolicDirection::NorthY, 0x84);
            coproc.exec_systolic_push(cronc::SystolicDirection::WestX, 0x21);
        }
        "unify" => {
            let mut vec_a = [0xFFFFu16; 16];
            let mut vec_b = [0xFFFFu16; 16];
            vec_a[0] = 10; vec_a[1] = 25; vec_a[2] = 42; vec_a[3] = 99;
            vec_b[0] = 7;  vec_b[1] = 25; vec_b[2] = 88; vec_b[3] = 99;
            let _ = coproc.exec_unify(&vec_a, &vec_b);
        }
        _ => {
            coproc.run_demo();
        }
    }

    set_result(coproc.to_json(), false)
}

/// Synthesize synthesizable Verilog RTL for the Esoteric AI Coprocessor from WebAssembly.
#[no_mangle]
pub extern "C" fn cron_wasm_esoteric_synth() -> *const u8 {
    let rtl = cronc::synthesize_verilog_esoteric_coprocessor();
    set_result(rtl, false)
}

/// Calculate optical insertion loss and WDM laser power budget from WebAssembly.
#[no_mangle]
pub extern "C" fn cron_wasm_optic_calc(ptr: *const u8, len: usize, mesh_dim: usize, wdm_ch: usize) -> *const u8 {
    let slice = unsafe { std::slice::from_raw_parts(ptr, len) };
    let cl_code = match std::str::from_utf8(slice) {
        Ok(s) => s,
        Err(e) => return set_result(format!("UTF-8 Decoding Error: {}", e), true),
    };
    let opt = cronc::ClOpticOptions {
        mesh_dim: if mesh_dim > 0 { mesh_dim } else { 16 },
        wdm_channels: if wdm_ch > 0 { wdm_ch } else { 8 },
        ..Default::default()
    };
    match cronc::analyze_cl_optic(cl_code, &opt) {
        Ok(rep) => set_result(cronc::optic_report_to_json(&rep), false),
        Err(e) => set_result(format!("Optical Analysis Error: {}", e), true),
    }
}

/// Inspect a binary .clpatch package from WebAssembly.
#[no_mangle]
pub extern "C" fn cron_wasm_patch_inspect(ptr: *const u8, len: usize) -> *const u8 {
    let slice = unsafe { std::slice::from_raw_parts(ptr, len) };
    match cronc::ClPatchPackage::from_bytes(slice) {
        Ok(pkg) => set_result(cronc::patch_package_to_json(&pkg), false),
        Err(e) => set_result(format!("Patch Deserialization Error: {}", e), true),
    }
}

/// Version string of the native Rust CRON WebAssembly engine.
#[no_mangle]
pub extern "C" fn cron_wasm_version() -> *const u8 {
    set_result("CRON Toolchain v0.1.0 (Rust wasm32 Native Engine)".to_string(), false)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wasm_compile_and_run_roundtrip() {
        let code = r#"
        .MODULE WasmRoundtrip
        .ENTRY _main
        _main:
            let ext_hbm_base: u32 = 0x1000
            let lin w_seed: wave_t = pack_wave(amp=[1, 2], phase=[3, 4])
            export w_seed as exported_w
        .END
        "#;

        let cl_ptr = cron_wasm_compile(code.as_ptr(), code.len());
        assert_eq!(cron_wasm_get_is_err(), 0);
        let cl_len = cron_wasm_get_result_len();
        let cl_slice = unsafe { std::slice::from_raw_parts(cl_ptr, cl_len) };
        let cl_str = std::str::from_utf8(cl_slice).unwrap();
        let cl_saved = cl_str.to_string();
        assert!(cl_saved.contains("B0001:"));
        assert!(cl_saved.contains("_TL03$0"));

        // Test running simulation on the compiled CL
        let sim_ptr = cron_wasm_run_simulation(cl_saved.as_ptr(), cl_saved.len());
        assert_eq!(cron_wasm_get_is_err(), 0);
        let sim_len = cron_wasm_get_result_len();
        let sim_slice = unsafe { std::slice::from_raw_parts(sim_ptr, sim_len) };
        let sim_json = std::str::from_utf8(sim_slice).unwrap();
        assert!(sim_json.contains("total_cycles"));
        assert!(sim_json.contains("registers"));

        // Test decompilation
        let dec_ptr = cron_wasm_decompile(cl_saved.as_ptr(), cl_saved.len());
        assert_eq!(cron_wasm_get_is_err(), 0);
        let dec_len = cron_wasm_get_result_len();
        let dec_slice = unsafe { std::slice::from_raw_parts(dec_ptr, dec_len) };
        let dec_cr = std::str::from_utf8(dec_slice).unwrap();
        assert!(dec_cr.contains(".MODULE"));
    }

    #[test]
    fn test_wasm_esoteric_sim_and_synth() {
        // Test Esoteric Coprocessor WASM simulation (demo mode)
        let sim_ptr = cron_wasm_esoteric_sim(std::ptr::null(), 0);
        assert_eq!(cron_wasm_get_is_err(), 0);
        let sim_len = cron_wasm_get_result_len();
        let sim_slice = unsafe { std::slice::from_raw_parts(sim_ptr, sim_len) };
        let sim_json = std::str::from_utf8(sim_slice).unwrap();
        assert!(sim_json.contains("\"status\": \"SUCCESS\""));
        assert!(sim_json.contains("\"total_trit_macs\":"));
        assert!(sim_json.contains("\"systolic_hops\":"));

        // Test Esoteric Coprocessor WASM Verilog synthesis
        let rtl_ptr = cron_wasm_esoteric_synth();
        assert_eq!(cron_wasm_get_is_err(), 0);
        let rtl_len = cron_wasm_get_result_len();
        let rtl_slice = unsafe { std::slice::from_raw_parts(rtl_ptr, rtl_len) };
        let rtl_v = std::str::from_utf8(rtl_slice).unwrap();
        assert!(rtl_v.contains("module esoteric_coprocessor"));
        assert!(rtl_v.contains("trit_weights"));
    }

    #[test]
    fn test_wasm_optic_calc_and_patch_inspect() {
        let cl_code = "
        B0000: _OP01$28F> _NO00#000> _NO00#000> _NO00#000>
        B0001: _WD00#100> _NO00#000> _NO00#000> _HL00#000!
        ";
        let opt_ptr = cron_wasm_optic_calc(cl_code.as_ptr(), cl_code.len(), 16, 8);
        assert_eq!(cron_wasm_get_is_err(), 0);
        let opt_len = cron_wasm_get_result_len();
        let opt_slice = unsafe { std::slice::from_raw_parts(opt_ptr, opt_len) };
        let opt_json = std::str::from_utf8(opt_slice).unwrap();
        assert!(opt_json.contains("\"status\": \"COMPLIANT\""));
        assert!(opt_json.contains("\"insertion_loss_db\":"));

        // Test patch inspect from bytes
        let mut pkg = cronc::ClPatchPackage::new("TORUS_256_REV_A");
        pkg.add_entry(cronc::MicrocodePatchEntry {
            entry_id: 0,
            target_cycle: 1,
            target_core_id: None,
            action: cronc::PatchAction::ReplaceBundle,
            replacement_bundle_raw: "_OP01$28F> _NO00#000> _NO00#000> _NO00#000>".to_string(),
            enabled: true,
            comment: "WASM test".to_string(),
        }).unwrap();
        let bytes = pkg.to_bytes();
        let patch_ptr = cron_wasm_patch_inspect(bytes.as_ptr(), bytes.len());
        assert_eq!(cron_wasm_get_is_err(), 0);
        let patch_len = cron_wasm_get_result_len();
        let patch_slice = unsafe { std::slice::from_raw_parts(patch_ptr, patch_len) };
        let patch_json = std::str::from_utf8(patch_slice).unwrap();
        assert!(patch_json.contains("\"silicon_rev\": \"TORUS_256_REV_A\""));
    }
}

