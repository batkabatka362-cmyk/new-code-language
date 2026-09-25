// ============================================================================
// CRON .cl 1.58-Bit Ternary SIMD Engine (Sub-Byte Ultra-Low Power AI)
// 64 trits per 128-bit VLIW Register (2 bits per trit: 00=0, 01=+1, 11=-1)
// Zero-Multiplication Matrix Multiplication via Pure Addition/Subtraction Networks
// Target: Neuromorphic SRAM Vector Lanes (< 0.5 pJ per MAC)
// ============================================================================

/// A 2-bit representation of a balanced ternary digit (trit in {-1, 0, +1}).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TernaryTrit {
    Zero = 0b00,
    Plus = 0b01,
    Minus = 0b11,
}

impl TernaryTrit {
    #[inline(always)]
    pub fn from_i8(val: i8) -> Self {
        match val {
            v if v > 0 => TernaryTrit::Plus,
            v if v < 0 => TernaryTrit::Minus,
            _ => TernaryTrit::Zero,
        }
    }

    #[inline(always)]
    pub fn to_i8(self) -> i8 {
        match self {
            TernaryTrit::Zero => 0,
            TernaryTrit::Plus => 1,
            TernaryTrit::Minus => -1,
        }
    }

    #[inline(always)]
    pub fn from_bits(bits: u8) -> Self {
        match bits & 0b11 {
            0b01 => TernaryTrit::Plus,
            0b11 => TernaryTrit::Minus,
            _ => TernaryTrit::Zero,
        }
    }

    #[inline(always)]
    pub fn to_bits(self) -> u8 {
        self as u8
    }
}

/// A 128-bit register packed with 64 balanced trits.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TritWord128 {
    pub low: u64,
    pub high: u64,
}

impl TritWord128 {
    pub const ZERO: Self = Self { low: 0, high: 0 };

    /// Packs a slice of up to 64 i8 values (-1, 0, +1) into a 128-bit word.
    pub fn pack(trits: &[i8]) -> Self {
        let mut low: u64 = 0;
        let mut high: u64 = 0;

        for (i, &val) in trits.iter().take(64).enumerate() {
            let trit = TernaryTrit::from_i8(val);
            let bits = (trit.to_bits() as u64) & 0b11;
            if i < 32 {
                low |= bits << (i * 2);
            } else {
                let idx = i - 32;
                high |= bits << (idx * 2);
            }
        }

        Self { low, high }
    }

    /// Unpacks the 64 trits into an array of i8.
    pub fn unpack(&self) -> [i8; 64] {
        let mut res = [0i8; 64];
        for i in 0..32 {
            let bits = ((self.low >> (i * 2)) & 0b11) as u8;
            res[i] = TernaryTrit::from_bits(bits).to_i8();
        }
        for i in 0..32 {
            let bits = ((self.high >> (i * 2)) & 0b11) as u8;
            res[32 + i] = TernaryTrit::from_bits(bits).to_i8();
        }
        res
    }

    /// Retrieves a single trit at index 0..63.
    #[inline(always)]
    pub fn get_trit(&self, index: usize) -> TernaryTrit {
        if index < 32 {
            let bits = ((self.low >> (index * 2)) & 0b11) as u8;
            TernaryTrit::from_bits(bits)
        } else if index < 64 {
            let idx = index - 32;
            let bits = ((self.high >> (idx * 2)) & 0b11) as u8;
            TernaryTrit::from_bits(bits)
        } else {
            TernaryTrit::Zero
        }
    }

    /// Counts non-zero trits (sparsity tracker).
    pub fn non_zero_count(&self) -> usize {
        let mut count = 0;
        for i in 0..64 {
            if self.get_trit(i) != TernaryTrit::Zero {
                count += 1;
            }
        }
        count
    }
}

/// High-throughput Zero-Multiplication Dot Product & Matrix Multiplication
pub struct TritSimdEngine;

impl TritSimdEngine {
    /// Zero-Multiplication Dot Product between 64 packed trits and 64 fixed-point (Q8.8 or i16) activations.
    /// Operates purely with parallel SIMD addition/subtraction accumulation lanes.
    #[inline]
    pub fn dot_product_i16(weights: &TritWord128, activations: &[i16; 64]) -> i32 {
        let mut acc: i32 = 0;

        // Process low 32 trits
        for i in 0..32 {
            let bits = ((weights.low >> (i * 2)) & 0b11) as u8;
            match bits {
                0b01 => acc += activations[i] as i32,
                0b11 => acc -= activations[i] as i32,
                _ => {} // 0b00 or 0b10: zero multiplication, no operation needed
            }
        }

        // Process high 32 trits
        for i in 0..32 {
            let bits = ((weights.high >> (i * 2)) & 0b11) as u8;
            match bits {
                0b01 => acc += activations[32 + i] as i32,
                0b11 => acc -= activations[32 + i] as i32,
                _ => {}
            }
        }

        acc
    }

    /// Ternary Matrix-Vector Multiplication:
    /// rows: array of packed TritWord128 weight vectors (one per output neuron)
    /// activations: input feature vector of length 64 (i16 fixed-point)
    /// returns: output neuron activation potentials
    pub fn matvec_mul(rows: &[TritWord128], activations: &[i16; 64]) -> Vec<i32> {
        let mut out = Vec::with_capacity(rows.len());
        for row in rows {
            out.push(Self::dot_product_i16(row, activations));
        }
        out
    }

    /// 1D Ternary Convolution over a temporal stream of activations.
    /// kernel: packed 64-trit filter
    /// signal: array of length N (i16)
    pub fn conv1d(kernel: &TritWord128, signal: &[i16], stride: usize) -> Vec<i32> {
        let k_len = 64;
        if signal.len() < k_len {
            return Vec::new();
        }
        let mut output = Vec::new();
        let mut i = 0;
        while i + k_len <= signal.len() {
            let mut window = [0i16; 64];
            window.copy_from_slice(&signal[i..i + k_len]);
            let res = Self::dot_product_i16(kernel, &window);
            output.push(res);
            i += stride.max(1);
        }
        output
    }

    /// Computes hardware energy metrics comparison:
    /// Returns (traditional_fp16_energy_pj, ternary_simd_energy_pj, energy_reduction_factor)
    pub fn energy_metric(mac_ops: usize, sparsity_ratio: f64) -> (f64, f64, f64) {
        // Standard FP16 MAC on modern 7nm/5nm silicon: ~1.5 pJ per MAC
        let fp16_energy_pj = mac_ops as f64 * 1.5;
        // Ternary Zero-Mul SRAM accumulator (only active on non-zero trits): ~0.03 pJ per Add/Sub
        let active_ops = mac_ops as f64 * (1.0 - sparsity_ratio.clamp(0.0, 1.0));
        let ternary_energy_pj = active_ops * 0.03 + (mac_ops as f64 * 0.005); // minor routing overhead
        let reduction = if ternary_energy_pj > 0.0 {
            fp16_energy_pj / ternary_energy_pj
        } else {
            100.0
        };
        (fp16_energy_pj, ternary_energy_pj, reduction)
    }
}
