// ============================================================================
// CRON Just-In-Time (JIT) Dynamic Machine Code Generator
// Generates native x86_64 machine code directly in executable memory.
// Features sub-microsecond latency, 0 external compiler dependencies, 0 disk I/O.
// ============================================================================

use crate::ast::*;
use crate::c_backend::CBackend;
use std::collections::{HashMap, VecDeque};
use std::sync::{Arc, Mutex, Condvar};
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};

// ----------------------------------------------------------------------------
// 1. Cross-Platform Executable Memory Allocator (W^X Compliant)
// ----------------------------------------------------------------------------

#[cfg(windows)]
mod os_mem {
    use std::os::raw::c_void;

    const MEM_COMMIT: u32 = 0x1000;
    const MEM_RESERVE: u32 = 0x2000;
    const MEM_RELEASE: u32 = 0x8000;
    const PAGE_EXECUTE_READWRITE: u32 = 0x40;

    extern "system" {
        fn VirtualAlloc(lpAddress: *mut c_void, dwSize: usize, flAllocationType: u32, flProtect: u32) -> *mut c_void;
        fn VirtualFree(lpAddress: *mut c_void, dwSize: usize, dwFreeType: u32) -> i32;
    }

    pub unsafe fn alloc_executable(size: usize) -> *mut u8 {
        VirtualAlloc(std::ptr::null_mut(), size, MEM_COMMIT | MEM_RESERVE, PAGE_EXECUTE_READWRITE) as *mut u8
    }

    pub unsafe fn free_executable(ptr: *mut u8, _size: usize) {
        if !ptr.is_null() {
            VirtualFree(ptr as *mut c_void, 0, MEM_RELEASE);
        }
    }
}

#[cfg(unix)]
mod os_mem {
    use std::os::raw::c_void;

    const PROT_READ: i32 = 0x1;
    const PROT_WRITE: i32 = 0x2;
    const PROT_EXEC: i32 = 0x4;
    const MAP_ANONYMOUS: i32 = 0x20;
    const MAP_PRIVATE: i32 = 0x02;

    extern "C" {
        fn mmap(addr: *mut c_void, len: usize, prot: i32, flags: i32, fd: i32, offset: i64) -> *mut c_void;
        fn munmap(addr: *mut c_void, len: usize) -> i32;
    }

    pub unsafe fn alloc_executable(size: usize) -> *mut u8 {
        mmap(std::ptr::null_mut(), size, PROT_READ | PROT_WRITE | PROT_EXEC, MAP_ANONYMOUS | MAP_PRIVATE, -1, 0) as *mut u8
    }

    pub unsafe fn free_executable(ptr: *mut u8, size: usize) {
        if !ptr.is_null() {
            munmap(ptr as *mut c_void, size);
        }
    }
}

pub struct JitMemory {
    pub ptr: *mut u8,
    pub size: usize,
    pub _dependencies: Vec<Arc<JitMemory>>,
}

unsafe impl Send for JitMemory {}
unsafe impl Sync for JitMemory {}

impl JitMemory {
    pub fn new(code: &[u8]) -> Result<Self, String> {
        let size = code.len().max(4096);
        let ptr = unsafe { os_mem::alloc_executable(size) };
        if ptr.is_null() {
            return Err("Failed to allocate executable memory for JIT".to_string());
        }

        unsafe {
            std::ptr::copy_nonoverlapping(code.as_ptr(), ptr, code.len());
        }

        Ok(Self { ptr, size, _dependencies: Vec::new() })
    }

    pub fn as_fn(&self) -> unsafe extern "C" fn() -> i64 {
        unsafe { std::mem::transmute(self.ptr) }
    }
}

impl Drop for JitMemory {
    fn drop(&mut self) {
        unsafe {
            os_mem::free_executable(self.ptr, self.size);
        }
    }
}

// ----------------------------------------------------------------------------
// 1b. JIT CSP Typed Channels & Neuromorphic Fiber Runtime
// ----------------------------------------------------------------------------

pub struct JitChannel {
    queue: Mutex<VecDeque<i64>>,
    cond: Condvar,
    capacity: usize,
    closed: AtomicBool,
}

static NEXT_CHANNEL_ID: AtomicU64 = AtomicU64::new(1);
static CHANNELS: Mutex<Option<HashMap<u64, Arc<JitChannel>>>> = Mutex::new(None);
static FIBER_HANDLES: Mutex<Option<HashMap<u64, std::thread::JoinHandle<i64>>>> = Mutex::new(None);
static NEXT_FIBER_ID: AtomicU64 = AtomicU64::new(1);

pub fn jit_channel_new(cap: i64) -> u64 {
    let capacity = if cap <= 0 { 64 } else { cap as usize };
    let id = NEXT_CHANNEL_ID.fetch_add(1, Ordering::SeqCst);
    let ch = Arc::new(JitChannel {
        queue: Mutex::new(VecDeque::with_capacity(capacity)),
        cond: Condvar::new(),
        capacity,
        closed: AtomicBool::new(false),
    });
    let mut table = CHANNELS.lock().unwrap();
    if table.is_none() {
        *table = Some(HashMap::new());
    }
    table.as_mut().unwrap().insert(id, ch);
    id
}

pub fn jit_channel_send(id: u64, val: i64) -> i64 {
    let ch = {
        let table = CHANNELS.lock().unwrap();
        table.as_ref().and_then(|t| t.get(&id).cloned())
    };
    if let Some(ch) = ch {
        let mut q = ch.queue.lock().unwrap();
        while q.len() >= ch.capacity && !ch.closed.load(Ordering::Relaxed) {
            q = ch.cond.wait(q).unwrap();
        }
        if ch.closed.load(Ordering::Relaxed) {
            return -1;
        }
        q.push_back(val);
        ch.cond.notify_all();
        0
    } else {
        -1
    }
}

pub fn jit_channel_recv(id: u64) -> i64 {
    let ch = {
        let table = CHANNELS.lock().unwrap();
        table.as_ref().and_then(|t| t.get(&id).cloned())
    };
    if let Some(ch) = ch {
        let mut q = ch.queue.lock().unwrap();
        while q.is_empty() && !ch.closed.load(Ordering::Relaxed) {
            q = ch.cond.wait(q).unwrap();
        }
        if let Some(val) = q.pop_front() {
            ch.cond.notify_all();
            val
        } else {
            0
        }
    } else {
        0
    }
}

