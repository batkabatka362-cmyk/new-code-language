// ============================================================================
// CRON Zero-Dependency Native NVIDIA CUDA Driver JIT Engine (cl_cuda_driver.rs)
//
// 100% Pure Rust - Zero CUDA Toolkit requirement, Zero NVCC requirement.
//
// Dynamically binds directly to the OS-installed NVIDIA display driver library:
//   - Windows: nvcuda.dll (shipped in C:\Windows\System32 with all NVIDIA drivers)
//   - Linux:   libcuda.so.1 / libcuda.so
//
// Capabilities:
//   1. Driver Initialization & Hardware Discovery (cuInit, cuDeviceGet, cuDeviceGetName)
//   2. High-Performance GPU Context Management (cuCtxCreate_v2, cuCtxDestroy_v2)
//   3. Direct PTX Assembly JIT Compilation & Kernel Discovery (cuModuleLoadData, cuModuleGetFunction)
//   4. Zero-Copy Host-to-Device & Device-to-Host Unified Transfers (cuMemAlloc, cuMemcpyHtoD, cuMemcpyDtoH)
//   5. Asynchronous Grid/Block Kernel Launch (cuLaunchKernel, cuCtxSynchronize)
// ============================================================================

use std::ffi::{c_void, CStr, CString};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

// CUDA Driver API Return Code
pub type CUresult = u32;
pub const CUDA_SUCCESS: CUresult = 0;

// CUDA Driver Handle Types
pub type CUdevice = i32;
pub type CUcontext = *mut c_void;
pub type CUmodule = *mut c_void;
pub type CUfunction = *mut c_void;
pub type CUdeviceptr = u64;
pub type CUstream = *mut c_void;

// Function Pointer Signatures
type FnCuInit = unsafe extern "C" fn(u32) -> CUresult;
type FnCuDriverGetVersion = unsafe extern "C" fn(*mut i32) -> CUresult;
type FnCuDeviceGetCount = unsafe extern "C" fn(*mut i32) -> CUresult;
type FnCuDeviceGet = unsafe extern "C" fn(*mut CUdevice, i32) -> CUresult;
type FnCuDeviceGetName = unsafe extern "C" fn(*mut u8, i32, CUdevice) -> CUresult;
type FnCuDeviceTotalMem = unsafe extern "C" fn(*mut usize, CUdevice) -> CUresult;
type FnCuCtxCreate = unsafe extern "C" fn(*mut CUcontext, u32, CUdevice) -> CUresult;
type FnCuCtxDestroy = unsafe extern "C" fn(CUcontext) -> CUresult;
type FnCuCtxSetCurrent = unsafe extern "C" fn(CUcontext) -> CUresult;
type FnCuCtxSynchronize = unsafe extern "C" fn() -> CUresult;
type FnCuModuleLoadData = unsafe extern "C" fn(*mut CUmodule, *const c_void) -> CUresult;
type FnCuModuleGetFunction = unsafe extern "C" fn(*mut CUfunction, CUmodule, *const u8) -> CUresult;
type FnCuModuleUnload = unsafe extern "C" fn(CUmodule) -> CUresult;
type FnCuMemAlloc = unsafe extern "C" fn(*mut CUdeviceptr, usize) -> CUresult;
type FnCuMemFree = unsafe extern "C" fn(CUdeviceptr) -> CUresult;
type FnCuMemcpyHtoD = unsafe extern "C" fn(CUdeviceptr, *const c_void, usize) -> CUresult;
type FnCuMemcpyDtoH = unsafe extern "C" fn(*mut c_void, CUdeviceptr, usize) -> CUresult;
type FnCuLaunchKernel = unsafe extern "C" fn(
    CUfunction,
    u32, u32, u32,
    u32, u32, u32,
    u32,
    CUstream,
    *mut *mut c_void,
    *mut *mut c_void,
) -> CUresult;
type FnCuGetErrorString = unsafe extern "C" fn(CUresult, *mut *const u8) -> CUresult;

