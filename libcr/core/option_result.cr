// ============================================================================
// CRON Standard Library - Core Algebraic Data Types (ADTs)
// Option<T> and Result<T, E> definitions with functional helper methods
// Target: 256-Core 4D-Torus Neuromorphic Photonic Processor
// ============================================================================

.MODULE cron.core.option_result

/// Option<T>: Representation of optional values without null pointers
enum Option<T> {
    Some(T),
    None
}

/// Result<T, E>: Error handling container for fallible operations
enum Result<T, E> {
    Ok(T),
    Err(E)
}

/// Returns true if the option is a Some value
def is_some(opt: Option<i32>) -> bool {
    match opt {
        Option::Some(_) => true,
        Option::None => false,
    }
}

/// Returns true if the option is a None value
def is_none(opt: Option<i32>) -> bool {
    match opt {
        Option::Some(_) => false,
        Option::None => true,
    }
}

/// Unwraps an option, yielding the content of a Some or a default value
def unwrap_or(opt: Option<i32>, default_val: i32) -> i32 {
    match opt {
        Option::Some(v) => v,
        Option::None => default_val,
    }
}

/// Returns true if the result is Ok
def is_ok(res: Result<i32, i32>) -> bool {
    match res {
        Result::Ok(_) => true,
        Result::Err(_) => false,
    }
}

/// Returns true if the result is Err
def is_err(res: Result<i32, i32>) -> bool {
    match res {
        Result::Ok(_) => false,
        Result::Err(_) => true,
    }
}

/// Unwraps a result, yielding the content of an Ok or a default value
def unwrap_result_or(res: Result<i32, i32>, default_val: i32) -> i32 {
    match res {
        Result::Ok(v) => v,
        Result::Err(_) => default_val,
    }
}
