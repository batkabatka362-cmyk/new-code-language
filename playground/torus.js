// ============================================================================
// CRON 4D-Torus Mesh Visualizer — Interactive Canvas 3D Projection Engine
// Features: 256-core (4×4×4×4) neuromorphic mesh, mouse orbit controls,
// core inspection, 8-neighbor 4D mesh highlighting, packet routing, and spike injection.
// ============================================================================

class TorusVisualizer {
  constructor(canvas) {
    this.canvas = canvas;
    this.ctx = canvas.getContext('2d');
    this.cores = [];
    this.packets = [];
    this.activeCores = new Set();
    
    // View state
    this.rotationY = 0.4;
    this.rotationX = 0.35;
    this.autoRotate = true;
    this.zoom = 1.0;
    this.time = 0;
    this.scale = 1;

    // Interaction state
    this.selectedCoreId = 0;
    this.hoveredCoreId = null;
    this.isDragging = false;
    this.dragStartX = 0;
    this.dragStartY = 0;
    this.hasDragged = false;
    this.projectedCores = [];

    // Callbacks
    this.onCoreSelect = null;

    this._initCores();
    this._resize();
    this._bindEvents();
    window.addEventListener('resize', () => this._resize());
  }

  _resize() {
    const rect = this.canvas.parentElement.getBoundingClientRect();
    const dpr = window.devicePixelRatio || 1;
    this.canvas.width = rect.width * dpr;
    this.canvas.height = rect.height * dpr;
    this.canvas.style.width = rect.width + 'px';
    this.canvas.style.height = rect.height + 'px';
    this.ctx.setTransform(dpr, 0, 0, dpr, 0, 0);
    this.w = rect.width;
    this.h = rect.height;
    this.cx = this.w / 2;
    this.cy = this.h / 2;
    this.scale = Math.min(this.w, this.h) * 0.28 * this.zoom;
  }

  _initCores() {
    this.cores = [];
    for (let w = 0; w < 4; w++) {
      for (let z = 0; z < 4; z++) {
        for (let y = 0; y < 4; y++) {
          for (let x = 0; x < 4; x++) {
            const id = x + 4 * y + 16 * z + 64 * w;
            this.cores.push({
              id, x, y, z, w,
              wx: (x - 1.5) * 1.0,
              wy: (y - 1.5) * 1.0,
              wz: (z - 1.5) * 1.0,
              ww: (w - 1.5) * 0.6,
              activation: 0,
              temperature: 36.5 + (id % 15) * 0.5,
              queueLength: 0,
            });
          }
        }
      }
    }
  }