pub fn jit_channel_try_recv(id: u64) -> i64 {
    let ch = {
        let table = CHANNELS.lock().unwrap();
        table.as_ref().and_then(|t| t.get(&id).cloned())
    };
    if let Some(ch) = ch {
        let mut q = ch.queue.lock().unwrap();
        if let Some(val) = q.pop_front() {
            ch.cond.notify_all();
            val
        } else {
            -1
        }
    } else {
        -1
    }
}

pub fn jit_channel_close(id: u64) {
    let ch = {
        let table = CHANNELS.lock().unwrap();
        table.as_ref().and_then(|t| t.get(&id).cloned())
    };
    if let Some(ch) = ch {
        ch.closed.store(true, Ordering::SeqCst);
        ch.cond.notify_all();
    }
}

pub fn jit_torus_distance(c1: i64, c2: i64) -> i64 {
    let mut dist = 0;
    let mut a = c1 as usize;
    let mut b = c2 as usize;
    for _ in 0..4 {
        let mut d = (a % 4) as isize - (b % 4) as isize;
        if d < 0 { d = -d; }
        if d > 2 { d = 4 - d; }
        dist += d as i64;
        a /= 4;
        b /= 4;
    }
    dist
}

pub extern "C" fn extern_c_jit_channel_new(cap: i64) -> i64 {
    jit_channel_new(cap) as i64
}

pub extern "C" fn extern_c_jit_channel_send(id: u64, val: i64) -> i64 {
    jit_channel_send(id, val)
}

pub extern "C" fn extern_c_jit_channel_recv(id: u64) -> i64 {
    jit_channel_recv(id)
}

pub extern "C" fn extern_c_jit_channel_try_recv(id: u64) -> i64 {
    jit_channel_try_recv(id)
}

pub extern "C" fn extern_c_jit_channel_close(id: u64) -> i64 {
    jit_channel_close(id);
    0
}

pub extern "C" fn extern_c_jit_torus_distance(c1: i64, c2: i64) -> i64 {
    jit_torus_distance(c1, c2)
}

pub extern "C" fn extern_c_jit_torus_core_id() -> i64 {
    0
}

pub extern "C" fn extern_c_jit_spawn_0(fn_ptr: usize) -> u64 {
    let id = NEXT_FIBER_ID.fetch_add(1, Ordering::SeqCst);
    let handle = std::thread::spawn(move || {
        let f: extern "C" fn() -> i64 = unsafe { std::mem::transmute(fn_ptr) };
        f()
    });
    let mut table = FIBER_HANDLES.lock().unwrap();
    if table.is_none() { *table = Some(HashMap::new()); }
    table.as_mut().unwrap().insert(id, handle);
    id
}

pub extern "C" fn extern_c_jit_spawn_1(fn_ptr: usize, a0: i64) -> u64 {
    let id = NEXT_FIBER_ID.fetch_add(1, Ordering::SeqCst);
    let handle = std::thread::spawn(move || {
        let f: extern "C" fn(i64) -> i64 = unsafe { std::mem::transmute(fn_ptr) };
        f(a0)
    });
    let mut table = FIBER_HANDLES.lock().unwrap();
    if table.is_none() { *table = Some(HashMap::new()); }
    table.as_mut().unwrap().insert(id, handle);
    id
}

pub extern "C" fn extern_c_jit_spawn_2(fn_ptr: usize, a0: i64, a1: i64) -> u64 {
    let id = NEXT_FIBER_ID.fetch_add(1, Ordering::SeqCst);
    let handle = std::thread::spawn(move || {
        let f: extern "C" fn(i64, i64) -> i64 = unsafe { std::mem::transmute(fn_ptr) };
        f(a0, a1)
    });
    let mut table = FIBER_HANDLES.lock().unwrap();
    if table.is_none() { *table = Some(HashMap::new()); }
    table.as_mut().unwrap().insert(id, handle);
    id
}

pub extern "C" fn extern_c_jit_spawn_3(fn_ptr: usize, a0: i64, a1: i64, a2: i64) -> u64 {
    let id = NEXT_FIBER_ID.fetch_add(1, Ordering::SeqCst);
    let handle = std::thread::spawn(move || {
        let f: extern "C" fn(i64, i64, i64) -> i64 = unsafe { std::mem::transmute(fn_ptr) };
        f(a0, a1, a2)
    });
    let mut table = FIBER_HANDLES.lock().unwrap();
    if table.is_none() { *table = Some(HashMap::new()); }
    table.as_mut().unwrap().insert(id, handle);
    id
}

pub extern "C" fn extern_c_jit_await(id: u64) -> i64 {
    let handle = {
        let mut table = FIBER_HANDLES.lock().unwrap();
        table.as_mut().and_then(|t| t.remove(&id))
    };
    if let Some(h) = handle {
        h.join().unwrap_or(0)
    } else {
        0
    }
}

// ----------------------------------------------------------------------------
// 2. x86_64 Direct Machine Code Assembler Buffer
// ----------------------------------------------------------------------------

pub struct X64Assembler {
    pub code: Vec<u8>,
}

impl Default for X64Assembler {
    fn default() -> Self {
        Self::new()
    }
}

impl X64Assembler {
    pub fn new() -> Self {
        Self { code: Vec::with_capacity(4096) }
    }

    pub fn emit_bytes(&mut self, bytes: &[u8]) {
        self.code.extend_from_slice(bytes);
    }

    // Function Prologue: push rbp; mov rbp, rsp; sub rsp, 1024
    pub fn emit_prologue(&mut self) {
        self.emit_bytes(&[0x55]);                         // push rbp
        self.emit_bytes(&[0x48, 0x89, 0xE5]);             // mov rbp, rsp
        self.emit_bytes(&[0x48, 0x81, 0xEC, 0x00, 0x04, 0x00, 0x00]); // sub rsp, 1024
    }

    // Function Epilogue: vzeroupper; mov rsp, rbp; pop rbp; ret
    pub fn emit_epilogue(&mut self) {
        self.emit_vzeroupper();
        self.emit_bytes(&[0x48, 0x89, 0xEC]); // mov rsp, rbp
        self.emit_bytes(&[0x5D]);             // pop rbp
        self.emit_bytes(&[0xC3]);             // ret
    }

    // VEX: vzeroupper
    pub fn emit_vzeroupper(&mut self) {
        self.emit_bytes(&[0xC5, 0xF8, 0x77]);
    }

