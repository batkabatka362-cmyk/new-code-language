// CRON Standard Library - Linear Ownership & Conservation Guarantees
// Module: cron.core.linear
// Target: 256-Core 4D-Torus Hardware Architecture

.MODULE cron.core.linear

// Linear Token Guard: Proves at compile-time that hardware resource is consumed exactly once
struct LinearGuard<T> {
    resource: T,
    consumed: bool
}

def wrap_linear<T>(val: T) -> linear LinearGuard<T> {
    return LinearGuard<T> {
        resource: val,
        consumed: false
    }
}

// Safely consume linear resource with zero runtime allocation
def unwrap_linear<T>(lin guard: linear LinearGuard<T>) -> T {
    let res = guard.resource
    consume(guard)
    return res
}

// Linear swap: Exchanges contents of two linear containers without copying
def linear_swap<T>(lin a: linear LinearGuard<T>, lin b: linear LinearGuard<T>) -> (linear LinearGuard<T>, linear LinearGuard<T>) {
    let val_a = a.resource
    let val_b = b.resource
    consume(a)
    consume(b)
    return (wrap_linear(val_b), wrap_linear(val_a))
}