  _bindEvents() {
    this.canvas.addEventListener('mousedown', (e) => {
      this.isDragging = true;
      this.dragStartX = e.clientX;
      this.dragStartY = e.clientY;
      this.hasDragged = false;
      this.canvas.style.cursor = 'grabbing';
    });

    window.addEventListener('mousemove', (e) => {
      if (this.isDragging) {
        const dx = e.clientX - this.dragStartX;
        const dy = e.clientY - this.dragStartY;
        if (Math.abs(dx) > 2 || Math.abs(dy) > 2) {
          this.hasDragged = true;
        }
        this.rotationY += dx * 0.007;
        this.rotationX += dy * 0.007;
        this.rotationX = Math.max(-1.4, Math.min(1.4, this.rotationX));
        this.dragStartX = e.clientX;
        this.dragStartY = e.clientY;
      }
    });

    this.canvas.addEventListener('mousemove', (e) => {
      if (this.isDragging) return;
      const rect = this.canvas.getBoundingClientRect();
      const mouseX = e.clientX - rect.left;
      const mouseY = e.clientY - rect.top;

      let closest = null;
      let minDst = 16;

      for (const c of this.projectedCores) {
        const dst = Math.hypot(c.sx - mouseX, c.sy - mouseY);
        if (dst < minDst) {
          minDst = dst;
          closest = c;
        }
      }

      this.hoveredCoreId = closest ? closest.id : null;
      this.canvas.style.cursor = this.hoveredCoreId !== null ? 'pointer' : 'grab';
    });

    window.addEventListener('mouseup', () => {
      if (this.isDragging) {
        this.isDragging = false;
        this.canvas.style.cursor = 'grab';
      }
    });

    this.canvas.addEventListener('mouseleave', () => {
      this.hoveredCoreId = null;
    });

    this.canvas.addEventListener('click', (e) => {
      if (this.hasDragged) return;
      const rect = this.canvas.getBoundingClientRect();
      const mouseX = e.clientX - rect.left;
      const mouseY = e.clientY - rect.top;

      let closest = null;
      let minDst = 18;

      for (const c of this.projectedCores) {
        const dst = Math.hypot(c.sx - mouseX, c.sy - mouseY);
        if (dst < minDst) {
          minDst = dst;
          closest = c;
        }
      }

      if (closest !== null) {
        this.selectCore(closest.id);
      }
    });

    this.canvas.addEventListener('wheel', (e) => {
      e.preventDefault();
      const factor = e.deltaY < 0 ? 1.08 : 0.92;
      this.zoom = Math.max(0.4, Math.min(3.0, this.zoom * factor));
      this.scale = Math.min(this.w, this.h) * 0.28 * this.zoom;
    }, { passive: false });
  }

  _project(wx, wy, wz, ww) {
    // Rotate around Y axis
    const cosY = Math.cos(this.rotationY);
    const sinY = Math.sin(this.rotationY);
    let rx = wx * cosY - wz * sinY;
    let rz = wx * sinY + wz * cosY;

    // Rotate around X axis
    const cosX = Math.cos(this.rotationX);
    const sinX = Math.sin(this.rotationX);
    let ry = wy * cosX - rz * sinX;
    let rz2 = wy * sinX + rz * cosX;

    // W-dimension mapped as spatial offset & size variation
    const wOffset = ww * 0.3;
    rx += wOffset * 0.5;
    ry -= wOffset * 0.3;

    // Perspective projection
    const fov = 4.0;
    const depth = fov / (fov + rz2 + 2.0);
    const sx = this.cx + rx * this.scale * depth;
    const sy = this.cy + ry * this.scale * depth;

    return { sx, sy, depth, rz: rz2 };
  }

  getNeighbors(id) {
    const c = this.cores[id];
    if (!c) return [];
    const axes = [
      { axis: 'X+', x: (c.x + 1) % 4, y: c.y, z: c.z, w: c.w },
      { axis: 'X-', x: (c.x + 3) % 4, y: c.y, z: c.z, w: c.w },
      { axis: 'Y+', x: c.x, y: (c.y + 1) % 4, z: c.z, w: c.w },
      { axis: 'Y-', x: c.x, y: (c.y + 3) % 4, z: c.z, w: c.w },
      { axis: 'Z+', x: c.x, y: c.y, z: (c.z + 1) % 4, w: c.w },
      { axis: 'Z-', x: c.x, y: c.y, z: (c.z + 3) % 4, w: c.w },
      { axis: 'W+', x: c.x, y: c.y, z: c.z, w: (c.w + 1) % 4 },
      { axis: 'W-', x: c.x, y: c.y, z: c.z, w: (c.w + 3) % 4 },
    ];
    return axes.map(a => ({
      axis: a.axis,
      targetId: a.x + 4 * a.y + 16 * a.z + 64 * a.w,
      coord: `[${a.x},${a.y},${a.z},${a.w}]`
    }));
  }