#[cfg(windows)]
mod os_dynload {
    use std::ffi::c_void;

    extern "system" {
        pub fn LoadLibraryA(lpLibFileName: *const u8) -> *mut c_void;
        pub fn GetProcAddress(hModule: *mut c_void, lpProcName: *const u8) -> *mut c_void;
        pub fn FreeLibrary(hLibModule: *mut c_void) -> i32;
    }

    pub unsafe fn load_library(name: &str) -> Option<*mut c_void> {
        let mut null_terminated = name.as_bytes().to_vec();
        null_terminated.push(0);
        let handle = LoadLibraryA(null_terminated.as_ptr());
        if handle.is_null() {
            None
        } else {
            Some(handle)
        }
    }

    pub unsafe fn get_symbol(handle: *mut c_void, symbol: &str) -> Option<*mut c_void> {
        let mut null_terminated = symbol.as_bytes().to_vec();
        null_terminated.push(0);
        let ptr = GetProcAddress(handle, null_terminated.as_ptr());
        if ptr.is_null() {
            None
        } else {
            Some(ptr)
        }
    }

    pub unsafe fn free_library(handle: *mut c_void) {
        if !handle.is_null() {
            FreeLibrary(handle);
        }
    }
}

#[cfg(unix)]
mod os_dynload {
    use std::ffi::c_void;
    use std::os::raw::{c_char, c_int};

    const RTLD_NOW: c_int = 2;

    extern "C" {
        pub fn dlopen(filename: *const c_char, flag: c_int) -> *mut c_void;
        pub fn dlsym(handle: *mut c_void, symbol: *const c_char) -> *mut c_void;
        pub fn dlclose(handle: *mut c_void) -> c_int;
    }

    pub unsafe fn load_library(name: &str) -> Option<*mut c_void> {
        let mut null_terminated = name.as_bytes().to_vec();
        null_terminated.push(0);
        let handle = dlopen(null_terminated.as_ptr() as *const c_char, RTLD_NOW);
        if handle.is_null() {
            None
        } else {
            Some(handle)
        }
    }

    pub unsafe fn get_symbol(handle: *mut c_void, symbol: &str) -> Option<*mut c_void> {
        let mut null_terminated = symbol.as_bytes().to_vec();
        null_terminated.push(0);
        let ptr = dlsym(handle, null_terminated.as_ptr() as *const c_char);
        if ptr.is_null() {
            None
        } else {
            Some(ptr)
        }
    }

    pub unsafe fn free_library(handle: *mut c_void) {
        if !handle.is_null() {
            dlclose(handle);
        }
    }
}

/// Dynamic Function Table for CUDA Driver
#[derive(Clone)]
pub struct CudaDriverVTable {
    pub cu_init: FnCuInit,
    pub cu_driver_get_version: FnCuDriverGetVersion,
    pub cu_device_get_count: FnCuDeviceGetCount,
    pub cu_device_get: FnCuDeviceGet,
    pub cu_device_get_name: FnCuDeviceGetName,
    pub cu_device_total_mem: FnCuDeviceTotalMem,
    pub cu_ctx_create: FnCuCtxCreate,
    pub cu_ctx_destroy: FnCuCtxDestroy,
    pub cu_ctx_set_current: FnCuCtxSetCurrent,
    pub cu_ctx_synchronize: FnCuCtxSynchronize,
    pub cu_module_load_data: FnCuModuleLoadData,
    pub cu_module_get_function: FnCuModuleGetFunction,
    pub cu_module_unload: FnCuModuleUnload,
    pub cu_mem_alloc: FnCuMemAlloc,
    pub cu_mem_free: FnCuMemFree,
    pub cu_memcpy_htod: FnCuMemcpyHtoD,
    pub cu_memcpy_dtoh: FnCuMemcpyDtoH,
    pub cu_launch_kernel: FnCuLaunchKernel,
    pub cu_get_error_string: FnCuGetErrorString,
}

