// ============================================================================
// CRON .cl Neuromorphic Sensory HAL (DVS Vision & Spiking Cochlea Engine)
// Asynchronous Event-Driven Sensor Ingestion (< 1ms latency, < 10mW power)
// Zero-Copy Spatial DMA Injection directly into 256-Core 4D-Torus Mesh
// ============================================================================

use crate::cl_torus_coherence::Torus4DCoord;

/// An asynchronous DVS event (pixel intensity change).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DvsEvent {
    pub x: u16,
    pub y: u16,
    /// Polarity: +1 (ON event / brighten) or -1 / 0 (OFF event / darken)
    pub polarity: i8,
    /// Timestamp in microseconds
    pub timestamp_us: u64,
}

impl DvsEvent {
    pub fn new(x: u16, y: u16, polarity: i8, timestamp_us: u64) -> Self {
        Self {
            x,
            y,
            polarity,
            timestamp_us,
        }
    }

    /// Packs into a compact 32-bit hardware wire format:
    /// [31]: Polarity (1=ON, 0=OFF)
    /// [30..22]: X (9 bits, 0..511)
    /// [21..13]: Y (9 bits, 0..511)
    /// [12..0]: Timestamp offset (13 bits)
    pub fn to_u32_packet(&self) -> u32 {
        let p_bit = if self.polarity > 0 { 1u32 } else { 0u32 };
        let x_bits = (self.x as u32 & 0x1FF) << 22;
        let y_bits = (self.y as u32 & 0x1FF) << 13;
        let t_bits = (self.timestamp_us as u32) & 0x1FFF;
        (p_bit << 31) | x_bits | y_bits | t_bits
    }

    /// Unpacks a 32-bit hardware wire packet into a DvsEvent.
    pub fn from_u32_packet(packed: u32, base_timestamp_us: u64) -> Self {
        let p = if (packed >> 31) & 1 == 1 { 1 } else { -1 };
        let x = ((packed >> 22) & 0x1FF) as u16;
        let y = ((packed >> 13) & 0x1FF) as u16;
        let t_offset = (packed & 0x1FFF) as u64;
        Self {
            x,
            y,
            polarity: p,
            timestamp_us: base_timestamp_us + t_offset,
        }
    }

    /// Direct spatial mapping from 2D sensor pixel (x, y) to 4D-Torus core (X, Y, Z, W).
    pub fn map_to_torus_coord(&self, sensor_w: u16, sensor_h: u16) -> Torus4DCoord {
        let grid_w = 4;
        let grid_h = 4;
        let grid_z = 4;
        let grid_w_dim = 4;

        let norm_x = (self.x as usize * grid_w) / (sensor_w.max(1) as usize);
        let norm_y = (self.y as usize * grid_h) / (sensor_h.max(1) as usize);

        let x_core = (norm_x.min(grid_w - 1)) as u8;
        let y_core = (norm_y.min(grid_h - 1)) as u8;
        let z_core = ((self.polarity.abs() as usize) % grid_z) as u8;
        let w_core = ((self.timestamp_us as usize / 1000) % grid_w_dim) as u8;

        Torus4DCoord::new(x_core, y_core, z_core, w_core)
    }
}

/// An auditory bio-cochlea frequency-banded spike event.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CochleaSpike {
    /// Frequency channel index (0..127 frequency bands)
    pub channel: u8,
    /// Spiking amplitude / phase intensity
    pub intensity: u16,
    /// Timestamp in microseconds
    pub timestamp_us: u64,
}

impl CochleaSpike {
    pub fn new(channel: u8, intensity: u16, timestamp_us: u64) -> Self {
        Self {
            channel,
            intensity,
            timestamp_us,
        }
    }

    /// Tonotopic mapping from auditory frequency channel (0..127) to 4D Torus Auditory Cortex cores.
    pub fn map_to_torus_coord(&self) -> Torus4DCoord {
        let ch = self.channel as usize;
        let x = (ch % 4) as u8;
        let y = ((ch / 4) % 4) as u8;
        let z = ((ch / 16) % 4) as u8;
        let w = ((ch / 64) % 4) as u8;
        Torus4DCoord::new(x, y, z, w)
    }
}

/// Time-surface accumulator for optical event-stream feature extraction.
#[derive(Debug, Clone)]
pub struct TimeSurfaceAccumulator {
    pub width: usize,
    pub height: usize,
    pub decay_tau_us: f64,
    pub surface_on: Vec<u64>,
    pub surface_off: Vec<u64>,
}

impl TimeSurfaceAccumulator {
    pub fn new(width: usize, height: usize, decay_tau_us: f64) -> Self {
        let sz = width * height;
        Self {
            width,
            height,
            decay_tau_us: decay_tau_us.max(1.0),
            surface_on: vec![0; sz],
            surface_off: vec![0; sz],
        }
    }

    /// Accumulates an incoming DVS event into the time surface.
    pub fn update(&mut self, event: &DvsEvent) -> f64 {
        let xi = event.x as usize;
        let yi = event.y as usize;
        if xi >= self.width || yi >= self.height {
            return 0.0;
        }

        let idx = yi * self.width + xi;
        let last_t = if event.polarity > 0 {
            let t = self.surface_on[idx];
            self.surface_on[idx] = event.timestamp_us;
            t
        } else {
            let t = self.surface_off[idx];
            self.surface_off[idx] = event.timestamp_us;
            t
        };

        if last_t == 0 {
            1.0
        } else {
            let dt = (event.timestamp_us.saturating_sub(last_t)) as f64;
            (-dt / self.decay_tau_us).exp()
        }
    }

    /// Computes local region salience vector for 64-dim Cortical Patch.
    pub fn extract_patch_64(&self, center_x: usize, center_y: usize) -> [i16; 64] {
        let mut patch = [0i16; 64];
        let half_sz = 4; // 8x8 patch = 64 elements

        for dy in 0..8 {
            for dx in 0..8 {
                let px = center_x.saturating_add(dx).saturating_sub(half_sz);
                let py = center_y.saturating_add(dy).saturating_sub(half_sz);

                if px < self.width && py < self.height {
                    let idx = py * self.width + px;
                    let val = (self.surface_on[idx] % 256) as i16;
                    patch[dy * 8 + dx] = val;
                }
            }
        }
        patch
    }
}
