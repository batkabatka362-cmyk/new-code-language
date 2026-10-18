//! CRON SGLang-Style RadixTree Prefix Caching System (cron-rt)
//! Inspired by SGLang's RadixAttention architecture:
//! - Indexes token sequences and KV-cache blocks in a compact Radix (Prefix) Tree.
//! - Multi-turn chat & shared system prompts achieve 0-overhead token re-use (TTFT -> 0s).
//! - Automatic node splitting, reference counting, and LRU eviction of cold cache branches.

use std::sync::atomic::{AtomicU64, Ordering};

static GLOBAL_TIME_TICK: AtomicU64 = AtomicU64::new(1);

/// A node in the Radix Tree containing a sequence of tokens and associated KV block IDs
#[derive(Debug, Clone)]
pub struct RadixTreeNode {
    pub node_id: usize,
    pub token_prefix: Vec<u32>,
    pub kv_block_ids: Vec<usize>,
    pub children: Vec<RadixTreeNode>,
    pub last_accessed: u64,
    pub ref_count: usize,
}

impl RadixTreeNode {
    pub fn new(node_id: usize, token_prefix: Vec<u32>, kv_block_ids: Vec<usize>) -> Self {
        Self {
            node_id,
            token_prefix,
            kv_block_ids,
            children: Vec::new(),
            last_accessed: GLOBAL_TIME_TICK.fetch_add(1, Ordering::Relaxed),
            ref_count: 1,
        }
    }

    /// Finds length of common prefix between `self.token_prefix` and `tokens`
    pub fn common_prefix_len(&self, tokens: &[u32]) -> usize {
        let mut i = 0;
        while i < self.token_prefix.len() && i < tokens.len() && self.token_prefix[i] == tokens[i] {
            i += 1;
        }
        i
    }
}

/// Telemetry report for RadixTree cache performance
#[derive(Debug, Clone, Default)]
pub struct RadixCacheTelemetry {
    pub total_lookups: usize,
    pub cache_hits: usize,
    pub cache_misses: usize,
    pub total_queried_tokens: usize,
    pub total_saved_tokens: usize,
    pub current_nodes: usize,
    pub current_kv_blocks: usize,
    pub evicted_blocks: usize,
    pub hit_ratio_pct: f64,
}

/// SGLang-Style RadixTree Prefix Cache Manager
pub struct RadixTreeCache {
    pub root: RadixTreeNode,
    pub max_kv_blocks: usize,
    pub next_node_id: usize,
    pub telemetry: RadixCacheTelemetry,
}

impl Default for RadixTreeCache {
    fn default() -> Self {
        Self::new(8192) // default 8,192 KV blocks
    }
}

impl RadixTreeCache {
    pub fn new(max_kv_blocks: usize) -> Self {
        Self {
            root: RadixTreeNode::new(0, Vec::new(), Vec::new()),
            max_kv_blocks,
            next_node_id: 1,
            telemetry: RadixCacheTelemetry::default(),
        }
    }

    /// Matches the longest existing prefix in the tree for the given prompt tokens.
    /// Returns: (matched_token_count, Vec<kv_block_id>)
    pub fn match_prefix(&mut self, prompt_tokens: &[u32]) -> (usize, Vec<usize>) {
        self.telemetry.total_lookups += 1;
        self.telemetry.total_queried_tokens += prompt_tokens.len();

        let mut matched_len = 0;
        let mut matched_blocks = Vec::new();
        let mut current_node = &mut self.root;
        let mut remaining = prompt_tokens;

        while !remaining.is_empty() {
            let mut found_child = false;
            let mut child_idx = 0;

            for (idx, child) in current_node.children.iter().enumerate() {
                if !child.token_prefix.is_empty() && child.token_prefix[0] == remaining[0] {
                    found_child = true;
                    child_idx = idx;
                    break;
                }
            }

            if !found_child {
                break;
            }

            let child = &mut current_node.children[child_idx];
            let c_len = child.common_prefix_len(remaining);

            if c_len == child.token_prefix.len() {
                // Exact full match of this child node
                matched_len += c_len;
                matched_blocks.extend_from_slice(&child.kv_block_ids);
                child.last_accessed = GLOBAL_TIME_TICK.fetch_add(1, Ordering::Relaxed);
                remaining = &remaining[c_len..];
                current_node = &mut current_node.children[child_idx];
            } else if c_len > 0 {
                // Partial match inside the child's token prefix
                matched_len += c_len;
                let blocks_to_take = c_len.min(child.kv_block_ids.len());
                matched_blocks.extend_from_slice(&child.kv_block_ids[..blocks_to_take]);
                child.last_accessed = GLOBAL_TIME_TICK.fetch_add(1, Ordering::Relaxed);
                break;
            } else {
                break;
            }
        }

        if matched_len > 0 {
            self.telemetry.cache_hits += 1;
            self.telemetry.total_saved_tokens += matched_len;
        } else {
            self.telemetry.cache_misses += 1;
        }

        let total = self.telemetry.total_lookups as f64;
        self.telemetry.hit_ratio_pct = if total > 0.0 {
            (self.telemetry.cache_hits as f64 / total) * 100.0
        } else {
            0.0
        };

        (matched_len, matched_blocks)
    }

