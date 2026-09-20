use cronc::{
    compute_magnitude_angle, compute_rope_frequencies, compute_sin_cos,
    compute_sinh_cosh, render_ascii_bloch_sphere, render_ascii_phase_orbit,
    run_cordic, synthesize_cordic_cl, verify_cl_program, run_cl_jit,
    CordicConfig, CordicMode,
};
use std::f64::consts::{FRAC_PI_2, FRAC_PI_3, FRAC_PI_4, FRAC_PI_6, PI};

#[test]
fn test_cordic_sin_cos_accuracy() {
    let test_angles = [
        0.0,
        FRAC_PI_6,
        FRAC_PI_4,
        FRAC_PI_3,
        FRAC_PI_2,
        PI * 0.75,
        PI,
        -FRAC_PI_4,
        -FRAC_PI_2,
    ];

    for &theta in &test_angles {
        let (s, c) = compute_sin_cos(theta, 20);
        let expected_s = theta.sin();
        let expected_c = theta.cos();

        let err_s = (s - expected_s).abs();
        let err_c = (c - expected_c).abs();

        assert!(
            err_s < 1e-3,
            "sin error too large for theta={}: got {}, expected {}, err={}",
            theta, s, expected_s, err_s
        );
        assert!(
            err_c < 1e-3,
            "cos error too large for theta={}: got {}, expected {}, err={}",
            theta, c, expected_c, err_c
        );
    }
}

#[test]
fn test_cordic_vectoring_magnitude_and_angle() {
    // 3-4-5 right triangle
    let (r, theta) = compute_magnitude_angle(3.0, 4.0, 20);
    let expected_r = 5.0;
    let expected_theta = 4.0f64.atan2(3.0);

    assert!(
        (r - expected_r).abs() < 1e-3,
        "Magnitude error: got {}, expected {}",
        r, expected_r
    );
    assert!(
        (theta - expected_theta).abs() < 1e-3,
        "Angle error: got {}, expected {}",
        theta, expected_theta
    );

    // Negative coordinates (quadrant 2)
    let (r2, theta2) = compute_magnitude_angle(-1.0, 1.0, 20);
    let expected_r2 = std::f64::consts::SQRT_2;
    let expected_theta2 = 1.0f64.atan2(-1.0);

    assert!(
        (r2 - expected_r2).abs() < 1e-3,
        "Quadrant 2 magnitude error: got {}, expected {}",
        r2, expected_r2
    );
    assert!(
        (theta2 - expected_theta2).abs() < 1e-3,
        "Quadrant 2 angle error: got {}, expected {}",
        theta2, expected_theta2
    );
}

#[test]
fn test_cordic_hyperbolic_walther() {
    let val = 0.5;
    let (sinh_val, cosh_val) = compute_sinh_cosh(val, 16);
    let expected_sinh = val.sinh();
    let expected_cosh = val.cosh();

    assert!(
        (sinh_val - expected_sinh).abs() < 1e-2,
        "Hyperbolic sinh error: got {}, expected {}",
        sinh_val, expected_sinh
    );
    assert!(
        (cosh_val - expected_cosh).abs() < 1e-2,
        "Hyperbolic cosh error: got {}, expected {}",
        cosh_val, expected_cosh
    );

    // Fundamental hyperbolic identity: cosh^2 - sinh^2 == 1
    let identity = cosh_val * cosh_val - sinh_val * sinh_val;
    assert!(
        (identity - 1.0).abs() < 2e-2,
        "cosh^2 - sinh^2 identity violated: got {}",
        identity
    );
}

#[test]
fn test_cordic_rope_frequencies() {
    let dim = 64;
    let pos = 16;
    let freqs = compute_rope_frequencies(dim, pos, 10000.0, 16);

    assert_eq!(freqs.len(), dim / 2);
    for (s, c) in freqs {
        assert!(!s.is_nan());
        assert!(!c.is_nan());
        let norm = s * s + c * c;
        assert!(
            (norm - 1.0).abs() < 1e-3,
            "Unit circle norm violated in RoPE: got {}",
            norm
        );
    }
}

#[test]
fn test_cordic_microcode_synthesis_and_jit() {
    let config = CordicConfig {
        mode: CordicMode::CircularRotation,
        iterations: 16,
        energy_per_iter_pj: 0.12,
    };

    let cl_code = synthesize_cordic_cl(&config);
    assert!(cl_code.contains("CRON 256-Core 4D-Torus CORDIC Geometric Silicon Microcode"));
    assert!(cl_code.contains("B0000: '==01#0C0>"));
    assert!(cl_code.contains("B0017: _bb00#000>"));

    // Verify .cl slot format and CRC-8 integrity
    let report = verify_cl_program(&cl_code).expect("Synthesized CORDIC .cl must pass verification");
    assert_eq!(report.total_bundles, 18); // 1 init + 16 iters + 1 writeback
    assert!(report.crc_verified > 0);

    // Execute on Silicon JIT Core
    let jit_res = run_cl_jit(&cl_code);
    assert!(jit_res.is_ok(), "Synthesized CORDIC .cl must execute cleanly on JIT: {:?}", jit_res.err());
}

#[test]
fn test_cordic_bloch_sphere_and_telemetry() {
    let theta = FRAC_PI_3;
    let phi = FRAC_PI_4;

    let sphere_ascii = render_ascii_bloch_sphere(theta, phi);
    assert!(sphere_ascii.contains("CRON BRAIN 5 QUANTUM & PHOTONIC BLOCH / POINCARÉ SPHERE"));
    assert!(sphere_ascii.contains("Spherical Angles"));
    assert!(sphere_ascii.contains("Cartesian Vector"));

    let orbit_ascii = render_ascii_phase_orbit(&[(1.0, 0.0), (0.7071, 0.7071)]);
    assert!(orbit_ascii.contains("Phase-Space Coordinate Trajectory"));

    let config = CordicConfig {
        mode: CordicMode::CircularRotation,
        iterations: 16,
        energy_per_iter_pj: 0.12,
    };
    let res = run_cordic(1.0, 0.0, FRAC_PI_4, &config);
    let json = res.to_json();
    assert!(json.contains("\"mode\": \"Circular Rotation (sin, cos)\""));
    assert!(json.contains("\"dynamic_energy_pj\""));
}
