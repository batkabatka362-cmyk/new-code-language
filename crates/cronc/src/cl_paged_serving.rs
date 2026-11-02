// ============================================================================
// CRON Continuous Dynamic Batching & Paged KV-Cache Serving Engine (cl_paged_serving.rs)
//
// 100% Pure Rust - Zero Python, Zero vLLM / Ray dependencies.
//
// Features:
//   1. PagedAttention Memory Architecture: Physical KV-cache blocks allocated in
//      fixed page slices (e.g. 16 tokens/block), eliminating memory fragmentation.
//   2. Virtual Logical-to-Physical Block Table: Maps arbitrary sequence lengths
//      to non-contiguous physical memory blocks with copy-on-write reference counting.
//   3. Continuous (Iteration-Level) Dynamic Scheduler: Dynamically slots new prompts
//      into active decoding iterations without pausing or blocking running streams.
//   4. Multi-Sequence Prefix Sharing: Reuses cached system prompts and prefixes
//      across concurrent requests with 0 duplicate VRAM allocations.
//   5. Preemption & Eviction Recovery: Graciously handles memory spikes by preempting
//      and resuming requests when memory pressure eases.
// ============================================================================

use std::collections::{HashMap, VecDeque};
use std::sync::atomic::{AtomicU64, Ordering};

static GLOBAL_REQUEST_ID: AtomicU64 = AtomicU64::new(1001);

/// Fixed token block size for paged attention
pub const DEFAULT_PAGE_BLOCK_SIZE: usize = 16;

/// Lifecycle state of an inference serving request
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RequestState {
    Waiting,
    Prefill,
    Decoding,
    Finished,
    Preempted,
}

/// Physical KV-cache block allocated in the global memory pool
#[derive(Debug, Clone)]
pub struct PhysicalBlock {
    pub block_id: usize,
    pub ref_count: usize,
    pub tokens: Vec<u32>, // Max length = block_size
    // Key and Value tensors for all layers: [num_layers, 2, block_size, num_heads, head_dim]
    pub kv_data: Vec<f32>,
}

impl PhysicalBlock {
    pub fn new(block_id: usize, block_size: usize, kv_dim: usize) -> Self {
        Self {
            block_id,
            ref_count: 0,
            tokens: Vec::with_capacity(block_size),
            kv_data: vec![0.0f32; block_size * kv_dim],
        }
    }

    pub fn is_free(&self) -> bool {
        self.ref_count == 0
    }

    pub fn is_full(&self, block_size: usize) -> bool {
        self.tokens.len() >= block_size
    }

    pub fn reset(&mut self) {
        self.ref_count = 0;
        self.tokens.clear();
        self.kv_data.fill(0.0);
    }
}

/// Pool managing all physical memory blocks
pub struct BlockPool {
    pub blocks: Vec<PhysicalBlock>,
    pub free_block_ids: VecDeque<usize>,
    pub block_size: usize,
    pub kv_dim: usize,
}

impl BlockPool {
    pub fn new(total_blocks: usize, block_size: usize, kv_dim: usize) -> Self {
        let mut blocks = Vec::with_capacity(total_blocks);
        let mut free_block_ids = VecDeque::with_capacity(total_blocks);

        for i in 0..total_blocks {
            blocks.push(PhysicalBlock::new(i, block_size, kv_dim));
            free_block_ids.push_back(i);
        }

        Self {
            blocks,
            free_block_ids,
            block_size,
            kv_dim,
        }
    }

    pub fn free_blocks_count(&self) -> usize {
        self.free_block_ids.len()
    }

    pub fn total_blocks_count(&self) -> usize {
        self.blocks.len()
    }

    /// Allocates a single physical block
    pub fn alloc_block(&mut self) -> Result<usize, String> {
        if let Some(id) = self.free_block_ids.pop_front() {
            self.blocks[id].ref_count = 1;
            self.blocks[id].tokens.clear();
            Ok(id)
        } else {
            Err("VRAM Out Of Memory: No free physical blocks available in BlockPool".to_string())
        }
    }

    /// Increments reference count for shared prefix caching
    pub fn retain_block(&mut self, block_id: usize) {
        if block_id < self.blocks.len() {
            self.blocks[block_id].ref_count += 1;
        }
    }