/// Global Raw Library Handle Wrapper
pub struct CudaLibraryHandle {
    raw_handle: *mut c_void,
}

unsafe impl Send for CudaLibraryHandle {}
unsafe impl Sync for CudaLibraryHandle {}

impl Drop for CudaLibraryHandle {
    fn drop(&mut self) {
        unsafe {
            os_dynload::free_library(self.raw_handle);
        }
    }
}

/// Primary CUDA Driver Interface
#[derive(Clone)]
pub struct CudaDriver {
    #[allow(dead_code)]
    lib_handle: Arc<CudaLibraryHandle>,
    pub vtable: CudaDriverVTable,
}

impl CudaDriver {
    /// Detects whether the host operating system has an active NVIDIA display driver installed
    pub fn is_available() -> bool {
        #[cfg(windows)]
        let candidates = ["nvcuda.dll"];
        #[cfg(unix)]
        let candidates = ["libcuda.so.1", "libcuda.so"];

        for candidate in &candidates {
            if let Some(h) = unsafe { os_dynload::load_library(candidate) } {
                unsafe { os_dynload::free_library(h) };
                return true;
            }
        }
        false
    }

    /// Dynamically initializes the CUDA Driver API
    pub fn init() -> Result<Self, String> {
        #[cfg(windows)]
        let candidates = ["nvcuda.dll"];
        #[cfg(unix)]
        let candidates = ["libcuda.so.1", "libcuda.so"];

        let mut loaded_handle = None;
        for candidate in &candidates {
            if let Some(h) = unsafe { os_dynload::load_library(candidate) } {
                loaded_handle = Some(h);
                break;
            }
        }

        let raw_handle = loaded_handle.ok_or_else(|| {
            "NVIDIA CUDA Driver library (nvcuda.dll / libcuda.so) could not be located in system path. Please verify NVIDIA display drivers are installed.".to_string()
        })?;

        unsafe {
            macro_rules! resolve_sym {
                ($sym:expr, $ty:ty) => {{
                    let ptr = os_dynload::get_symbol(raw_handle, $sym)
                        .ok_or_else(|| format!("Required CUDA Driver symbol '{}' not found in driver DLL", $sym))?;
                    std::mem::transmute::<*mut c_void, $ty>(ptr)
                }};
            }

            macro_rules! resolve_sym_fallback {
                ($sym_v2:expr, $sym_fallback:expr, $ty:ty) => {{
                    let ptr = match os_dynload::get_symbol(raw_handle, $sym_v2) {
                        Some(p) => p,
                        None => os_dynload::get_symbol(raw_handle, $sym_fallback)
                            .ok_or_else(|| format!("Neither '{}' nor '{}' found in driver DLL", $sym_v2, $sym_fallback))?,
                    };
                    std::mem::transmute::<*mut c_void, $ty>(ptr)
                }};
            }

            let cu_init: FnCuInit = resolve_sym!("cuInit", FnCuInit);
            let cu_driver_get_version: FnCuDriverGetVersion = resolve_sym!("cuDriverGetVersion", FnCuDriverGetVersion);
            let cu_device_get_count: FnCuDeviceGetCount = resolve_sym!("cuDeviceGetCount", FnCuDeviceGetCount);
            let cu_device_get: FnCuDeviceGet = resolve_sym!("cuDeviceGet", FnCuDeviceGet);
            let cu_device_get_name: FnCuDeviceGetName = resolve_sym!("cuDeviceGetName", FnCuDeviceGetName);
            let cu_device_total_mem: FnCuDeviceTotalMem = resolve_sym_fallback!("cuDeviceTotalMem_v2", "cuDeviceTotalMem", FnCuDeviceTotalMem);
            let cu_ctx_create: FnCuCtxCreate = resolve_sym_fallback!("cuCtxCreate_v2", "cuCtxCreate", FnCuCtxCreate);
            let cu_ctx_destroy: FnCuCtxDestroy = resolve_sym_fallback!("cuCtxDestroy_v2", "cuCtxDestroy", FnCuCtxDestroy);
            let cu_ctx_set_current: FnCuCtxSetCurrent = resolve_sym!("cuCtxSetCurrent", FnCuCtxSetCurrent);
            let cu_ctx_synchronize: FnCuCtxSynchronize = resolve_sym!("cuCtxSynchronize", FnCuCtxSynchronize);
            let cu_module_load_data: FnCuModuleLoadData = resolve_sym!("cuModuleLoadData", FnCuModuleLoadData);
            let cu_module_get_function: FnCuModuleGetFunction = resolve_sym!("cuModuleGetFunction", FnCuModuleGetFunction);
            let cu_module_unload: FnCuModuleUnload = resolve_sym!("cuModuleUnload", FnCuModuleUnload);
            let cu_mem_alloc: FnCuMemAlloc = resolve_sym_fallback!("cuMemAlloc_v2", "cuMemAlloc", FnCuMemAlloc);
            let cu_mem_free: FnCuMemFree = resolve_sym_fallback!("cuMemFree_v2", "cuMemFree", FnCuMemFree);
            let cu_memcpy_htod: FnCuMemcpyHtoD = resolve_sym_fallback!("cuMemcpyHtoD_v2", "cuMemcpyHtoD", FnCuMemcpyHtoD);
            let cu_memcpy_dtoh: FnCuMemcpyDtoH = resolve_sym_fallback!("cuMemcpyDtoH_v2", "cuMemcpyDtoH", FnCuMemcpyDtoH);
            let cu_launch_kernel: FnCuLaunchKernel = resolve_sym!("cuLaunchKernel", FnCuLaunchKernel);
            let cu_get_error_string: FnCuGetErrorString = resolve_sym!("cuGetErrorString", FnCuGetErrorString);

            // Initialize CUDA Driver API
            let res = cu_init(0);
            if res != CUDA_SUCCESS {
                return Err(format!("cuInit(0) failed with error code: {}", res));
            }

            Ok(Self {
                lib_handle: Arc::new(CudaLibraryHandle { raw_handle }),
                vtable: CudaDriverVTable {
                    cu_init,
                    cu_driver_get_version,
                    cu_device_get_count,
                    cu_device_get,
                    cu_device_get_name,
                    cu_device_total_mem,
                    cu_ctx_create,
                    cu_ctx_destroy,
                    cu_ctx_set_current,
                    cu_ctx_synchronize,
                    cu_module_load_data,
                    cu_module_get_function,
                    cu_module_unload,
                    cu_mem_alloc,
                    cu_mem_free,
                    cu_memcpy_htod,
                    cu_memcpy_dtoh,
                    cu_launch_kernel,
                    cu_get_error_string,
                },
            })
        }
    }