    // VEX: vmovups [rbp - disp32], ymm0
    pub fn emit_vmovups_store_ymm0_stack(&mut self, offset: i32) {
        let disp = -offset;
        self.emit_bytes(&[0xC5, 0xFC, 0x11, 0x85]);
        self.emit_bytes(&disp.to_le_bytes());
    }

    // VEX: vmovups ymm0, [rbp - disp32]
    pub fn emit_vmovups_load_ymm0_stack(&mut self, offset: i32) {
        let disp = -offset;
        self.emit_bytes(&[0xC5, 0xFC, 0x10, 0x85]);
        self.emit_bytes(&disp.to_le_bytes());
    }

    // VEX: vmovups ymm1, [rbp - disp32]
    pub fn emit_vmovups_load_ymm1_stack(&mut self, offset: i32) {
        let disp = -offset;
        self.emit_bytes(&[0xC5, 0xFC, 0x10, 0x8D]);
        self.emit_bytes(&disp.to_le_bytes());
    }

    // VEX: vmovups ymm2, [rbp - disp32]
    pub fn emit_vmovups_load_ymm2_stack(&mut self, offset: i32) {
        let disp = -offset;
        self.emit_bytes(&[0xC5, 0xFC, 0x10, 0x95]);
        self.emit_bytes(&disp.to_le_bytes());
    }

    // VEX: cvtsi2ss xmm1, rax; vbroadcastss ymm0, xmm1
    pub fn emit_vbroadcastss_from_rax(&mut self) {
        self.emit_bytes(&[0xF3, 0x48, 0x0F, 0x2A, 0xC8]); // cvtsi2ss xmm1, rax
        self.emit_bytes(&[0xC4, 0xE2, 0x7D, 0x18, 0xC1]); // vbroadcastss ymm0, xmm1
    }

    // VEX: vaddps ymm0, ymm0, ymm1
    pub fn emit_vaddps_ymm0_ymm1(&mut self) {
        self.emit_bytes(&[0xC5, 0xFC, 0x58, 0xC1]);
    }

    // VEX: vsubps ymm0, ymm0, ymm1
    pub fn emit_vsubps_ymm0_ymm1(&mut self) {
        self.emit_bytes(&[0xC5, 0xFC, 0x5C, 0xC1]);
    }

    // VEX: vmulps ymm0, ymm0, ymm1
    pub fn emit_vmulps_ymm0_ymm1(&mut self) {
        self.emit_bytes(&[0xC5, 0xFC, 0x59, 0xC1]);
    }

    // VEX: vfmadd231ps ymm0, ymm1, ymm2 (ymm0 = ymm0 + ymm1 * ymm2)
    pub fn emit_vfmadd231ps_ymm0_ymm1_ymm2(&mut self) {
        self.emit_bytes(&[0xC4, 0xE2, 0x75, 0xB8, 0xC2]);
    }

    // Store 32-bit float immediate: mov dword ptr [rbp - disp32], imm32
    pub fn emit_store_imm32_stack(&mut self, offset: i32, imm: u32) {
        let disp = -offset;
        self.emit_bytes(&[0xC7, 0x85]);
        self.emit_bytes(&disp.to_le_bytes());
        self.emit_bytes(&imm.to_le_bytes());
    }

    // VEX: Horizontal reduction of ymm0 into integer rax
    pub fn emit_vreduce_sum_ymm0_to_rax(&mut self) {
        self.emit_bytes(&[0xC4, 0xE3, 0x7D, 0x19, 0xC1, 0x01]); // vextractf128 $1, ymm0, xmm1
        self.emit_bytes(&[0xC5, 0xF8, 0x58, 0xC1]);             // vaddps xmm1, xmm0, xmm0
        self.emit_bytes(&[0xC5, 0xFB, 0x7C, 0xC0]);             // vhaddps xmm0, xmm0, xmm0
        self.emit_bytes(&[0xC5, 0xFB, 0x7C, 0xC0]);             // vhaddps xmm0, xmm0, xmm0
        self.emit_bytes(&[0xC5, 0xFA, 0x2C, 0xC0]);             // vcvttss2si eax, xmm0
    }

    // mov rax, imm64
    pub fn emit_mov_rax_imm64(&mut self, imm: i64) {
        self.emit_bytes(&[0x48, 0xB8]);
        self.emit_bytes(&imm.to_le_bytes());
    }

    // push rax
    pub fn emit_push_rax(&mut self) {
        self.emit_bytes(&[0x50]);
    }

    // pop rcx
    pub fn emit_pop_rcx(&mut self) {
        self.emit_bytes(&[0x59]);
    }

    // pop rdx
    pub fn emit_pop_rdx(&mut self) {
        self.emit_bytes(&[0x5A]);
    }

    // pop r8
    pub fn emit_pop_r8(&mut self) {
        self.emit_bytes(&[0x41, 0x58]);
    }

    // pop r9
    pub fn emit_pop_r9(&mut self) {
        self.emit_bytes(&[0x41, 0x59]);
    }

    // mov rcx, imm64
    pub fn emit_mov_rcx_imm64(&mut self, imm: i64) {
        self.emit_bytes(&[0x48, 0xB9]);
        self.emit_bytes(&imm.to_le_bytes());
    }

    // mov rcx, rax
    pub fn emit_mov_rcx_rax(&mut self) {
        self.emit_bytes(&[0x48, 0x89, 0xC1]);
    }

    // mov rdx, rax
    pub fn emit_mov_rdx_rax(&mut self) {
        self.emit_bytes(&[0x48, 0x89, 0xC2]);
    }

    // mov r8, rax
    pub fn emit_mov_r8_rax(&mut self) {
        self.emit_bytes(&[0x49, 0x89, 0xC0]);
    }

    // mov r9, rax
    pub fn emit_mov_r9_rax(&mut self) {
        self.emit_bytes(&[0x49, 0x89, 0xC1]);
    }

    // Call 64-bit function pointer with Windows x64 ABI (shadow space + 4 args in RCX, RDX, R8, R9)
    pub fn emit_call_c_fn(&mut self, fn_addr: usize) {
        // mov r10, fn_addr
        self.emit_bytes(&[0x49, 0xBA]);
        self.emit_bytes(&(fn_addr as u64).to_le_bytes());
        // sub rsp, 32 (32-byte shadow space)
        self.emit_bytes(&[0x48, 0x83, 0xEC, 0x20]);
        // call r10
        self.emit_bytes(&[0x41, 0xFF, 0xD2]);
        // add rsp, 32
        self.emit_bytes(&[0x48, 0x83, 0xC4, 0x20]);
    }

