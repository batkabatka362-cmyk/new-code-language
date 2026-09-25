# Module `knowledge_graph`

> Target: 256-Core 4D-Torus Neuromorphic Photonic Processor

## 📦 Structures & Linear Types

### `struct Triple`

| Field | Type |
|---|---|
| `subject_id` | `u32` |
| `predicate_id` | `u16` |
| `object_id` | `u32` |
| `confidence_fixed` | `u16` |

### `struct KnowledgeGraphPartition`

| Field | Type |
|---|---|
| `triples` | `[Triple; 64]` |
| `triple_count` | `u32` |
| `partition_core_id` | `u32` |

## ⚡ Functions & Intrinsics

### `fn init_kg_partition(core_id: u32) -> KnowledgeGraphPartition`

### `fn assert_triple(kg: linear KnowledgeGraphPartition, subj: u32, pred: u16, obj: u32, confidence: u16) -> linear KnowledgeGraphPartition`

### `fn infer_causal_reachability(kg: KnowledgeGraphPartition, start_node: u32, target_node: u32) -> bool`