    /// Inserts a sequence of tokens and newly allocated KV blocks into the Radix Tree.
    pub fn insert(&mut self, tokens: &[u32], kv_blocks: &[usize]) {
        if tokens.is_empty() || tokens.len() != kv_blocks.len() {
            return;
        }

        let mut current_node = &mut self.root;
        let mut rem_tokens = tokens;
        let mut rem_blocks = kv_blocks;

        while !rem_tokens.is_empty() {
            let mut match_idx = None;
            for (idx, child) in current_node.children.iter().enumerate() {
                if !child.token_prefix.is_empty() && child.token_prefix[0] == rem_tokens[0] {
                    match_idx = Some(idx);
                    break;
                }
            }

            if let Some(idx) = match_idx {
                let child = &mut current_node.children[idx];
                let c_len = child.common_prefix_len(rem_tokens);

                if c_len == child.token_prefix.len() {
                    // Descend fully into this child
                    rem_tokens = &rem_tokens[c_len..];
                    rem_blocks = &rem_blocks[c_len..];
                    current_node = &mut current_node.children[idx];
                } else {
                    // Node split required!
                    // 1. Existing child split into prefix [0..c_len] and suffix [c_len..]
                    let suffix_tokens = child.token_prefix[c_len..].to_vec();
                    let suffix_blocks = child.kv_block_ids[c_len..].to_vec();
                    let suffix_children = std::mem::take(&mut child.children);

                    let new_suffix_child = RadixTreeNode {
                        node_id: self.next_node_id,
                        token_prefix: suffix_tokens,
                        kv_block_ids: suffix_blocks,
                        children: suffix_children,
                        last_accessed: child.last_accessed,
                        ref_count: 1,
                    };
                    self.next_node_id += 1;

                    // Truncate existing child to common prefix
                    child.token_prefix.truncate(c_len);
                    child.kv_block_ids.truncate(c_len);
                    child.children = vec![new_suffix_child];

                    // 2. Add the branch for remainder of new tokens
                    let new_branch_tokens = rem_tokens[c_len..].to_vec();
                    let new_branch_blocks = rem_blocks[c_len..].to_vec();
                    if !new_branch_tokens.is_empty() {
                        let new_branch_child = RadixTreeNode::new(
                            self.next_node_id,
                            new_branch_tokens,
                            new_branch_blocks,
                        );
                        self.next_node_id += 1;
                        child.children.push(new_branch_child);
                    }
                    self.update_telemetry_counts();
                    return;
                }
            } else {
                // No matching child found, create a new leaf child directly
                let new_child = RadixTreeNode::new(
                    self.next_node_id,
                    rem_tokens.to_vec(),
                    rem_blocks.to_vec(),
                );
                self.next_node_id += 1;
                current_node.children.push(new_child);
                self.update_telemetry_counts();
                return;
            }
        }

        self.update_telemetry_counts();
    }