  getCoreInfo(id) {
    const c = this.cores[id];
    if (!c) return null;
    const subsystems = [
      'Layer 0 • Photonic GEMM & Attention (Brain 2)',
      'Layer 1 • Quantum Phase & Reversible Logic (Brain 3)',
      'Layer 2 • Neuromorphic STDP Plasticity (Brain 4)',
      'Layer 3 • Metacognitive Sentry & Arbiter (Brain 6)',
    ];
    return {
      id: c.id,
      x: c.x,
      y: c.y,
      z: c.z,
      w: c.w,
      temperature: c.temperature.toFixed(1),
      activation: Math.round(c.activation * 100),
      subsystem: subsystems[c.w] || 'General Coprocessor',
      neighbors: this.getNeighbors(c.id),
    };
  }

  selectCore(id) {
    if (id >= 0 && id < 256) {
      this.selectedCoreId = id;
      if (this.onCoreSelect) {
        this.onCoreSelect(this.getCoreInfo(id));
      }
    }
  }

  pingNeighbors(coreId) {
    const neighbors = this.getNeighbors(coreId);
    const colors = ['#00f5ff', '#ff00ff', '#00ff88', '#ffb800'];
    neighbors.forEach((n, idx) => {
      this.sendPacket(coreId, n.targetId, colors[idx % colors.length]);
    });
    this.activateCore(coreId, 1.0);
  }

  injectSpike(coreId) {
    this.activateCore(coreId, 1.0);
    const c = this.cores[coreId];
    if (c) {
      c.temperature = Math.min(110, c.temperature + 12.0);
      const neighbors = this.getNeighbors(coreId);
      for (let i = 0; i < 3; i++) {
        const n = neighbors[Math.floor(Math.random() * neighbors.length)];
        this.sendPacket(coreId, n.targetId, '#ff4466');
      }
    }
    if (this.selectedCoreId === coreId && this.onCoreSelect) {
      this.onCoreSelect(this.getCoreInfo(coreId));
    }
  }

  resetView() {
    this.rotationX = 0.35;
    this.rotationY = 0.4;
    this.zoom = 1.0;
    this.autoRotate = true;
    this.scale = Math.min(this.w, this.h) * 0.28;
  }

  activateCore(coreId, level = 1.0) {
    if (coreId >= 0 && coreId < 256) {
      this.cores[coreId].activation = level;
      this.cores[coreId].temperature = Math.min(105, this.cores[coreId].temperature + 1.2);
      this.activeCores.add(coreId);
    }
  }

  sendPacket(fromId, toId, color = '#00f5ff') {
    this.packets.push({
      from: fromId,
      to: toId,
      progress: 0,
      color,
      speed: 0.02 + Math.random() * 0.015,
    });
  }

  resetActivations() {
    this.activeCores.clear();
    this.cores.forEach(c => {
      c.activation = 0;
      c.temperature = Math.max(36.5, c.temperature - 0.5);
    });
    this.packets = [];
  }