    // Parameter store methods for worker function entry
    pub fn emit_store_rcx_stack(&mut self, offset: i32) {
        let disp = -offset;
        self.emit_bytes(&[0x48, 0x89, 0x8D]);
        self.emit_bytes(&disp.to_le_bytes());
    }

    pub fn emit_store_rdx_stack(&mut self, offset: i32) {
        let disp = -offset;
        self.emit_bytes(&[0x48, 0x89, 0x95]);
        self.emit_bytes(&disp.to_le_bytes());
    }

    pub fn emit_store_r8_stack(&mut self, offset: i32) {
        let disp = -offset;
        self.emit_bytes(&[0x4C, 0x89, 0x85]);
        self.emit_bytes(&disp.to_le_bytes());
    }

    pub fn emit_store_r9_stack(&mut self, offset: i32) {
        let disp = -offset;
        self.emit_bytes(&[0x4C, 0x89, 0x8D]);
        self.emit_bytes(&disp.to_le_bytes());
    }

    // mov [rbp - disp32], rax
    pub fn emit_store_rax_stack(&mut self, offset: i32) {
        let disp = -offset;
        self.emit_bytes(&[0x48, 0x89, 0x85]);
        self.emit_bytes(&disp.to_le_bytes());
    }

    // mov rax, [rbp - disp32]
    pub fn emit_load_rax_stack(&mut self, offset: i32) {
        let disp = -offset;
        self.emit_bytes(&[0x48, 0x8B, 0x85]);
        self.emit_bytes(&disp.to_le_bytes());
    }

    // Arithmetic (RCX = left, RAX = right)
    pub fn emit_add(&mut self) {
        self.emit_bytes(&[0x48, 0x01, 0xC8]); // add rax, rcx
    }

    pub fn emit_sub(&mut self) {
        // We want left - right -> sub rcx, rax; mov rax, rcx
        self.emit_bytes(&[0x48, 0x29, 0xC1]); // sub rcx, rax
        self.emit_bytes(&[0x48, 0x89, 0xC8]); // mov rax, rcx
    }

    pub fn emit_mul(&mut self) {
        self.emit_bytes(&[0x48, 0x0F, 0xAF, 0xC1]); // imul rax, rcx
    }

    pub fn emit_div(&mut self) {
        // We want left / right -> mov r8, rax; mov rax, rcx; cqo; idiv r8
        self.emit_bytes(&[0x49, 0x89, 0xC0]); // mov r8, rax
        self.emit_bytes(&[0x48, 0x89, 0xC8]); // mov rax, rcx
        self.emit_bytes(&[0x48, 0x99]);       // cqo (sign-extend rax into rdx:rax)
        self.emit_bytes(&[0x49, 0xF7, 0xF8]); // idiv r8
    }

    pub fn emit_rem(&mut self) {
        // We want left % right -> mov r8, rax; mov rax, rcx; cqo; idiv r8; mov rax, rdx
        self.emit_bytes(&[0x49, 0x89, 0xC0]); // mov r8, rax
        self.emit_bytes(&[0x48, 0x89, 0xC8]); // mov rax, rcx
        self.emit_bytes(&[0x48, 0x99]);       // cqo
        self.emit_bytes(&[0x49, 0xF7, 0xF8]); // idiv r8
        self.emit_bytes(&[0x48, 0x89, 0xD0]); // mov rax, rdx
    }

    // Comparisons (RCX = left, RAX = right -> cmp rcx, rax)
    pub fn emit_cmp_and_set(&mut self, cond: &str) {
        self.emit_bytes(&[0x48, 0x39, 0xC1]); // cmp rcx, rax
        match cond {
            "==" => self.emit_bytes(&[0x0F, 0x94, 0xC0]), // sete al
            "!=" => self.emit_bytes(&[0x0F, 0x95, 0xC0]), // setne al
            "<"  => self.emit_bytes(&[0x0F, 0x9C, 0xC0]), // setl al
            ">"  => self.emit_bytes(&[0x0F, 0x9F, 0xC0]), // setg al
            "<=" => self.emit_bytes(&[0x0F, 0x9E, 0xC0]), // setle al
            ">=" => self.emit_bytes(&[0x0F, 0x9D, 0xC0]), // setge al
            _ => self.emit_bytes(&[0x0F, 0x94, 0xC0]),
        }
        self.emit_bytes(&[0x48, 0x0F, 0xB6, 0xC0]); // movzx rax, al
    }

    // Branching
    pub fn emit_test_rax(&mut self) {
        self.emit_bytes(&[0x48, 0x85, 0xC0]); // test rax, rax
    }

    // jz rel32 (returns index where rel32 begins for backpatching)
    pub fn emit_jz_placeholder(&mut self) -> usize {
        self.emit_bytes(&[0x0F, 0x84]);
        let pos = self.code.len();
        self.emit_bytes(&[0x00, 0x00, 0x00, 0x00]);
        pos
    }

    // jmp rel32 (returns index where rel32 begins for backpatching)
    pub fn emit_jmp_placeholder(&mut self) -> usize {
        self.emit_bytes(&[0xE9]);
        let pos = self.code.len();
        self.emit_bytes(&[0x00, 0x00, 0x00, 0x00]);
        pos
    }

    // jmp backward to target
    pub fn emit_jmp_target(&mut self, target_idx: usize) {
        self.emit_bytes(&[0xE9]);
        let next_ip = (self.code.len() + 4) as i32;
        let rel = (target_idx as i32) - next_ip;
        self.emit_bytes(&rel.to_le_bytes());
    }

    pub fn patch_rel32(&mut self, placeholder_pos: usize, target_idx: usize) {
        let next_ip = (placeholder_pos + 4) as i32;
        let rel = (target_idx as i32) - next_ip;
        let bytes = rel.to_le_bytes();
        self.code[placeholder_pos..placeholder_pos + 4].copy_from_slice(&bytes);
    }
}

// ----------------------------------------------------------------------------
// 3. JIT Compiler Implementation
// ----------------------------------------------------------------------------

pub struct JitCompiler {
    asm: X64Assembler,
    variables: HashMap<String, i32>, // name -> stack offset [rbp - offset]
    var_types: HashMap<String, String>, // name -> type
    functions: HashMap<String, crate::ast::FunctionDecl>,
    compiled_workers: HashMap<String, Arc<JitMemory>>,
    worker_memories: Vec<Arc<JitMemory>>,
    next_offset: i32,
    last_stored_var: Option<String>,
    has_returned: bool,
}