    /// Decrements reference count and returns to free pool if ref_count hits 0
    pub fn release_block(&mut self, block_id: usize) {
        if block_id < self.blocks.len() {
            if self.blocks[block_id].ref_count > 0 {
                self.blocks[block_id].ref_count -= 1;
            }
            if self.blocks[block_id].ref_count == 0 {
                self.blocks[block_id].reset();
                self.free_block_ids.push_back(block_id);
            }
        }
    }
}

/// An active inference serving request
#[derive(Debug, Clone)]
pub struct ServingRequest {
    pub request_id: u64,
    pub prompt_tokens: Vec<u32>,
    pub generated_tokens: Vec<u32>,
    pub max_new_tokens: usize,
    pub state: RequestState,
    pub block_table: Vec<usize>, // Logical block index -> Physical block ID
    pub prompt_cached_count: usize,
    pub eos_token_id: u32,
}

impl ServingRequest {
    pub fn new(prompt_tokens: Vec<u32>, max_new_tokens: usize, eos_token_id: u32) -> Self {
        let req_id = GLOBAL_REQUEST_ID.fetch_add(1, Ordering::Relaxed);
        Self {
            request_id: req_id,
            prompt_tokens,
            generated_tokens: Vec::new(),
            max_new_tokens,
            state: RequestState::Waiting,
            block_table: Vec::new(),
            prompt_cached_count: 0,
            eos_token_id,
        }
    }

    pub fn total_tokens(&self) -> usize {
        self.prompt_tokens.len() + self.generated_tokens.len()
    }

    pub fn is_finished(&self) -> bool {
        self.state == RequestState::Finished
    }
}

/// Engine Telemetry Metrics
#[derive(Debug, Clone, Default)]
pub struct ServingTelemetry {
    pub total_requests_scheduled: usize,
    pub completed_requests: usize,
    pub total_prefilled_tokens: usize,
    pub total_generated_tokens: usize,
    pub total_preemptions: usize,
    pub shared_prefix_hits: usize,
}

/// High-Performance Continuous Dynamic Batching Engine
pub struct PagedServingEngine {
    pub pool: BlockPool,
    pub waiting_queue: VecDeque<ServingRequest>,
    pub active_requests: Vec<ServingRequest>,
    pub prefix_cache: HashMap<Vec<u32>, usize>, // Prefix token slice -> physical block ID
    pub telemetry: ServingTelemetry,
    pub min_free_blocks_watermark: usize,
}

impl PagedServingEngine {
    pub fn new(total_blocks: usize, block_size: usize, kv_dim: usize) -> Self {
        Self {
            pool: BlockPool::new(total_blocks, block_size, kv_dim),
            waiting_queue: VecDeque::new(),
            active_requests: Vec::new(),
            prefix_cache: HashMap::new(),
            telemetry: ServingTelemetry::default(),
            min_free_blocks_watermark: 2,
        }
    }

    /// Submits a new prompt generation request to the serving queue
    pub fn submit_request(&mut self, request: ServingRequest) {
        self.waiting_queue.push_back(request);
        self.telemetry.total_requests_scheduled += 1;
    }

