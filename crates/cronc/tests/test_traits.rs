// ============================================================================
// Test: Zero-Cost Static Traits & Monomorphization (Milestone #005)
// Verifies: trait declaration, impl blocks, contract checking, monomorphized
//           codegen with zero vtable indirection on 256-core processor.
// ============================================================================

use cronc::compile_source;

#[test]
fn test_trait_declaration_and_impl() {
    let code = r#"
    .MODULE TraitMod
    struct OpticalMZIEngine {
        wavelength: f64,
        mesh_size: u32,
    }

    trait ComputeEngine {
        def forward(self, input: u32) -> u32;
        def reset(self) -> u32;
    }

    impl ComputeEngine for OpticalMZIEngine {
        def forward(self, input: u32) -> u32 {
            let result = input * 2
            return result
        }
        def reset(self) -> u32 {
            return 0
        }
    }

    _main:
        let engine = OpticalMZIEngine { wavelength: 1550.0, mesh_size: 16 }
        let output = engine.forward(42)
    .END
    "#;
    let result = compile_source(code);
    assert!(result.is_ok(), "Failed to compile trait + impl: {:?}", result.err());
    let machine_code = result.unwrap();
    // Verify machine code was generated (contains VLIW bundles)
    assert!(machine_code.contains("B0001:"), "Expected VLIW bundles in output");
}

#[test]
fn test_multiple_impl_for_different_structs() {
    let code = r#"
    .MODULE MultiImplMod
    struct PhotonicCore {
        power: u32,
    }

    struct BiologicalCore {
        spike_rate: u32,
    }

    trait NeuralCompute {
        def activate(self, signal: u32) -> u32;
    }

    impl NeuralCompute for PhotonicCore {
        def activate(self, signal: u32) -> u32 {
            let out = signal * 4
            return out
        }
    }

    impl NeuralCompute for BiologicalCore {
        def activate(self, signal: u32) -> u32 {
            let out = signal + 1
            return out
        }
    }

    _main:
        let pc = PhotonicCore { power: 100 }
        let bc = BiologicalCore { spike_rate: 50 }
        let r1 = pc.activate(10)
        let r2 = bc.activate(20)
    .END
    "#;
    let result = compile_source(code);
    assert!(result.is_ok(), "Failed to compile multiple impls: {:?}", result.err());
}

#[test]
fn test_trait_missing_method_rejected() {
    let code = r#"
    .MODULE IncompleteImplMod
    struct Incomplete {
        x: u32,
    }

    trait FullContract {
        def method_a(self) -> u32;
        def method_b(self, val: u32) -> u32;
    }

    impl FullContract for Incomplete {
        def method_a(self) -> u32 {
            return 1
        }
        // method_b is missing — should trigger E0008
    }

    _main:
        let inc = Incomplete { x: 0 }
    .END
    "#;
    let result = compile_source(code);
    assert!(result.is_err(), "Expected error for missing trait method");
    let err = result.unwrap_err();
    assert!(err.contains("E0008"), "Expected E0008 trait contract violation, got: {}", err);
    assert!(err.contains("method_b"), "Expected mention of missing method_b");
}

#[test]
fn test_monomorphized_codegen_no_vtable() {
    let code = r#"
    .MODULE MonomorphMod
    struct MZIEngine {
        size: u32,
    }

    trait Processor {
        def compute(self, x: u32) -> u32;
    }

    impl Processor for MZIEngine {
        def compute(self, x: u32) -> u32 {
            let r = x + 1
            return r
        }
    }

    _main:
        let eng = MZIEngine { size: 8 }
        let val = eng.compute(100)
    .END
    "#;
    let result = compile_source(code);
    assert!(result.is_ok(), "Failed to compile: {:?}", result.err());
    let mc = result.unwrap();
    // The machine code should NOT contain any vtable pointer indirection slots
    // (no _VT or indirect call patterns)
    assert!(!mc.contains("_VT"), "Zero-vtable violation: found _VT in machine code");
}

#[test]
fn test_method_call_on_field_access() {
    let code = r#"
    .MODULE MethodFieldMod
    struct Sensor {
        value: u32,
    }

    trait Readable {
        def read(self) -> u32;
    }

    impl Readable for Sensor {
        def read(self) -> u32 {
            return 42
        }
    }

    _main:
        let s = Sensor { value: 10 }
        let r = s.read()
    .END
    "#;
    let result = compile_source(code);
    assert!(result.is_ok(), "Failed to compile method call: {:?}", result.err());
}

#[test]
fn test_trait_with_multiple_params() {
    let code = r#"
    .MODULE MultiParamTraitMod
    struct ALU {
        width: u32,
    }

    trait Arithmetic {
        def add(self, a: u32, b: u32) -> u32;
        def mul(self, a: u32, b: u32) -> u32;
    }

    impl Arithmetic for ALU {
        def add(self, a: u32, b: u32) -> u32 {
            return a + b
        }
        def mul(self, a: u32, b: u32) -> u32 {
            return a * b
        }
    }

    _main:
        let alu = ALU { width: 32 }
        let sum = alu.add(3, 4)
        let product = alu.mul(5, 6)
    .END
    "#;
    let result = compile_source(code);
    assert!(result.is_ok(), "Failed to compile trait with multiple params: {:?}", result.err());
}