    /// Translates a CUDA error code into a human-readable string
    pub fn get_error_string(&self, err: CUresult) -> String {
        unsafe {
            let mut ptr: *const u8 = std::ptr::null();
            if (self.vtable.cu_get_error_string)(err, &mut ptr) == CUDA_SUCCESS && !ptr.is_null() {
                CStr::from_ptr(ptr as *const std::os::raw::c_char)
                    .to_string_lossy()
                    .into_owned()
            } else {
                format!("CUDA Error Code {}", err)
            }
        }
    }

    /// Queries the installed driver version (e.g. 13010 for CUDA 13.1)
    pub fn driver_version(&self) -> Result<i32, String> {
        let mut ver: i32 = 0;
        let res = unsafe { (self.vtable.cu_driver_get_version)(&mut ver) };
        if res == CUDA_SUCCESS {
            Ok(ver)
        } else {
            Err(self.get_error_string(res))
        }
    }

    /// Returns the number of physical CUDA GPUs available
    pub fn device_count(&self) -> Result<i32, String> {
        let mut count: i32 = 0;
        let res = unsafe { (self.vtable.cu_device_get_count)(&mut count) };
        if res == CUDA_SUCCESS {
            Ok(count)
        } else {
            Err(self.get_error_string(res))
        }
    }

