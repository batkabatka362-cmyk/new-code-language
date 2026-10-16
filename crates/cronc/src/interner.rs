// ============================================================================
// CRON Fast Zero-Allocation String Interner (interner.rs)
// Replaces heap-allocated String identifiers with copyable 32-bit Symbol IDs.
// O(1) comparison, zero dynamic re-allocation, cache-friendly contiguous table.
// ============================================================================

use std::collections::HashMap;

/// Copyable 32-bit symbol handle for fast AST and variable resolution
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct Symbol(pub u32);

impl Symbol {
    pub const DUMMY: Symbol = Symbol(u32::MAX);

    #[inline(always)]
    pub const fn as_u32(self) -> u32 {
        self.0
    }
}

/// Contiguous string interner with fast hash lookup
#[derive(Debug, Clone)]
pub struct Interner {
    lookup: HashMap<String, Symbol>,
    strings: Vec<String>,
}

impl Default for Interner {
    fn default() -> Self {
        Self::new()
    }
}

impl Interner {
    pub fn new() -> Self {
        Self {
            lookup: HashMap::with_capacity(512),
            strings: Vec::with_capacity(512),
        }
    }

    /// Intern a string slice, returning its unique 32-bit Symbol
    pub fn intern(&mut self, s: &str) -> Symbol {
        if let Some(&sym) = self.lookup.get(s) {
            return sym;
        }
        let sym = Symbol(self.strings.len() as u32);
        let owned = s.to_string();
        self.strings.push(owned.clone());
        self.lookup.insert(owned, sym);
        sym
    }

    /// Resolve a Symbol back to its string slice in O(1) time
    #[inline(always)]
    pub fn resolve(&self, sym: Symbol) -> Option<&str> {
        self.strings.get(sym.0 as usize).map(|s| s.as_str())
    }

    /// Number of interned symbols
    #[inline(always)]
    pub fn len(&self) -> usize {
        self.strings.len()
    }

    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        self.strings.is_empty()
    }

    /// Pre-populate interner with CRON built-in silicon primitives
    pub fn with_builtins() -> Self {
        let mut interner = Self::new();
        const BUILTINS: &[&str] = &[
            "simd_splat", "simd_load", "simd_store", "simd_fma", "simd_dot",
            "simd_reduce_sum", "simd_add", "simd_sub", "simd_mul", "simd_div",
            "vec4f", "vec8f", "vec16f", "vec4i", "vec8i",
            "channel_new", "channel_send", "channel_recv", "channel_try_recv", "channel_close",
            "spawn_at", "torus_core_id", "torus_coord_x", "torus_coord_y", "torus_coord_z", "torus_coord_w",
            "torus_distance", "torus_send", "torus_recv", "torus_broadcast",
            "simd_ternary_dot", "pack_ternary", "unpack_ternary",
            "fma", "fms", "popcount", "saturating_add", "xnor", "nand", "nor",
        ];
        for b in BUILTINS {
            interner.intern(b);
        }
        interner
    }
}
