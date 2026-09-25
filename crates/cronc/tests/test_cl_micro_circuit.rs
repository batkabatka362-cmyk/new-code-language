use cronc::cl_lang::{verify_cl_program, ClDirective};

#[test]
fn test_cl_circuit_directive_parsing() {
    let cl_code = r#"
.stage id="neocortex" params="7B" precision="ternary_1.58b" d_model=4096 heads=32 kv_heads=8 intermediate=11008 zero_overhead=true
.circuit core=0 layer=L4 exc=90 inh=10 tau=10000 da=0.60
.circuit core=1 layer=L23 exc=80 inh=20 tau=15000 da=0.75
.circuit core=2 layer=L5 exc=85 inh=15 tau=25000 da=0.80
.circuit core=3 layer=L6 exc=75 inh=25 tau=30000 da=0.50

@entry:
B0000: _NO00#070> _NO00#070> _NO00#070> _NO00#070>
B0001: _HL00$0E8! _NO00#070> _NO00#070> _NO00#070>
"#;

    let report = verify_cl_program(cl_code).expect("Should verify cleanly");
    assert_eq!(report.circuits_configured, 4);

    let circuits: Vec<&ClDirective> = report
        .directives_parsed
        .iter()
        .filter(|d| matches!(d, ClDirective::Circuit { .. }))
        .collect();

    assert_eq!(circuits.len(), 4);
    if let ClDirective::Circuit { core_id, layer, excitatory, inhibitory, tau_us, dopamine } = circuits[1] {
        assert_eq!(*core_id, 1);
        assert_eq!(layer, "L23");
        assert_eq!(*excitatory, 80);
        assert_eq!(*inhibitory, 20);
        assert_eq!(*tau_us, 15000);
        assert!((*dopamine - 0.75).abs() < 1e-4);
    } else {
        panic!("Expected Circuit directive");
    }
}
