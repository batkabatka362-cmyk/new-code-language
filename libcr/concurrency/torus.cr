// CRON Standard Library: 4D-Torus Distributed Mesh Actors & NoC Routing
// Target: 256-Core Distributed Neuromorphic Processor Architecture

.MODULE cron.concurrency.torus

import { TorusCoord, core_id_to_coord, coord_to_core_id, manhattan_distance_4d, route_next_hop_dor } from "core/torus.cr"
import { Channel } from "concurrency/channel.cr"

struct TorusMessage {
    src_core: i64,
    dst_core: i64,
    payload: i64,
    hop_count: i64,
}

struct TorusRouter {
    core_id: i64,
    rx_channel: Channel<i64>,
}

impl TorusRouter {
    fn new(core_id: i64) -> TorusRouter {
        let ch_id = channel_new(128);
        return TorusRouter {
            core_id: core_id,
            rx_channel: Channel<i64> {
                id: ch_id,
                capacity: 128,
            },
        };
    }


    fn distance_to(self, target_core: i64) -> i64 {
        let src = core_id_to_coord(self.core_id as u32);
        let dst = core_id_to_coord(target_core as u32);
        return manhattan_distance_4d(src, dst) as i64;
    }

    fn send_to(self, target_core: i64, payload: i64) -> i64 {
        return torus_send(target_core as u32, payload as u64) as i64;
    }

    fn receive(self) -> i64 {
        return self.rx_channel.recv();
    }
}

/// Compute 4D toroidal Manhattan distance between two core IDs directly
fn torus_hop_distance(c1: i64, c2: i64) -> i64 {
    let src = core_id_to_coord(c1 as u32);
    let dst = core_id_to_coord(c2 as u32);
    return manhattan_distance_4d(src, dst) as i64;
}

/// Resolve next immediate core ID on the path from src to dst using Dimension-Order Routing
fn torus_next_hop(src_id: i64, dst_id: i64) -> i64 {
    let src = core_id_to_coord(src_id as u32);
    let dst = core_id_to_coord(dst_id as u32);
    return route_next_hop_dor(src, dst) as i64;
}
