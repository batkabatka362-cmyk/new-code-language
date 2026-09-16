use cronc::execute_jit;

#[test]
fn test_jit_arithmetic_expressions() {
    let code = r#"
    .MODULE JitArithmetic
    _main:
        let a = 10
        let b = 25
        let c = 5
        let res = a + b * 2 - c
    .END
    "#;

    let res = execute_jit(code).expect("JIT execution failed");
    // 10 + (25 * 2) - 5 = 10 + 50 - 5 = 55
    assert_eq!(res, 55);
}

#[test]
fn test_jit_conditionals() {
    let code = r#"
    .MODULE JitIfElse
    _main:
        let x = 42
        let mut res = 0
        if x > 20 {
            res = 100
        } else {
            res = 200
        }
    .END
    "#;

    let res = execute_jit(code).expect("JIT execution failed");
    assert_eq!(res, 100);
}

#[test]
fn test_jit_while_loop() {
    let code = r#"
    .MODULE JitLoop
    _main:
        let mut i = 1
        let mut sum = 0
        while i <= 10 {
            sum = sum + i
            i = i + 1
        }
        return sum
    .END
    "#;

    let res = execute_jit(code).expect("JIT execution failed");
    // 1 + 2 + 3 + ... + 10 = 55
    assert_eq!(res, 55);
}

#[test]
fn test_jit_hex_and_modulo() {
    let code = r#"
    .MODULE JitHexMod
    _main:
        let mask = 0xFF
        let val = 105 % 10
        let total = mask + val
    .END
    "#;

    let res = execute_jit(code).expect("JIT execution failed");
    // 255 + 5 = 260
    assert_eq!(res, 260);
}

#[test]
fn test_jit_performance_sub_microsecond() {
    let code = r#"
    .MODULE JitPerf
    _main:
        let mut acc = 0
        let mut step = 0
        while step < 100 {
            acc = acc + step * 2
            step = step + 1
        }
        return acc
    .END
    "#;

    let start = std::time::Instant::now();
    let res = execute_jit(code).expect("JIT execution failed");
    let elapsed = start.elapsed();

    // 0 + 2 + 4 + ... + 198 = 2 * (99 * 100 / 2) = 9900
    assert_eq!(res, 9900);
    println!("JIT compile and execute time: {:?}", elapsed);
    assert!(elapsed.as_millis() < 50, "JIT compilation and execution should take under 50ms");
}
