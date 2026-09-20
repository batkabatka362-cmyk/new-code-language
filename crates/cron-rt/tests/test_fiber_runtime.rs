// ============================================================================
// CRON Fiber Runtime Verification Suite (test_fiber_runtime.rs)
// Tests high-density M:N work-stealing fibers and inter-fiber channels.
// ============================================================================

use cron_rt::fiber::{FiberChannel, FiberConfig, FiberScheduler};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::time::Instant;

#[test]
fn test_fiber_runtime_high_density_spawn() {
    let config = FiberConfig {
        num_workers: 4,
        stack_size: 16 * 1024,
    };
    let scheduler = FiberScheduler::new(config);

    let counter = Arc::new(AtomicU64::new(0));
    let num_tasks: usize = 100_000;

    let start = Instant::now();
    let mut tasks = Vec::with_capacity(num_tasks);

    for _ in 0..num_tasks {
        let c = Arc::clone(&counter);
        tasks.push(move || {
            c.fetch_add(1, Ordering::Relaxed);
        });
    }

    scheduler.spawn_batch(tasks);
    scheduler.wait_idle(num_tasks as u64);
    let elapsed = start.elapsed();

    assert_eq!(counter.load(Ordering::Acquire), num_tasks as u64);
    let ops_per_sec = (num_tasks as f64) / elapsed.as_secs_f64();
    println!(
        "CRON Fiber Runtime: Executed {} fibers in {:.3?} ({:.0} fibers/sec)",
        num_tasks, elapsed, ops_per_sec
    );
    assert!(
        ops_per_sec > 100_000.0,
        "Fiber scheduler throughput must exceed 100,000 tasks/sec"
    );

    scheduler.shutdown();
}

#[test]
fn test_fiber_channel_ring_buffer() {
    let chan = FiberChannel::<u64>::new(1024);
    assert!(chan.is_empty());

    for i in 0..512 {
        assert!(chan.send(i).is_ok());
    }
    assert_eq!(chan.len(), 512);

    for i in 0..512 {
        assert_eq!(chan.try_recv(), Some(i));
    }
    assert!(chan.is_empty());
}

#[test]
fn test_fiber_individual_spawn_and_handle() {
    let scheduler = FiberScheduler::new(FiberConfig::default());
    let flag = Arc::new(AtomicU64::new(0));

    let flag_clone = Arc::clone(&flag);
    let handle = scheduler.spawn(move || {
        flag_clone.store(42, Ordering::Release);
    });

    handle.wait();
    assert_eq!(flag.load(Ordering::Acquire), 42);
    assert!(handle.is_completed());

    scheduler.shutdown();
}