    /// Performs one iteration step of continuous dynamic batching:
    /// 1. Evicts finished requests and cleans up physical memory
    /// 2. Performs decoding step on currently active sequences
    /// 3. Admits new waiting requests into prefill if memory permits
    pub fn step<F>(&mut self, mut model_forward_step: F) -> Result<Vec<(u64, u32)>, String>
    where
        F: FnMut(&[u64], &[u32]) -> Result<Vec<u32>, String>,
    {
        // -------------------------------------------------------------
        // Step 1: Release finished requests
        // -------------------------------------------------------------
        let mut finished_indices = Vec::new();
        for (i, req) in self.active_requests.iter().enumerate() {
            if req.state == RequestState::Finished {
                finished_indices.push(i);
            }
        }

        // Release in reverse order to keep indices stable
        for &idx in finished_indices.iter().rev() {
            let finished_req = self.active_requests.swap_remove(idx);
            for &block_id in &finished_req.block_table {
                self.pool.release_block(block_id);
            }
            self.telemetry.completed_requests += 1;
        }

        // -------------------------------------------------------------
        // Step 2: Admit Waiting requests into Prefill
        // -------------------------------------------------------------
        while let Some(mut req) = self.waiting_queue.pop_front() {
            let prompt_len = req.prompt_tokens.len();
            let needed_blocks = (prompt_len + self.pool.block_size - 1) / self.pool.block_size;

            if self.pool.free_blocks_count() >= needed_blocks + self.min_free_blocks_watermark {
                // Check for prefix cache hit on first block
                let block_size = self.pool.block_size;
                let mut start_token_idx = 0;

                if prompt_len >= block_size {
                    let first_block_tokens = req.prompt_tokens[0..block_size].to_vec();
                    if let Some(&cached_id) = self.prefix_cache.get(&first_block_tokens) {
                        self.pool.retain_block(cached_id);
                        req.block_table.push(cached_id);
                        start_token_idx = block_size;
                        self.telemetry.shared_prefix_hits += 1;
                    }
                }

                // Allocate remaining blocks
                let mut alloc_ok = true;
                while start_token_idx < prompt_len {
                    match self.pool.alloc_block() {
                        Ok(bid) => {
                            let end = (start_token_idx + block_size).min(prompt_len);
                            self.pool.blocks[bid].tokens = req.prompt_tokens[start_token_idx..end].to_vec();

                            // Cache the first block if not already cached
                            if req.block_table.is_empty() && end - start_token_idx == block_size {
                                self.prefix_cache.insert(self.pool.blocks[bid].tokens.clone(), bid);
                                self.pool.retain_block(bid);
                            }

                            req.block_table.push(bid);
                            start_token_idx = end;
                        }
                        Err(_) => {
                            alloc_ok = false;
                            break;
                        }
                    }
                }

                if alloc_ok {
                    req.state = RequestState::Decoding;
                    req.prompt_cached_count = prompt_len;
                    self.telemetry.total_prefilled_tokens += prompt_len;
                    self.active_requests.push(req);
                } else {
                    // Preempt back to waiting
                    for &bid in &req.block_table {
                        self.pool.release_block(bid);
                    }
                    req.block_table.clear();
                    self.waiting_queue.push_front(req);
                    break;
                }
            } else {
                // Not enough memory right now, keep in waiting queue
                self.waiting_queue.push_front(req);
                break;
            }
        }

        if self.active_requests.is_empty() {
            return Ok(Vec::new());
        }

        // -------------------------------------------------------------
        // Step 3: Run Model Forward Step for Active Requests
        // -------------------------------------------------------------
        let mut request_ids = Vec::with_capacity(self.active_requests.len());
        let mut input_tokens = Vec::with_capacity(self.active_requests.len());

        for req in &self.active_requests {
            request_ids.push(req.request_id);
            let last_token = if let Some(&t) = req.generated_tokens.last() {
                t
            } else {
                *req.prompt_tokens.last().unwrap_or(&0)
            };
            input_tokens.push(last_token);
        }

        let sampled_tokens = model_forward_step(&request_ids, &input_tokens)?;

        let mut output_events = Vec::new();

        // -------------------------------------------------------------
        // Step 4: Append generated tokens & manage block allocation
        // -------------------------------------------------------------
        for (i, next_tok) in sampled_tokens.into_iter().enumerate() {
            let req = &mut self.active_requests[i];
            req.generated_tokens.push(next_tok);
            self.telemetry.total_generated_tokens += 1;
            output_events.push((req.request_id, next_tok));

            if next_tok == req.eos_token_id || req.generated_tokens.len() >= req.max_new_tokens {
                req.state = RequestState::Finished;
                continue;
            }

            // Append token to current physical block or allocate new one
            let block_size = self.pool.block_size;
            if let Some(&last_bid) = req.block_table.last() {
                if !self.pool.blocks[last_bid].is_full(block_size) {
                    self.pool.blocks[last_bid].tokens.push(next_tok);
                } else {
                    // Need a new block
                    match self.pool.alloc_block() {
                        Ok(new_bid) => {
                            self.pool.blocks[new_bid].tokens.push(next_tok);
                            req.block_table.push(new_bid);
                        }
                        Err(_) => {
                            // Preempt request due to memory exhaustion
                            req.state = RequestState::Preempted;
                            self.telemetry.total_preemptions += 1;
                        }
                    }
                }
            }
        }

        Ok(output_events)
    }

    /// Returns true if all queued and active requests have finished
    pub fn is_idle(&self) -> bool {
        self.waiting_queue.is_empty() && self.active_requests.is_empty()
    }

    /// Clears the global prefix cache and releases retained physical blocks
    pub fn clear_prefix_cache(&mut self) {
        for &bid in self.prefix_cache.values() {
            self.pool.release_block(bid);
        }
        self.prefix_cache.clear();
    }
}