    /// Returns the ASCII model name of a GPU ordinal
    pub fn device_name(&self, ordinal: i32) -> Result<String, String> {
        let mut dev: CUdevice = 0;
        let res = unsafe { (self.vtable.cu_device_get)(&mut dev, ordinal) };
        if res != CUDA_SUCCESS {
            return Err(self.get_error_string(res));
        }

        let mut buf = [0u8; 256];
        let res = unsafe { (self.vtable.cu_device_get_name)(buf.as_mut_ptr(), buf.len() as i32, dev) };
        if res != CUDA_SUCCESS {
            return Err(self.get_error_string(res));
        }

        let cstr = unsafe { CStr::from_ptr(buf.as_ptr() as *const std::os::raw::c_char) };
        Ok(cstr.to_string_lossy().into_owned())
    }

    /// Returns total VRAM capacity in bytes for a GPU ordinal
    pub fn device_total_memory(&self, ordinal: i32) -> Result<usize, String> {
        let mut dev: CUdevice = 0;
        let res = unsafe { (self.vtable.cu_device_get)(&mut dev, ordinal) };
        if res != CUDA_SUCCESS {
            return Err(self.get_error_string(res));
        }

        let mut bytes: usize = 0;
        let res = unsafe { (self.vtable.cu_device_total_mem)(&mut bytes, dev) };
        if res == CUDA_SUCCESS {
            Ok(bytes)
        } else {
            Err(self.get_error_string(res))
        }
    }

    /// Creates an active compute context on the specified GPU ordinal
    pub fn create_context(&self, ordinal: i32) -> Result<CudaContext, String> {
        let mut dev: CUdevice = 0;
        let res = unsafe { (self.vtable.cu_device_get)(&mut dev, ordinal) };
        if res != CUDA_SUCCESS {
            return Err(self.get_error_string(res));
        }

        let mut ctx: CUcontext = std::ptr::null_mut();
        let res = unsafe { (self.vtable.cu_ctx_create)(&mut ctx, 0, dev) };
        if res != CUDA_SUCCESS {
            return Err(format!("Failed to create CUDA context: {}", self.get_error_string(res)));
        }

        Ok(CudaContext {
            driver: self.clone(),
            raw_ctx: ctx,
            active: Arc::new(AtomicBool::new(true)),
        })
    }
}

/// Managed CUDA Compute Context with RAII Destruction
pub struct CudaContext {
    pub driver: CudaDriver,
    pub raw_ctx: CUcontext,
    active: Arc<AtomicBool>,
}

unsafe impl Send for CudaContext {}
unsafe impl Sync for CudaContext {}

impl Drop for CudaContext {
    fn drop(&mut self) {
        if self.active.load(Ordering::SeqCst) && !self.raw_ctx.is_null() {
            unsafe {
                (self.driver.vtable.cu_ctx_destroy)(self.raw_ctx);
            }
            self.active.store(false, Ordering::SeqCst);
        }
    }
}

impl CudaContext {
    /// Sets this context as the current active context on the calling thread
    pub fn make_current(&self) -> Result<(), String> {
        let res = unsafe { (self.driver.vtable.cu_ctx_set_current)(self.raw_ctx) };
        if res == CUDA_SUCCESS {
            Ok(())
        } else {
            Err(self.driver.get_error_string(res))
        }
    }

    /// Synchronizes all pending GPU operations in this context
    pub fn synchronize(&self) -> Result<(), String> {
        let res = unsafe { (self.driver.vtable.cu_ctx_synchronize)() };
        if res == CUDA_SUCCESS {
            Ok(())
        } else {
            Err(self.driver.get_error_string(res))
        }
    }