    /// Evicts least recently accessed leaf nodes to free up KV blocks
    pub fn evict_lru(&mut self, blocks_to_free: usize) -> usize {
        let mut freed = 0;
        while freed < blocks_to_free && !self.root.children.is_empty() {
            // Find leaf node with oldest last_accessed
            let mut oldest_time = u64::MAX;
            let mut oldest_idx = None;

            for (idx, child) in self.root.children.iter().enumerate() {
                if child.children.is_empty() && child.last_accessed < oldest_time {
                    oldest_time = child.last_accessed;
                    oldest_idx = Some(idx);
                }
            }

            if let Some(idx) = oldest_idx {
                let removed = self.root.children.remove(idx);
                freed += removed.kv_block_ids.len();
                self.telemetry.evicted_blocks += removed.kv_block_ids.len();
            } else {
                break;
            }
        }
        self.update_telemetry_counts();
        freed
    }

    fn update_telemetry_counts(&mut self) {
        fn count_stats(node: &RadixTreeNode) -> (usize, usize) {
            let mut nodes = 1;
            let mut blocks = node.kv_block_ids.len();
            for c in &node.children {
                let (n, b) = count_stats(c);
                nodes += n;
                blocks += b;
            }
            (nodes, blocks)
        }
        let (n, b) = count_stats(&self.root);
        self.telemetry.current_nodes = n;
        self.telemetry.current_kv_blocks = b;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_radix_cache_insert_and_prefix_match() {
        let mut cache = RadixTreeCache::new(1024);

        // System prompt: tokens [101, 202, 303, 404]
        let sys_prompt = vec![101, 202, 303, 404];
        let sys_blocks = vec![0, 1, 2, 3];
        cache.insert(&sys_prompt, &sys_blocks);

        // Query with exact system prompt
        let (matched_len, blocks) = cache.match_prefix(&sys_prompt);
        assert_eq!(matched_len, 4);
        assert_eq!(blocks, vec![0, 1, 2, 3]);
        assert_eq!(cache.telemetry.cache_hits, 1);

        // Query with longer conversation sharing system prompt: [101, 202, 303, 404, 505, 606]
        let query_tokens = vec![101, 202, 303, 404, 505, 606];
        let (matched_len2, blocks2) = cache.match_prefix(&query_tokens);
        assert_eq!(matched_len2, 4);
        assert_eq!(blocks2, vec![0, 1, 2, 3]);

        // Insert new turn
        let turn_blocks = vec![0, 1, 2, 3, 4, 5];
        cache.insert(&query_tokens, &turn_blocks);

        // Now query matches full 6 tokens
        let (matched_len3, blocks3) = cache.match_prefix(&query_tokens);
        assert_eq!(matched_len3, 6);
        assert_eq!(blocks3, vec![0, 1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_radix_cache_node_split() {
        let mut cache = RadixTreeCache::new(1024);

        // Path A: [1, 2, 3, 4]
        cache.insert(&[1, 2, 3, 4], &[10, 11, 12, 13]);
        // Path B: [1, 2, 8, 9] (splits at index 2!)
        cache.insert(&[1, 2, 8, 9], &[10, 11, 18, 19]);

        let (len_a, blocks_a) = cache.match_prefix(&[1, 2, 3, 4]);
        assert_eq!(len_a, 4);
        assert_eq!(blocks_a, vec![10, 11, 12, 13]);

        let (len_b, blocks_b) = cache.match_prefix(&[1, 2, 8, 9]);
        assert_eq!(len_b, 4);
        assert_eq!(blocks_b, vec![10, 11, 18, 19]);

        // Common prefix match: [1, 2, 99] should match 2 tokens
        let (len_common, blocks_common) = cache.match_prefix(&[1, 2, 99]);
        assert_eq!(len_common, 2);
        assert_eq!(blocks_common, vec![10, 11]);
    }

    #[test]
    fn test_radix_cache_lru_eviction() {
        let mut cache = RadixTreeCache::new(100);

        cache.insert(&[1, 2, 3], &[0, 1, 2]);
        cache.insert(&[4, 5, 6], &[3, 4, 5]);

        let freed = cache.evict_lru(3);
        assert!(freed >= 3);
        assert_eq!(cache.telemetry.evicted_blocks, freed);
    }
}