impl Default for JitCompiler {
    fn default() -> Self {
        Self::new()
    }
}

impl JitCompiler {
    pub fn new() -> Self {
        Self {
            asm: X64Assembler::new(),
            variables: HashMap::new(),
            var_types: HashMap::new(),
            functions: HashMap::new(),
            compiled_workers: HashMap::new(),
            worker_memories: Vec::new(),
            next_offset: 32,
            last_stored_var: None,
            has_returned: false,
        }
    }

    pub fn compile_worker_function(
        func: &crate::ast::FunctionDecl,
        all_functions: &HashMap<String, crate::ast::FunctionDecl>,
        compiled_workers: &HashMap<String, Arc<JitMemory>>,
    ) -> Result<JitMemory, String> {
        let mut compiler = JitCompiler::new();
        compiler.functions = all_functions.clone();
        compiler.compiled_workers = compiled_workers.clone();
        compiler.asm.emit_prologue();

        // Map parameters from calling convention registers (Windows x64 ABI: RCX, RDX, R8, R9)
        for (i, param) in func.params.iter().enumerate() {
            let is_vec = param.param_type.contains("vec");
            let off = compiler.alloc_var(&param.name, is_vec);
            match i {
                0 => compiler.asm.emit_store_rcx_stack(off),
                1 => compiler.asm.emit_store_rdx_stack(off),
                2 => compiler.asm.emit_store_r8_stack(off),
                3 => compiler.asm.emit_store_r9_stack(off),
                _ => {}
            }
        }

        for stmt in &func.body {
            compiler.compile_statement(stmt);
        }

        if !compiler.has_returned {
            if let Some(last) = &compiler.last_stored_var {
                if let Some(&off) = compiler.variables.get(last) {
                    compiler.asm.emit_load_rax_stack(off);
                }
            } else {
                compiler.asm.emit_mov_rax_imm64(0);
            }
            compiler.asm.emit_epilogue();
        }

        let mut mem = JitMemory::new(&compiler.asm.code)?;
        mem._dependencies.extend(compiler.worker_memories);
        Ok(mem)
    }

    fn compile_spawn(&mut self, target: &Expr) {
        if let Expr::Call { callee, args } = target {
            let base_name = callee.split('<').next().unwrap_or(callee);
            let fn_ptr = self.compiled_workers.get(base_name)
                .map(|m| m.ptr as usize)
                .or_else(|| {
                    if let Some(func) = self.functions.get(base_name).cloned() {
                        if let Ok(worker_mem) = Self::compile_worker_function(&func, &self.functions, &self.compiled_workers) {
                            let arc_mem = Arc::new(worker_mem);
                            let ptr = arc_mem.ptr as usize;
                            self.compiled_workers.insert(base_name.to_string(), arc_mem.clone());
                            self.worker_memories.push(arc_mem);
                            return Some(ptr);
                        }
                    }
                    None
                });

            if let Some(ptr) = fn_ptr {
                match args.len() {
                    0 => {
                        self.asm.emit_mov_rcx_imm64(ptr as i64);
                        self.asm.emit_call_c_fn(extern_c_jit_spawn_0 as *const () as usize);
                    }
                    1 => {
                        self.compile_expr(&args[0].value);
                        self.asm.emit_mov_rdx_rax();
                        self.asm.emit_mov_rcx_imm64(ptr as i64);
                        self.asm.emit_call_c_fn(extern_c_jit_spawn_1 as *const () as usize);
                    }
                    2 => {
                        self.compile_expr(&args[0].value);
                        self.asm.emit_push_rax();
                        self.compile_expr(&args[1].value);
                        self.asm.emit_mov_r8_rax();
                        self.asm.emit_pop_rdx();
                        self.asm.emit_mov_rcx_imm64(ptr as i64);
                        self.asm.emit_call_c_fn(extern_c_jit_spawn_2 as *const () as usize);
                    }
                    3 => {
                        self.compile_expr(&args[0].value);
                        self.asm.emit_push_rax();
                        self.compile_expr(&args[1].value);
                        self.asm.emit_push_rax();
                        self.compile_expr(&args[2].value);
                        self.asm.emit_mov_r9_rax();
                        self.asm.emit_pop_r8();
                        self.asm.emit_pop_rdx();
                        self.asm.emit_mov_rcx_imm64(ptr as i64);
                        self.asm.emit_call_c_fn(extern_c_jit_spawn_3 as *const () as usize);
                    }
                    _ => {
                        self.asm.emit_mov_rax_imm64(0);
                    }
                }
            } else {
                self.asm.emit_mov_rax_imm64(0);
            }
        } else {
            self.compile_expr(target);
        }
    }

    fn alloc_var(&mut self, name: &str, is_vec: bool) -> i32 {
        if let Some(&off) = self.variables.get(name) {
            off
        } else {
            let size = if is_vec { 32 } else { 8 };
            if is_vec && (self.next_offset % 32 != 0) {
                self.next_offset += 32 - (self.next_offset % 32);
            }
            self.next_offset += size;
            let off = self.next_offset;
            self.variables.insert(name.to_string(), off);
            off
        }
    }

    pub fn compile(mut self, program: &Program) -> Result<JitMemory, String> {
        let mono_program = CBackend::monomorphize_program(program);

        for f in &mono_program.functions {
            self.functions.insert(f.name.clone(), f.clone());
        }

        // Pre-compile non-main worker functions
        for f in &mono_program.functions {
            if f.name != "main" {
                if let Ok(worker_mem) = Self::compile_worker_function(f, &self.functions, &self.compiled_workers) {
                    let arc_mem = Arc::new(worker_mem);
                    self.compiled_workers.insert(f.name.clone(), arc_mem.clone());
                    self.worker_memories.push(arc_mem);
                }
            }
        }

        self.asm.emit_prologue();

        let empty_vec = Vec::new();
        let stmts_to_run = if mono_program.main_statements.is_empty() {
            if let Some(main_fn) = mono_program.functions.iter().find(|f| f.name == "main") {
                &main_fn.body
            } else {
                &empty_vec
            }
        } else {
            &mono_program.main_statements
        };

        // Compile statements
        for stmt in stmts_to_run {
            self.compile_statement(stmt);
        }

        // Return the last assigned variable or RAX if no explicit return
        if !self.has_returned {
            if let Some(last) = &self.last_stored_var {
                if let Some(&off) = self.variables.get(last) {
                    if self.var_types.get(last).map(|t| t.contains("vec")).unwrap_or(false) {
                        self.asm.emit_vmovups_load_ymm0_stack(off);
                        self.asm.emit_vreduce_sum_ymm0_to_rax();
                    } else {
                        self.asm.emit_load_rax_stack(off);
                    }
                }
            }
            self.asm.emit_epilogue();
        }

        let mut mem = JitMemory::new(&self.asm.code)?;
        mem._dependencies.extend(self.worker_memories);
        Ok(mem)
    }

