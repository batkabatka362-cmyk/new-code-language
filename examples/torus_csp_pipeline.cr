// ============================================================================
// CRON Example: 3-Stage Asynchronous 4D-Torus CSP Pipeline
// Architecture: 256-Core Neuromorphic Processor ($4 \times 4 \times 4 \times 4$)
// Features: spawn at(core_id), channel<i64>, non-blocking CSP message passing,
//           Dimension-Order Routing, and Toroidal Manhattan Hop calculations.
// ============================================================================

.MODULE cron.example.torus_pipeline

fn producer(out_ch: i64, count: i64) -> i64 {
    let mut i = 1;
    while i <= count {
        let sensor_sample = i * 10;
        channel_send(out_ch, sensor_sample);
        i = i + 1;
    }
    channel_close(out_ch);
    return count;
}

fn transformer(in_ch: i64, out_ch: i64, scale_factor: i64) -> i64 {
    let mut processed = 0;
    let mut active = 1;
    while active == 1 {
        let val = channel_recv(in_ch);
        if val == 0 {
            active = 0;
        } else {
            let transformed = val * scale_factor + 7;
            channel_send(out_ch, transformed);
            processed = processed + 1;
        }
    }
    channel_close(out_ch);
    return processed;
}

fn consumer(in_ch: i64) -> i64 {
    let mut total_sum = 0;
    let mut active = 1;
    while active == 1 {
        let item = channel_recv(in_ch);
        if item == 0 {
            active = 0;
        } else {
            total_sum = total_sum + item;
        }
    }
    return total_sum;
}

fn main() -> i64 {
    // 1. Calculate Torus distance between Core 0 and Core 42
    let core_prod = 0;
    let core_trans = 42;
    let core_cons = 127;
    let dist_pt = torus_distance(core_prod, core_trans);
    let dist_tc = torus_distance(core_trans, core_cons);

    // 2. Allocate bounded CSP channels with capacity 16
    let ch1 = channel_new(16);
    let ch2 = channel_new(16);

    // 3. Spawn distributed workers across 4D-Torus cores
    let f1 = spawn at(core_prod) producer(ch1, 5);
    let f2 = spawn at(core_trans) transformer(ch1, ch2, 3);
    let f3 = spawn at(core_cons) consumer(ch2);

    // 4. Await pipeline completion and collect aggregate result
    let p_count = await f1;
    let t_count = await f2;
    let total = await f3;

    // Output: Total sum + hop distance verification
    // 5 samples: 10, 20, 30, 40, 50
    // Transformed (* 3 + 7): 37, 67, 97, 127, 157 -> Sum = 485
    let result = total + dist_pt + dist_tc;
    return result;
}
