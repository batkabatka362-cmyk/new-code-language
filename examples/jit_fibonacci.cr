.MODULE JitBenchmark

_main:
    let n: i32 = 20
    let mut a: i32 = 0
    let mut b: i32 = 1
    let mut i: i32 = 0
    while i < n {
        let temp: i32 = a + b
        a = b
        b = temp
        i = i + 1
    }
    return a
.END