  render() {
    this.time += 0.016;
    if (this.autoRotate && !this.isDragging) {
      this.rotationY += 0.003;
    }

    const ctx = this.ctx;
    ctx.clearRect(0, 0, this.w, this.h);

    // Project all cores
    const projected = this.cores.map(c => {
      const p = this._project(c.wx, c.wy, c.wz, c.ww);
      return { ...c, ...p };
    });
    this.projectedCores = projected;

    // Fast lookup map for screen coordinates
    const coreMap = new Map();
    for (const p of projected) coreMap.set(p.id, p);

    // Determine neighbor IDs of selected core
    const selectedNeighbors = this.selectedCoreId !== null ? new Set(this.getNeighbors(this.selectedCoreId).map(n => n.targetId)) : new Set();

    // Sort by depth (far to near) for painter's algorithm
    projected.sort((a, b) => a.depth - b.depth);

    // Draw regular mesh edges
    ctx.lineWidth = 0.5;
    for (const c of projected) {
      for (const [dx, dy, dz] of [[1,0,0],[0,1,0],[0,0,1]]) {
        const nx = (c.x + dx) % 4;
        const ny = (c.y + dy) % 4;
        const nz = (c.z + dz) % 4;
        const nid = nx + 4 * ny + 16 * nz + 64 * c.w;
        const neighbor = coreMap.get(nid);
        if (neighbor && c.id < nid) {
          const isSelectedEdge = (c.id === this.selectedCoreId && selectedNeighbors.has(nid)) ||
                                 (nid === this.selectedCoreId && selectedNeighbors.has(c.id));
          if (!isSelectedEdge) {
            const alpha = Math.min(c.depth, neighbor.depth) * 0.12;
            ctx.strokeStyle = `rgba(100, 160, 255, ${alpha})`;
            ctx.beginPath();
            ctx.moveTo(c.sx, c.sy);
            ctx.lineTo(neighbor.sx, neighbor.sy);
            ctx.stroke();
          }
        }
      }
    }

    // Highlight selected core's 8 neighbor connections with glowing lines!
    if (this.selectedCoreId !== null) {
      const sel = coreMap.get(this.selectedCoreId);
      if (sel) {
        ctx.save();
        for (const n of this.getNeighbors(this.selectedCoreId)) {
          const target = coreMap.get(n.targetId);
          if (target) {
            const isWJump = n.axis.startsWith('W');
            ctx.strokeStyle = isWJump ? 'rgba(255, 0, 255, 0.7)' : 'rgba(0, 245, 255, 0.8)';
            ctx.lineWidth = isWJump ? 1.2 : 1.6;
            ctx.shadowColor = isWJump ? '#ff00ff' : '#00f5ff';
            ctx.shadowBlur = 8;
            ctx.setLineDash(isWJump ? [4, 4] : []);
            ctx.beginPath();
            ctx.moveTo(sel.sx, sel.sy);
            ctx.lineTo(target.sx, target.sy);
            ctx.stroke();
          }
        }
        ctx.restore();
      }
    }

    // Draw packets
    this.packets = this.packets.filter(pkt => {
      pkt.progress += pkt.speed;
      if (pkt.progress >= 1) return false;
      const from = this.cores[pkt.from];
      const to = this.cores[pkt.to];
      if (!from || !to) return false;

      const t = pkt.progress;
      const mx = from.wx + (to.wx - from.wx) * t;
      const my = from.wy + (to.wy - from.wy) * t;
      const mz = from.wz + (to.wz - from.wz) * t;
      const mw = from.ww + (to.ww - from.ww) * t;
      const p = this._project(mx, my, mz, mw);

      const r = 3.2 * p.depth;
      ctx.beginPath();
      ctx.arc(p.sx, p.sy, r, 0, Math.PI * 2);
      ctx.fillStyle = pkt.color;
      ctx.shadowColor = pkt.color;
      ctx.shadowBlur = 10;
      ctx.fill();
      ctx.shadowBlur = 0;
      return true;
    });

    // Draw cores
    for (const c of projected) {
      const baseR = 2.5 * c.depth;
      let r = baseR;
      let color = `rgba(100, 160, 255, ${0.15 + c.depth * 0.2})`;
      let glow = 0;
      const isSelected = c.id === this.selectedCoreId;
      const isHovered = c.id === this.hoveredCoreId;
      const isNeighbor = selectedNeighbors.has(c.id);

      if (c.activation > 0) {
        const pulse = 0.7 + 0.3 * Math.sin(this.time * 4 + c.id * 0.3);
        r = baseR * (1.2 + 0.5 * c.activation * pulse);
        const a = c.activation * pulse;

        // Color by W dimension (brain layer)
        const colors = [
          [0, 245, 255],   // W=0: Cyan
          [255, 0, 255],   // W=1: Magenta
          [0, 255, 136],   // W=2: Green
          [255, 184, 0],   // W=3: Amber
        ];
        const [cr, cg, cb] = colors[c.w] || colors[0];
        color = `rgba(${cr}, ${cg}, ${cb}, ${0.6 + a * 0.4})`;
        glow = 10 * a;
      }

      // Draw neighbor indicator halo
      if (isNeighbor && !isSelected) {
        ctx.beginPath();
        ctx.arc(c.sx, c.sy, r * 1.8, 0, Math.PI * 2);
        ctx.strokeStyle = 'rgba(0, 245, 255, 0.45)';
        ctx.lineWidth = 1.0;
        ctx.stroke();
      }

      // Draw core dot
      if (glow > 0) {
        ctx.shadowColor = color;
        ctx.shadowBlur = glow;
      }

      ctx.beginPath();
      ctx.arc(c.sx, c.sy, Math.max(r, 1), 0, Math.PI * 2);
      ctx.fillStyle = color;
      ctx.fill();
      ctx.shadowBlur = 0;

      // Draw Selected Reticle Target
      if (isSelected) {
        ctx.save();
        const pulse = 1.0 + 0.15 * Math.sin(this.time * 6);
        const ringR = Math.max(10, r * 3.5 * pulse);

        // Outer pulsing ring
        ctx.beginPath();
        ctx.arc(c.sx, c.sy, ringR, 0, Math.PI * 2);
        ctx.strokeStyle = '#00f5ff';
        ctx.lineWidth = 1.5;
        ctx.shadowColor = '#00f5ff';
        ctx.shadowBlur = 12;
        ctx.stroke();

        // Inner ring
        ctx.beginPath();
        ctx.arc(c.sx, c.sy, ringR * 0.6, 0, Math.PI * 2);
        ctx.strokeStyle = 'rgba(255, 255, 255, 0.8)';
        ctx.lineWidth = 1.0;
        ctx.stroke();

        // Crosshairs tick marks
        const tick = 4;
        ctx.beginPath();
        ctx.moveTo(c.sx - ringR - tick, c.sy);
        ctx.lineTo(c.sx - ringR + tick, c.sy);
        ctx.moveTo(c.sx + ringR - tick, c.sy);
        ctx.lineTo(c.sx + ringR + tick, c.sy);
        ctx.moveTo(c.sx, c.sy - ringR - tick);
        ctx.lineTo(c.sx, c.sy - ringR + tick);
        ctx.moveTo(c.sx, c.sy + ringR - tick);
        ctx.lineTo(c.sx, c.sy + ringR + tick);
        ctx.strokeStyle = '#00f5ff';
        ctx.lineWidth = 1.5;
        ctx.stroke();

        // Core label
        ctx.font = 'bold 9px JetBrains Mono, monospace';
        ctx.fillStyle = '#00f5ff';
        ctx.textAlign = 'center';
        ctx.fillText(`#${c.id}`, c.sx, c.sy - ringR - 6);

        ctx.restore();
      }

      // Draw Hover ring & tooltip
      if (isHovered && !isSelected) {
        ctx.save();
        ctx.beginPath();
        ctx.arc(c.sx, c.sy, Math.max(8, r * 2.2), 0, Math.PI * 2);
        ctx.strokeStyle = '#ffffff';
        ctx.lineWidth = 1.2;
        ctx.shadowColor = '#ffffff';
        ctx.shadowBlur = 8;
        ctx.stroke();

        ctx.font = '10px Inter, sans-serif';
        ctx.fillStyle = '#ffffff';
        ctx.textAlign = 'center';
        ctx.fillText(`Core #${c.id}`, c.sx, c.sy - r * 2.2 - 5);
        ctx.restore();
      }
    }

    // Legend
    ctx.font = '10px Inter, sans-serif';
    ctx.fillStyle = 'rgba(100, 116, 139, 0.6)';
    ctx.textAlign = 'left';
    ctx.fillText('W=0 ● Cyan (Brain 2)  W=1 ● Magenta (Brain 3)  W=2 ● Green (Brain 4)  W=3 ● Amber (Brain 6)', 10, this.h - 8);
  }
}
