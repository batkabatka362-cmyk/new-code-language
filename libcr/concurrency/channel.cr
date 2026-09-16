// CRON Standard Library: Concurrency & CSP Message-Passing Channels
// Target: 256-Core Neuromorphic 4D-Torus Processor & Multi-Threaded Fiber Runtime

.MODULE cron.concurrency.channel

/// Lock-free bounded FIFO channel for inter-fiber and inter-core communication
struct Channel<T> {
    id: i64,
    capacity: i64,
}

impl<T> Channel<T> {
    fn new(capacity: i64) -> Channel<T> {
        let ch_id = channel_new(capacity);
        return Channel<T> {
            id: ch_id,
            capacity: capacity,
        };
    }

    fn send(self, val: T) -> i64 {
        return channel_send(self.id, val as i64);
    }

    fn recv(self) -> i64 {
        return channel_recv(self.id);
    }

    fn try_recv(self) -> i64 {
        return channel_try_recv(self.id);
    }

    fn close(self) -> i64 {
        return channel_close(self.id);
    }
}

/// Helper constructor to create a bounded channel with given capacity
fn channel_create<T>(cap: i64) -> Channel<T> {
    let ch_id = channel_new(cap);
    return Channel<T> {
        id: ch_id,
        capacity: cap,
    };
}


/// Helper to close a channel by ID
fn channel_shutdown(id: i64) -> i64 {
    return channel_close(id);
}