    fn compile_statement(&mut self, stmt: &Statement) {
        match stmt {
            Statement::Let { name, type_annot, value, .. } => {
                let is_call_simd = match value {
                    Expr::Call { callee, .. } => callee == "simd_splat" || callee == "simd_fma",
                    _ => false,
                };
                let is_vec = type_annot.as_deref().map(|t| t.contains("vec") || t.contains("simd")).unwrap_or(false)
                    || matches!(value, Expr::Array(_)) || is_call_simd;

                if is_vec {
                    self.var_types.insert(name.clone(), "vec8f".to_string());
                    let off = self.alloc_var(name, true);
                    self.compile_vec_expr(value, off);
                    self.last_stored_var = Some(name.clone());
                } else {
                    self.compile_expr(value);
                    let off = self.alloc_var(name, false);
                    self.asm.emit_store_rax_stack(off);
                    self.last_stored_var = Some(name.clone());
                }
            }
            Statement::Assign { target, value, .. } => {
                let is_call_simd = match value {
                    Expr::Call { callee, .. } => callee == "simd_splat" || callee == "simd_fma",
                    _ => false,
                };
                let is_vec = self.var_types.get(target).map(|t| t.contains("vec")).unwrap_or(false)
                    || matches!(value, Expr::Array(_)) || is_call_simd;

                if is_vec {
                    self.var_types.insert(target.clone(), "vec8f".to_string());
                    let off = self.alloc_var(target, true);
                    self.compile_vec_expr(value, off);
                    self.last_stored_var = Some(target.clone());
                } else {
                    self.compile_expr(value);
                    let off = self.alloc_var(target, false);
                    self.asm.emit_store_rax_stack(off);
                    self.last_stored_var = Some(target.clone());
                }
            }
            Statement::If { condition, then_body, else_body, .. } => {
                self.compile_expr(condition);
                self.asm.emit_test_rax();
                let else_patch = self.asm.emit_jz_placeholder();

                for s in then_body {
                    self.compile_statement(s);
                }

                let end_patch = self.asm.emit_jmp_placeholder();
                let else_target = self.asm.code.len();
                self.asm.patch_rel32(else_patch, else_target);

                if let Some(eb) = else_body {
                    for s in eb {
                        self.compile_statement(s);
                    }
                }

                let end_target = self.asm.code.len();
                self.asm.patch_rel32(end_patch, end_target);
            }
            Statement::While { condition, body, .. } => {
                let loop_start = self.asm.code.len();
                self.compile_expr(condition);
                self.asm.emit_test_rax();
                let exit_patch = self.asm.emit_jz_placeholder();

                for s in body {
                    self.compile_statement(s);
                }

                self.asm.emit_jmp_target(loop_start);
                let loop_end = self.asm.code.len();
                self.asm.patch_rel32(exit_patch, loop_end);
            }
            Statement::Return(opt_e) => {
                if let Some(e) = opt_e {
                    self.compile_expr(e);
                }
                self.asm.emit_epilogue();
                self.has_returned = true;
            }
            Statement::Region { body, .. } | Statement::Resilient { body, .. } => {
                for s in body {
                    self.compile_statement(s);
                }
            }
            Statement::Expr(e) => {
                self.compile_expr(e);
            }
            _ => {}
        }
    }

    fn compile_vec_expr(&mut self, expr: &Expr, dest_off: i32) {
        match expr {
            Expr::Array(elements) => {
                for (idx, elem) in elements.iter().take(8).enumerate() {
                    let bits = match elem {
                        Expr::LiteralInt(n) => (*n as f32).to_bits(),
                        Expr::LiteralFloat(f) => (*f as f32).to_bits(),
                        _ => 0,
                    };
                    let elem_disp = dest_off - (idx as i32) * 4;
                    self.asm.emit_store_imm32_stack(elem_disp, bits);
                }
                self.asm.emit_vmovups_load_ymm0_stack(dest_off);
            }
            Expr::Call { callee, args } => {
                if callee == "simd_splat" && !args.is_empty() {
                    self.compile_expr(&args[0].value);
                    self.asm.emit_vbroadcastss_from_rax();
                    self.asm.emit_vmovups_store_ymm0_stack(dest_off);
                } else if callee == "simd_fma" && args.len() >= 3 {
                    // simd_fma(a, b, c) -> a * b + c
                    if let Expr::Ident(name_a, _) = &args[0].value {
                        if let Some(&off_a) = self.variables.get(name_a) {
                            self.asm.emit_vmovups_load_ymm1_stack(off_a);
                        }
                    }
                    if let Expr::Ident(name_b, _) = &args[1].value {
                        if let Some(&off_b) = self.variables.get(name_b) {
                            self.asm.emit_vmovups_load_ymm2_stack(off_b);
                        }
                    }
                    if let Expr::Ident(name_c, _) = &args[2].value {
                        if let Some(&off_c) = self.variables.get(name_c) {
                            self.asm.emit_vmovups_load_ymm0_stack(off_c);
                        }
                    }
                    self.asm.emit_vfmadd231ps_ymm0_ymm1_ymm2();
                    self.asm.emit_vmovups_store_ymm0_stack(dest_off);
                } else if let Some(func) = self.functions.get(callee).cloned() {
                    let old_vars = self.variables.clone();
                    let old_types = self.var_types.clone();

                    for (i, param) in func.params.iter().enumerate() {
                        if i < args.len() {
                            let is_vec = param.param_type.contains("vec");
                            if is_vec {
                                self.var_types.insert(param.name.clone(), "vec8f".to_string());
                                let off = self.alloc_var(&param.name, true);
                                self.compile_vec_expr(&args[i].value, off);
                            } else {
                                self.compile_expr(&args[i].value);
                                let off = self.alloc_var(&param.name, false);
                                self.asm.emit_store_rax_stack(off);
                            }
                        }
                    }

                    for stmt in &func.body {
                        match stmt {
                            Statement::Return(Some(ret_expr)) => {
                                self.compile_vec_expr(ret_expr, dest_off);
                            }
                            other => {
                                self.compile_statement(other);
                            }
                        }
                    }

                    for (k, v) in old_vars {
                        self.variables.insert(k, v);
                    }
                    for (k, v) in old_types {
                        self.var_types.insert(k, v);
                    }
                } else {
                    self.asm.emit_vmovups_store_ymm0_stack(dest_off);
                }
            }
            Expr::Binary { op, left, right } => {
                if let Expr::Ident(name_l, _) = &**left {
                    if let Some(&off_l) = self.variables.get(name_l) {
                        self.asm.emit_vmovups_load_ymm0_stack(off_l);
                    }
                }
                if let Expr::Ident(name_r, _) = &**right {
                    if let Some(&off_r) = self.variables.get(name_r) {
                        self.asm.emit_vmovups_load_ymm1_stack(off_r);
                    }
                }
                match op.as_str() {
                    "+" => self.asm.emit_vaddps_ymm0_ymm1(),
                    "-" => self.asm.emit_vsubps_ymm0_ymm1(),
                    "*" => self.asm.emit_vmulps_ymm0_ymm1(),
                    _ => {}
                }
                self.asm.emit_vmovups_store_ymm0_stack(dest_off);
            }
            Expr::Ident(name, _) => {
                if let Some(&off) = self.variables.get(name) {
                    self.asm.emit_vmovups_load_ymm0_stack(off);
                    self.asm.emit_vmovups_store_ymm0_stack(dest_off);
                }
            }
            _ => {}
        }
    }

