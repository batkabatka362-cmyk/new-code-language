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

#[test]
fn test_lock_free_mpmc_concurrent_stress() {
    let chan = Arc::new(FiberChannel::<u64>::new(2048));
    let num_producers = 8;
    let num_consumers = 8;
    let items_per_producer = 5_000u64;
    let total_items = num_producers * items_per_producer;

    let received_sum = Arc::new(AtomicU64::new(0));
    let received_count = Arc::new(AtomicU64::new(0));

    // Spawn consumers
    let mut consumer_handles = Vec::new();
    for _ in 0..num_consumers {
        let ch = Arc::clone(&chan);
        let sum_ref = Arc::clone(&received_sum);
        let cnt_ref = Arc::clone(&received_count);
        let handle = std::thread::spawn(move || {
            while cnt_ref.load(Ordering::Acquire) < total_items {
                if let Some(val) = ch.try_recv() {
                    sum_ref.fetch_add(val, Ordering::Relaxed);
                    cnt_ref.fetch_add(1, Ordering::Release);
                } else {
                    std::hint::spin_loop();
                }
            }
        });
        consumer_handles.push(handle);
    }

    // Spawn producers
    let mut producer_handles = Vec::new();
    for p in 0..num_producers {
        let ch = Arc::clone(&chan);
        let handle = std::thread::spawn(move || {
            for i in 0..items_per_producer {
                let val = (p as u64) * items_per_producer + i + 1;
                while ch.send(val).is_err() {
                    std::hint::spin_loop();
                }
            }
        });
        producer_handles.push(handle);
    }

    for h in producer_handles {
        h.join().unwrap();
    }
    for h in consumer_handles {
        h.join().unwrap();
    }

    let expected_sum: u64 = (1..=total_items).sum();
    assert_eq!(received_count.load(Ordering::Acquire), total_items);
    assert_eq!(received_sum.load(Ordering::Acquire), expected_sum);
    assert!(chan.is_empty());
}