    /// Allocates `size_bytes` of contiguous VRAM on the GPU
    pub fn alloc(&self, size_bytes: usize) -> Result<CudaDeviceBuffer, String> {
        let mut dptr: CUdeviceptr = 0;
        let res = unsafe { (self.driver.vtable.cu_mem_alloc)(&mut dptr, size_bytes) };
        if res != CUDA_SUCCESS {
            return Err(format!("cuMemAlloc failed for {} bytes: {}", size_bytes, self.driver.get_error_string(res)));
        }

        Ok(CudaDeviceBuffer {
            driver: self.driver.clone(),
            dptr,
            size_bytes,
        })
    }

    /// Copies data from host slice to device memory buffer
    pub fn copy_htod<T>(&self, dst: &CudaDeviceBuffer, src: &[T]) -> Result<(), String> {
        let bytes = src.len() * std::mem::size_of::<T>();
        if bytes > dst.size_bytes {
            return Err(format!("Buffer overflow: copying {} bytes into {} byte VRAM buffer", bytes, dst.size_bytes));
        }

        let res = unsafe {
            (self.driver.vtable.cu_memcpy_htod)(dst.dptr, src.as_ptr() as *const c_void, bytes)
        };
        if res == CUDA_SUCCESS {
            Ok(())
        } else {
            Err(self.driver.get_error_string(res))
        }
    }

    /// Copies data from device memory buffer to host slice
    pub fn copy_dtoh<T>(&self, src: &CudaDeviceBuffer, dst: &mut [T]) -> Result<(), String> {
        let bytes = dst.len() * std::mem::size_of::<T>();
        if bytes > src.size_bytes {
            return Err(format!("Buffer underflow: reading {} bytes from {} byte VRAM buffer", bytes, src.size_bytes));
        }

        let res = unsafe {
            (self.driver.vtable.cu_memcpy_dtoh)(dst.as_mut_ptr() as *mut c_void, src.dptr, bytes)
        };
        if res == CUDA_SUCCESS {
            Ok(())
        } else {
            Err(self.driver.get_error_string(res))
        }
    }

    /// Compiles and loads a PTX assembly string dynamically into this context
    pub fn load_ptx(&self, ptx_code: &str) -> Result<CudaModule, String> {
        let c_ptx = CString::new(ptx_code)
            .map_err(|e| format!("Invalid PTX string null byte: {}", e))?;

        let mut module: CUmodule = std::ptr::null_mut();
        let res = unsafe {
            (self.driver.vtable.cu_module_load_data)(&mut module, c_ptx.as_ptr() as *const c_void)
        };
        if res != CUDA_SUCCESS {
            return Err(format!("PTX JIT compilation/load failed: {}", self.driver.get_error_string(res)));
        }

        Ok(CudaModule {
            driver: self.driver.clone(),
            module,
        })
    }
}

/// RAII GPU Device Memory Allocation
pub struct CudaDeviceBuffer {
    driver: CudaDriver,
    pub dptr: CUdeviceptr,
    pub size_bytes: usize,
}

unsafe impl Send for CudaDeviceBuffer {}
unsafe impl Sync for CudaDeviceBuffer {}

impl Drop for CudaDeviceBuffer {
    fn drop(&mut self) {
        if self.dptr != 0 {
            unsafe {
                (self.driver.vtable.cu_mem_free)(self.dptr);
            }
            self.dptr = 0;
        }
    }
}

/// Loaded CUDA Module
pub struct CudaModule {
    driver: CudaDriver,
    module: CUmodule,
}

unsafe impl Send for CudaModule {}
unsafe impl Sync for CudaModule {}

impl Drop for CudaModule {
    fn drop(&mut self) {
        if !self.module.is_null() {
            unsafe {
                (self.driver.vtable.cu_module_unload)(self.module);
            }
            self.module = std::ptr::null_mut();
        }
    }
}

