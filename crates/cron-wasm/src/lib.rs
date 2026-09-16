// ============================================================================
// CRON WebAssembly Bridge (crates/cron-wasm)
// Exposes the native Rust CRON compiler (cronc), cycle-accurate 256-core
// 4D-Torus VM simulator (cron-vm), and decompiler (cron-decompile) to browsers.
// ============================================================================

#![allow(clippy::not_unsafe_ptr_arg_deref)]

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
}