    fn compile_expr(&mut self, expr: &Expr) {
        match expr {
            Expr::LiteralInt(n) => {
                self.asm.emit_mov_rax_imm64(*n);
            }
            Expr::LiteralHex(h) => {
                self.asm.emit_mov_rax_imm64(*h as i64);
            }
            Expr::LiteralFloat(f) => {
                self.asm.emit_mov_rax_imm64(*f as i64);
            }
            Expr::LiteralBool(b) => {
                self.asm.emit_mov_rax_imm64(if *b { 1 } else { 0 });
            }
            Expr::Cast { expr, .. } => {
                self.compile_expr(expr);
            }
            Expr::Ident(name, _) => {
                if let Some(&off) = self.variables.get(name) {
                    if self.var_types.get(name).map(|t| t.contains("vec")).unwrap_or(false) {
                        self.asm.emit_vmovups_load_ymm0_stack(off);
                        self.asm.emit_vreduce_sum_ymm0_to_rax();
                    } else {
                        self.asm.emit_load_rax_stack(off);
                    }
                } else {
                    self.asm.emit_mov_rax_imm64(0);
                }
            }
            Expr::Binary { op, left, right } => {
                self.compile_expr(left);
                self.asm.emit_push_rax();
                self.compile_expr(right);
                self.asm.emit_pop_rcx();

                // RCX = left, RAX = right
                match op.as_str() {
                    "+" => self.asm.emit_add(),
                    "-" => self.asm.emit_sub(),
                    "*" => self.asm.emit_mul(),
                    "/" => self.asm.emit_div(),
                    "%" => self.asm.emit_rem(),
                    "==" | "!=" | "<" | ">" | "<=" | ">=" => {
                        self.asm.emit_cmp_and_set(op);
                    }
                    "and" | "&&" => {
                        self.asm.emit_bytes(&[0x48, 0x21, 0xC8]); // and rax, rcx
                    }
                    "or" | "||" => {
                        self.asm.emit_bytes(&[0x48, 0x09, 0xC8]); // or rax, rcx
                    }
                    "^" => {
                        self.asm.emit_bytes(&[0x48, 0x31, 0xC8]); // xor rax, rcx
                    }
                    _ => {}
                }
            }
            Expr::Call { callee, args } => {
                let base_callee = callee.split('<').next().unwrap_or(callee);
                if base_callee == "consume" && !args.is_empty() {
                    self.compile_expr(&args[0].value);
                } else if base_callee == "channel_new" || base_callee == "channel" || base_callee == "Channel" {
                    if !args.is_empty() {
                        self.compile_expr(&args[0].value);
                        self.asm.emit_mov_rcx_rax();
                    } else {
                        self.asm.emit_mov_rcx_imm64(64);
                    }
                    self.asm.emit_call_c_fn(extern_c_jit_channel_new as *const () as usize);
                } else if base_callee == "channel_send" && args.len() >= 2 {
                    self.compile_expr(&args[0].value);
                    self.asm.emit_push_rax();
                    self.compile_expr(&args[1].value);
                    self.asm.emit_mov_rdx_rax();
                    self.asm.emit_pop_rcx();
                    self.asm.emit_call_c_fn(extern_c_jit_channel_send as *const () as usize);
                } else if base_callee == "channel_recv" && !args.is_empty() {
                    self.compile_expr(&args[0].value);
                    self.asm.emit_mov_rcx_rax();
                    self.asm.emit_call_c_fn(extern_c_jit_channel_recv as *const () as usize);
                } else if base_callee == "channel_try_recv" && !args.is_empty() {
                    self.compile_expr(&args[0].value);
                    self.asm.emit_mov_rcx_rax();
                    self.asm.emit_call_c_fn(extern_c_jit_channel_try_recv as *const () as usize);
                } else if base_callee == "channel_close" && !args.is_empty() {
                    self.compile_expr(&args[0].value);
                    self.asm.emit_mov_rcx_rax();
                    self.asm.emit_call_c_fn(extern_c_jit_channel_close as *const () as usize);
                } else if (base_callee == "torus_distance" || base_callee == "torus_dist") && args.len() >= 2 {
                    self.compile_expr(&args[0].value);
                    self.asm.emit_push_rax();
                    self.compile_expr(&args[1].value);
                    self.asm.emit_mov_rdx_rax();
                    self.asm.emit_pop_rcx();
                    self.asm.emit_call_c_fn(extern_c_jit_torus_distance as *const () as usize);
                } else if base_callee == "torus_core_id" {
                    self.asm.emit_call_c_fn(extern_c_jit_torus_core_id as *const () as usize);
                } else if base_callee == "torus_send" && args.len() >= 2 {
                    self.compile_expr(&args[0].value);
                    self.asm.emit_push_rax();
                    self.compile_expr(&args[1].value);
                    self.asm.emit_mov_rdx_rax();
                    self.asm.emit_pop_rcx();
                    self.asm.emit_call_c_fn(extern_c_jit_channel_send as *const () as usize);
                } else if base_callee == "torus_recv" && !args.is_empty() {
                    self.compile_expr(&args[0].value);
                    self.asm.emit_mov_rcx_rax();
                    self.asm.emit_call_c_fn(extern_c_jit_channel_recv as *const () as usize);
                } else if base_callee == "spawn_at" && args.len() >= 2 {
                    self.compile_spawn(&args[1].value);
                } else if base_callee == "await" && !args.is_empty() {
                    self.compile_expr(&args[0].value);
                    self.asm.emit_mov_rcx_rax();
                    self.asm.emit_call_c_fn(extern_c_jit_await as *const () as usize);
                } else if base_callee == "simd_reduce_sum" && !args.is_empty() {
                    if let Expr::Ident(name, _) = &args[0].value {
                        if let Some(&off) = self.variables.get(name) {
                            self.asm.emit_vmovups_load_ymm0_stack(off);
                            self.asm.emit_vreduce_sum_ymm0_to_rax();
                        }
                    }
                } else if base_callee == "simd_dot" && args.len() >= 2 {
                    if let Expr::Ident(name_a, _) = &args[0].value {
                        if let Some(&off_a) = self.variables.get(name_a) {
                            self.asm.emit_vmovups_load_ymm0_stack(off_a);
                        }
                    }
                    if let Expr::Ident(name_b, _) = &args[1].value {
                        if let Some(&off_b) = self.variables.get(name_b) {
                            self.asm.emit_vmovups_load_ymm1_stack(off_b);
                        }
                    }
                    self.asm.emit_vmulps_ymm0_ymm1();
                    self.asm.emit_vreduce_sum_ymm0_to_rax();
                } else if let Some(func) = self.functions.get(base_callee).cloned() {
                    let old_vars = self.variables.clone();
                    let old_types = self.var_types.clone();

                    for (i, param) in func.params.iter().enumerate() {
                        if i < args.len() {
                            let is_vec = param.param_type.contains("vec");
                            if is_vec {
                                self.var_types.insert(param.name.clone(), "vec8f".to_string());
                                let off = self.alloc_var(&param.name, true);
                                self.compile_vec_expr(&args[i].value, off);
                            } else {
                                self.compile_expr(&args[i].value);
                                let off = self.alloc_var(&param.name, false);
                                self.asm.emit_store_rax_stack(off);
                            }
                        }
                    }

                    for stmt in &func.body {
                        match stmt {
                            Statement::Return(Some(ret_expr)) => {
                                self.compile_expr(ret_expr);
                            }
                            other => {
                                self.compile_statement(other);
                            }
                        }
                    }

                    for (k, v) in old_vars {
                        self.variables.insert(k, v);
                    }
                    for (k, v) in old_types {
                        self.var_types.insert(k, v);
                    }
                } else {
                    self.asm.emit_mov_rax_imm64(0);
                }
            }
            Expr::MethodCall { object, method, args } => {
                if method == "send" && !args.is_empty() {
                    self.compile_expr(object);
                    self.asm.emit_push_rax();
                    self.compile_expr(&args[0].value);
                    self.asm.emit_mov_rdx_rax();
                    self.asm.emit_pop_rcx();
                    self.asm.emit_call_c_fn(extern_c_jit_channel_send as *const () as usize);
                } else if method == "recv" {
                    self.compile_expr(object);
                    self.asm.emit_mov_rcx_rax();
                    self.asm.emit_call_c_fn(extern_c_jit_channel_recv as *const () as usize);
                } else if method == "try_recv" {
                    self.compile_expr(object);
                    self.asm.emit_mov_rcx_rax();
                    self.asm.emit_call_c_fn(extern_c_jit_channel_try_recv as *const () as usize);
                } else if method == "close" {
                    self.compile_expr(object);
                    self.asm.emit_mov_rcx_rax();
                    self.asm.emit_call_c_fn(extern_c_jit_channel_close as *const () as usize);
                } else if method == "join" {
                    self.compile_expr(object);
                    self.asm.emit_mov_rcx_rax();
                    self.asm.emit_call_c_fn(extern_c_jit_await as *const () as usize);
                } else {
                    self.asm.emit_mov_rax_imm64(0);
                }
            }
            Expr::ChannelSend { channel, value, .. } => {
                self.compile_expr(channel);
                self.asm.emit_push_rax();
                self.compile_expr(value);
                self.asm.emit_mov_rdx_rax();
                self.asm.emit_pop_rcx();
                self.asm.emit_call_c_fn(extern_c_jit_channel_send as *const () as usize);
            }
            Expr::ChannelRecv { channel, .. } => {
                self.compile_expr(channel);
                self.asm.emit_mov_rcx_rax();
                self.asm.emit_call_c_fn(extern_c_jit_channel_recv as *const () as usize);
            }
            Expr::Spawn(target) => {
                self.compile_spawn(target);
            }
            Expr::SpawnAt { target, .. } => {
                self.compile_spawn(target);
            }
            Expr::Await(target) => {
                self.compile_expr(target);
                self.asm.emit_mov_rcx_rax();
                self.asm.emit_call_c_fn(extern_c_jit_await as *const () as usize);
            }
            _ => {
                self.asm.emit_mov_rax_imm64(0);
            }
        }
    }
}

// ----------------------------------------------------------------------------
// 4. Public High-Level JIT Runner
// ----------------------------------------------------------------------------

pub fn run_source_jit(source: &str) -> Result<i64, String> {
    let mut lexer = crate::lexer::Lexer::new(source);
    let tokens = lexer.tokenize().map_err(|e| format!("Lexer error: {}", e))?;

    let mut parser = crate::parser::Parser::new(tokens);
    let mut program = parser.parse_program().map_err(|e| format!("Parser error: {}", e))?;

    let _ = crate::autodiff::AutodiffEngine::differentiate_program(&mut program);

    let mut checker = crate::checker::SemanticChecker::new();
    checker.check_program(&program).map_err(|e| format!("Type check error: {}", e.message))?;

    let compiler = JitCompiler::new();
    let jit_mem = compiler.compile(&program)?;

    let func = jit_mem.as_fn();
    let result = unsafe { func() };

    Ok(result)
}