impl CudaModule {
    /// Retrieves an entry function kernel from the loaded PTX module
    pub fn get_kernel(&self, name: &str) -> Result<CudaKernel, String> {
        let mut null_name = name.as_bytes().to_vec();
        null_name.push(0);

        let mut func: CUfunction = std::ptr::null_mut();
        let res = unsafe {
            (self.driver.vtable.cu_module_get_function)(&mut func, self.module, null_name.as_ptr())
        };
        if res != CUDA_SUCCESS {
            return Err(format!("Kernel '{}' not found in PTX module: {}", name, self.driver.get_error_string(res)));
        }

        Ok(CudaKernel {
            driver: self.driver.clone(),
            func,
            name: name.to_string(),
        })
    }
}

/// Entry Function CUDA Kernel
pub struct CudaKernel {
    driver: CudaDriver,
    func: CUfunction,
    pub name: String,
}

unsafe impl Send for CudaKernel {}
unsafe impl Sync for CudaKernel {}

impl CudaKernel {
    /// Launches the kernel with the specified grid/block dimensions and raw parameter pointers
    pub unsafe fn launch_raw(
        &self,
        grid: (u32, u32, u32),
        block: (u32, u32, u32),
        shared_mem: u32,
        kernel_params: &mut [*mut c_void],
    ) -> Result<(), String> {
        let res = (self.driver.vtable.cu_launch_kernel)(
            self.func,
            grid.0, grid.1, grid.2,
            block.0, block.1, block.2,
            shared_mem,
            std::ptr::null_mut(),
            kernel_params.as_mut_ptr(),
            std::ptr::null_mut(),
        );

        if res == CUDA_SUCCESS {
            Ok(())
        } else {
            Err(format!("Kernel launch '{}' failed: {}", self.name, self.driver.get_error_string(res)))
        }
    }
}

/// Turnkey One-Shot PTX Kernel Execution Engine
///
/// Compiles PTX code, uploads inputs to GPU VRAM, executes the kernel on actual NVIDIA hardware,
/// and reads back the computed result slice into host memory.
pub fn run_ptx_vector_kernel(
    ptx_code: &str,
    kernel_name: &str,
    grid: (u32, u32, u32),
    block: (u32, u32, u32),
    inputs: &[&[f32]],
    output_len: usize,
) -> Result<Vec<f32>, String> {
    let driver = CudaDriver::init()?;
    let ctx = driver.create_context(0)?;
    ctx.make_current()?;

    let module = ctx.load_ptx(ptx_code)?;
    let kernel = module.get_kernel(kernel_name)?;

    // Allocate input device buffers & copy data
    let mut dev_inputs = Vec::with_capacity(inputs.len());
    for inp in inputs {
        let bytes = inp.len() * std::mem::size_of::<f32>();
        let d_buf = ctx.alloc(bytes)?;
        ctx.copy_htod(&d_buf, inp)?;
        dev_inputs.push(d_buf);
    }

    // Allocate output device buffer
    let out_bytes = output_len * std::mem::size_of::<f32>();
    let d_out = ctx.alloc(out_bytes)?;

    let n = output_len as u32;

    // Prepare parameter pointers
    // By convention: [d_in0, d_in1, ..., d_out, n]
    let mut param_pointers: Vec<*mut c_void> = Vec::new();
    let mut d_ptrs: Vec<CUdeviceptr> = dev_inputs.iter().map(|b| b.dptr).collect();
    let mut d_out_ptr = d_out.dptr;
    let mut n_param = n;

    for ptr in &mut d_ptrs {
        param_pointers.push(ptr as *mut CUdeviceptr as *mut c_void);
    }
    param_pointers.push(&mut d_out_ptr as *mut CUdeviceptr as *mut c_void);
    param_pointers.push(&mut n_param as *mut u32 as *mut c_void);

    unsafe {
        kernel.launch_raw(grid, block, 0, &mut param_pointers)?;
    }

    ctx.synchronize()?;

    // Read back results
    let mut result = vec![0.0f32; output_len];
    ctx.copy_dtoh(&d_out, &mut result)?;

    Ok(result)
}
