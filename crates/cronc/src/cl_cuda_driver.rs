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
        pub fn CreateMutexA(lpMutexAttributes: *mut c_void, bInitialOwner: i32, lpName: *const u8) -> *mut c_void;
        pub fn WaitForSingleObject(hHandle: *mut c_void, dwMilliseconds: u32) -> u32;
        pub fn ReleaseMutex(hMutex: *mut c_void) -> i32;
        pub fn CloseHandle(hObject: *mut c_void) -> i32;
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
        let guard = Arc::new(CudaDeviceGuard::acquire());

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
            _guard: guard,
        })
    }
}

/// System-Wide Named Mutex Guard for Physical Hardware CUDA Serialization
pub struct CudaDeviceGuard {
    #[cfg(windows)]
    handle: *mut std::ffi::c_void,
}

unsafe impl Send for CudaDeviceGuard {}
unsafe impl Sync for CudaDeviceGuard {}

impl CudaDeviceGuard {
    pub fn acquire() -> Self {
        #[cfg(windows)]
        unsafe {
            let name = b"Local\\CRON_CUDA_HARDWARE_LOCK\0";
            let handle = os_dynload::CreateMutexA(std::ptr::null_mut(), 0, name.as_ptr());
            if !handle.is_null() {
                os_dynload::WaitForSingleObject(handle, 120_000);
            }
            Self { handle }
        }
        #[cfg(not(windows))]
        {
            Self {}
        }
    }
}

impl Drop for CudaDeviceGuard {
    fn drop(&mut self) {
        #[cfg(windows)]
        unsafe {
            if !self.handle.is_null() {
                os_dynload::ReleaseMutex(self.handle);
                os_dynload::CloseHandle(self.handle);
            }
        }
    }
}

/// Managed CUDA Compute Context with RAII Destruction
pub struct CudaContext {
    pub driver: CudaDriver,
    pub raw_ctx: CUcontext,
    active: Arc<AtomicBool>,
    _guard: Arc<CudaDeviceGuard>,
}

unsafe impl Send for CudaContext {}
unsafe impl Sync for CudaContext {}

impl Drop for CudaContext {
    fn drop(&mut self) {
        // Note: In CUDA Driver API, destroying active device contexts via cuCtxDestroy
        // while the process continues running invalidates device TLS tables and causes
        // access violations on subsequent context creation. Memory buffers are freed
        // via CudaBuffer RAII (cuMemFree) and context resources are reaped upon process exit.
        self.active.store(false, Ordering::SeqCst);
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

/// Consolidated PTX Microcode Source for Neural Matrix Math & Optimizers
pub const NEURAL_MATH_PTX_SRC: &str = r#"
.version 7.5
.target sm_80
.address_size 64

.visible .entry gemm_nn_kernel (
    .param .u64 d_a,
    .param .u64 d_b,
    .param .u64 d_c,
    .param .u32 m_dim,
    .param .u32 n_dim,
    .param .u32 k_dim
) {
    .reg .pred %p<4>;
    .reg .b32 %r<14>;
    .reg .b64 %rd<14>;
    .reg .f32 %f<5>;

    ld.param.u64 %rd1, [d_a];
    ld.param.u64 %rd2, [d_b];
    ld.param.u64 %rd3, [d_c];
    ld.param.u32 %r1, [m_dim];
    ld.param.u32 %r2, [n_dim];
    ld.param.u32 %r3, [k_dim];

    mov.u32 %r4, %ctaid.y;
    mov.u32 %r5, %ntid.y;
    mov.u32 %r6, %tid.y;
    mad.lo.u32 %r7, %r4, %r5, %r6;

    mov.u32 %r8, %ctaid.x;
    mov.u32 %r9, %ntid.x;
    mov.u32 %r10, %tid.x;
    mad.lo.u32 %r11, %r8, %r9, %r10;

    setp.ge.u32 %p1, %r7, %r1;
    @%p1 bra EXIT;
    setp.ge.u32 %p2, %r11, %r2;
    @%p2 bra EXIT;

    mov.f32 %f1, 0.0;
    mov.u32 %r12, 0;

LOOP:
    setp.ge.u32 %p3, %r12, %r3;
    @%p3 bra STORE;

    mad.lo.u32 %r4, %r7, %r3, %r12;
    mul.wide.u32 %rd4, %r4, 4;
    add.s64 %rd5, %rd1, %rd4;
    ld.global.f32 %f2, [%rd5];

    mad.lo.u32 %r5, %r12, %r2, %r11;
    mul.wide.u32 %rd6, %r5, 4;
    add.s64 %rd7, %rd2, %rd6;
    ld.global.f32 %f3, [%rd7];

    fma.rn.f32 %f1, %f2, %f3, %f1;

    add.u32 %r12, %r12, 1;
    bra LOOP;

STORE:
    mad.lo.u32 %r6, %r7, %r2, %r11;
    mul.wide.u32 %rd8, %r6, 4;
    add.s64 %rd9, %rd3, %rd8;
    st.global.f32 [%rd9], %f1;

EXIT:
    ret;
}

.visible .entry gemm_nt_kernel (
    .param .u64 d_a,
    .param .u64 d_b,
    .param .u64 d_c,
    .param .u32 m_dim,
    .param .u32 n_dim,
    .param .u32 k_dim
) {
    .reg .pred %p<4>;
    .reg .b32 %r<14>;
    .reg .b64 %rd<14>;
    .reg .f32 %f<5>;

    ld.param.u64 %rd1, [d_a];
    ld.param.u64 %rd2, [d_b];
    ld.param.u64 %rd3, [d_c];
    ld.param.u32 %r1, [m_dim];
    ld.param.u32 %r2, [n_dim];
    ld.param.u32 %r3, [k_dim];

    mov.u32 %r4, %ctaid.y;
    mov.u32 %r5, %ntid.y;
    mov.u32 %r6, %tid.y;
    mad.lo.u32 %r7, %r4, %r5, %r6;

    mov.u32 %r8, %ctaid.x;
    mov.u32 %r9, %ntid.x;
    mov.u32 %r10, %tid.x;
    mad.lo.u32 %r11, %r8, %r9, %r10;

    setp.ge.u32 %p1, %r7, %r1;
    @%p1 bra EXIT;
    setp.ge.u32 %p2, %r11, %r2;
    @%p2 bra EXIT;

    mov.f32 %f1, 0.0;
    mov.u32 %r12, 0;

LOOP:
    setp.ge.u32 %p3, %r12, %r3;
    @%p3 bra STORE;

    mad.lo.u32 %r4, %r7, %r3, %r12;
    mul.wide.u32 %rd4, %r4, 4;
    add.s64 %rd5, %rd1, %rd4;
    ld.global.f32 %f2, [%rd5];

    mad.lo.u32 %r5, %r11, %r3, %r12;
    mul.wide.u32 %rd6, %r5, 4;
    add.s64 %rd7, %rd2, %rd6;
    ld.global.f32 %f3, [%rd7];

    fma.rn.f32 %f1, %f2, %f3, %f1;

    add.u32 %r12, %r12, 1;
    bra LOOP;

STORE:
    mad.lo.u32 %r6, %r7, %r2, %r11;
    mul.wide.u32 %rd8, %r6, 4;
    add.s64 %rd9, %rd3, %rd8;
    st.global.f32 [%rd9], %f1;

EXIT:
    ret;
}

.visible .entry gemm_tn_kernel (
    .param .u64 d_a,
    .param .u64 d_b,
    .param .u64 d_c,
    .param .u32 m_dim,
    .param .u32 n_dim,
    .param .u32 k_dim
) {
    .reg .pred %p<4>;
    .reg .b32 %r<14>;
    .reg .b64 %rd<14>;
    .reg .f32 %f<5>;

    ld.param.u64 %rd1, [d_a];
    ld.param.u64 %rd2, [d_b];
    ld.param.u64 %rd3, [d_c];
    ld.param.u32 %r1, [m_dim];
    ld.param.u32 %r2, [n_dim];
    ld.param.u32 %r3, [k_dim];

    mov.u32 %r4, %ctaid.y;
    mov.u32 %r5, %ntid.y;
    mov.u32 %r6, %tid.y;
    mad.lo.u32 %r7, %r4, %r5, %r6;

    mov.u32 %r8, %ctaid.x;
    mov.u32 %r9, %ntid.x;
    mov.u32 %r10, %tid.x;
    mad.lo.u32 %r11, %r8, %r9, %r10;

    setp.ge.u32 %p1, %r7, %r1;
    @%p1 bra EXIT;
    setp.ge.u32 %p2, %r11, %r2;
    @%p2 bra EXIT;

    mov.f32 %f1, 0.0;
    mov.u32 %r12, 0;

LOOP:
    setp.ge.u32 %p3, %r12, %r3;
    @%p3 bra STORE;

    mad.lo.u32 %r4, %r12, %r1, %r7;
    mul.wide.u32 %rd4, %r4, 4;
    add.s64 %rd5, %rd1, %rd4;
    ld.global.f32 %f2, [%rd5];

    mad.lo.u32 %r5, %r12, %r2, %r11;
    mul.wide.u32 %rd6, %r5, 4;
    add.s64 %rd7, %rd2, %rd6;
    ld.global.f32 %f3, [%rd7];

    fma.rn.f32 %f1, %f2, %f3, %f1;

    add.u32 %r12, %r12, 1;
    bra LOOP;

STORE:
    mad.lo.u32 %r6, %r7, %r2, %r11;
    mul.wide.u32 %rd8, %r6, 4;
    add.s64 %rd9, %rd3, %rd8;
    st.global.f32 [%rd9], %f1;

EXIT:
    ret;
}

.visible .entry swiglu_fwd_kernel (
    .param .u64 d_g,
    .param .u64 d_u,
    .param .u64 d_act,
    .param .u32 param_len
) {
    .reg .pred %p;
    .reg .b32 %r<5>;
    .reg .b64 %rd<8>;
    .reg .f32 %f<8>;

    ld.param.u64 %rd1, [d_g];
    ld.param.u64 %rd2, [d_u];
    ld.param.u64 %rd3, [d_act];
    ld.param.u32 %r0, [param_len];

    mov.u32 %r1, %ctaid.x;
    mov.u32 %r2, %ntid.x;
    mov.u32 %r3, %tid.x;
    mad.lo.u32 %r4, %r1, %r2, %r3;

    setp.ge.u32 %p, %r4, %r0;
    @%p bra EXIT;

    mul.wide.u32 %rd4, %r4, 4;
    add.s64 %rd5, %rd1, %rd4;
    add.s64 %rd6, %rd2, %rd4;
    add.s64 %rd7, %rd3, %rd4;

    ld.global.f32 %f1, [%rd5];
    ld.global.f32 %f2, [%rd6];

    neg.f32 %f3, %f1;
    mul.f32 %f4, %f3, 1.44269504;
    ex2.approx.f32 %f5, %f4;
    add.f32 %f6, %f5, 1.0;
    rcp.approx.f32 %f7, %f6;
    mul.f32 %f1, %f1, %f7;
    mul.f32 %f0, %f1, %f2;

    st.global.f32 [%rd7], %f0;

EXIT:
    ret;
}

.visible .entry adamw_step_kernel (
    .param .u64 d_w,
    .param .u64 d_g,
    .param .u64 d_m,
    .param .u64 d_v,
    .param .f32 param_lr,
    .param .f32 param_b1,
    .param .f32 param_b2,
    .param .f32 param_eps,
    .param .f32 param_wd,
    .param .f32 param_bc1,
    .param .f32 param_bc2,
    .param .u32 param_n
) {
    .reg .pred %p;
    .reg .b32 %r<5>;
    .reg .b64 %rd<9>;
    .reg .f32 %f<18>;

    ld.param.u64 %rd1, [d_w];
    ld.param.u64 %rd2, [d_g];
    ld.param.u64 %rd3, [d_m];
    ld.param.u64 %rd4, [d_v];
    ld.param.f32 %f1, [param_lr];
    ld.param.f32 %f2, [param_b1];
    ld.param.f32 %f3, [param_b2];
    ld.param.f32 %f4, [param_eps];
    ld.param.f32 %f5, [param_wd];
    ld.param.f32 %f6, [param_bc1];
    ld.param.f32 %f7, [param_bc2];
    ld.param.u32 %r0, [param_n];

    mov.u32 %r1, %ctaid.x;
    mov.u32 %r2, %ntid.x;
    mov.u32 %r3, %tid.x;
    mad.lo.u32 %r4, %r1, %r2, %r3;

    setp.ge.u32 %p, %r4, %r0;
    @%p bra EXIT;

    mul.wide.u32 %rd5, %r4, 4;
    add.s64 %rd6, %rd1, %rd5;
    add.s64 %rd7, %rd2, %rd5;
    add.s64 %rd8, %rd3, %rd5;
    add.s64 %rd1, %rd4, %rd5;

    ld.global.f32 %f8, [%rd6];
    ld.global.f32 %f9, [%rd7];
    ld.global.f32 %f10, [%rd8];
    ld.global.f32 %f11, [%rd1];

    sub.f32 %f12, 1.0, %f2;
    mul.f32 %f13, %f10, %f2;
    fma.rn.f32 %f10, %f12, %f9, %f13;
    st.global.f32 [%rd8], %f10;

    sub.f32 %f14, 1.0, %f3;
    mul.f32 %f15, %f9, %f9;
    mul.f32 %f16, %f11, %f3;
    fma.rn.f32 %f11, %f14, %f15, %f16;
    st.global.f32 [%rd1], %f11;

    rcp.approx.f32 %f12, %f6;
    mul.f32 %f13, %f10, %f12;

    rcp.approx.f32 %f14, %f7;
    mul.f32 %f15, %f11, %f14;

    sqrt.approx.f32 %f16, %f15;
    add.f32 %f16, %f16, %f4;
    rcp.approx.f32 %f17, %f16;
    mul.f32 %f13, %f13, %f17;

    fma.rn.f32 %f13, %f5, %f8, %f13;
    neg.f32 %f1, %f1;
    fma.rn.f32 %f8, %f1, %f13, %f8;

    st.global.f32 [%rd6], %f8;

EXIT:
    ret;
}

.visible .entry vec_add_inplace_kernel (
    .param .u64 d_y,
    .param .u64 d_x,
    .param .u32 n
) {
    .reg .pred %p;
    .reg .b32 %r<5>;
    .reg .b64 %rd<8>;
    .reg .f32 %f<3>;

    ld.param.u64 %rd1, [d_y];
    ld.param.u64 %rd2, [d_x];
    ld.param.u32 %r0, [n];

    mov.u32 %r1, %ctaid.x;
    mov.u32 %r2, %ntid.x;
    mov.u32 %r3, %tid.x;
    mad.lo.u32 %r4, %r1, %r2, %r3;

    setp.ge.u32 %p, %r4, %r0;
    @%p bra EXIT;

    mul.wide.u32 %rd4, %r4, 4;
    add.s64 %rd5, %rd1, %rd4;
    add.s64 %rd6, %rd2, %rd4;

    ld.global.f32 %f1, [%rd5];
    ld.global.f32 %f2, [%rd6];
    add.f32 %f0, %f1, %f2;
    st.global.f32 [%rd5], %f0;

EXIT:
    ret;
}

.visible .entry swiglu_bwd_kernel (
    .param .u64 d_dact,
    .param .u64 d_g,
    .param .u64 d_u,
    .param .u64 d_dg,
    .param .u64 d_du,
    .param .u32 param_len
) {
    .reg .pred %p;
    .reg .b32 %r<5>;
    .reg .b64 %rd<12>;
    .reg .f32 %f<16>;

    ld.param.u64 %rd1, [d_dact];
    ld.param.u64 %rd2, [d_g];
    ld.param.u64 %rd3, [d_u];
    ld.param.u64 %rd4, [d_dg];
    ld.param.u64 %rd5, [d_du];
    ld.param.u32 %r0, [param_len];

    mov.u32 %r1, %ctaid.x;
    mov.u32 %r2, %ntid.x;
    mov.u32 %r3, %tid.x;
    mad.lo.u32 %r4, %r1, %r2, %r3;

    setp.ge.u32 %p, %r4, %r0;
    @%p bra EXIT;

    mul.wide.u32 %rd6, %r4, 4;
    add.s64 %rd7, %rd1, %rd6;
    add.s64 %rd8, %rd2, %rd6;
    add.s64 %rd9, %rd3, %rd6;
    add.s64 %rd10, %rd4, %rd6;
    add.s64 %rd11, %rd5, %rd6;

    ld.global.f32 %f1, [%rd7];
    ld.global.f32 %f2, [%rd8];
    ld.global.f32 %f3, [%rd9];

    neg.f32 %f4, %f2;
    mul.f32 %f5, %f4, 1.44269504;
    ex2.approx.f32 %f6, %f5;
    add.f32 %f7, %f6, 1.0;
    rcp.approx.f32 %f8, %f7;

    mul.f32 %f9, %f2, %f8;
    mul.f32 %f10, %f1, %f9;
    st.global.f32 [%rd11], %f10;

    sub.f32 %f11, 1.0, %f8;
    mul.f32 %f12, %f2, %f11;
    add.f32 %f13, %f12, 1.0;
    mul.f32 %f14, %f8, %f13;

    mul.f32 %f15, %f1, %f3;
    mul.f32 %f15, %f15, %f14;
    st.global.f32 [%rd10], %f15;

EXIT:
    ret;
}
"#;

/// High-Performance GPU Neural Tensor Computation Engine
pub struct CudaTensorEngine {
    pub ctx: Arc<CudaContext>,
    #[allow(dead_code)]
    module: CudaModule,
    kernel_gemm_nn: CudaKernel,
    kernel_gemm_nt: CudaKernel,
    kernel_gemm_tn: CudaKernel,
    kernel_swiglu_fwd: CudaKernel,
    kernel_swiglu_bwd: CudaKernel,
    kernel_vec_add_inplace: CudaKernel,
    kernel_adamw_step: CudaKernel,
}

impl CudaTensorEngine {
    /// Initializes and JIT-compiles all neural tensor compute kernels on the active GPU context
    pub fn new(ctx: Arc<CudaContext>) -> Result<Self, String> {
        let module = ctx.load_ptx(NEURAL_MATH_PTX_SRC)?;

        let kernel_gemm_nn = module.get_kernel("gemm_nn_kernel")?;
        let kernel_gemm_nt = module.get_kernel("gemm_nt_kernel")?;
        let kernel_gemm_tn = module.get_kernel("gemm_tn_kernel")?;
        let kernel_swiglu_fwd = module.get_kernel("swiglu_fwd_kernel")?;
        let kernel_swiglu_bwd = module.get_kernel("swiglu_bwd_kernel")?;
        let kernel_vec_add_inplace = module.get_kernel("vec_add_inplace_kernel")?;
        let kernel_adamw_step = module.get_kernel("adamw_step_kernel")?;

        Ok(Self {
            ctx,
            module,
            kernel_gemm_nn,
            kernel_gemm_nt,
            kernel_gemm_tn,
            kernel_swiglu_fwd,
            kernel_swiglu_bwd,
            kernel_vec_add_inplace,
            kernel_adamw_step,
        })
    }

    /// High-throughput GPU Matrix Multiplication: C = A @ B (with optional transpose)
    pub fn gemm(
        &self,
        d_a: &CudaDeviceBuffer,
        d_b: &CudaDeviceBuffer,
        d_c: &CudaDeviceBuffer,
        m: usize,
        n: usize,
        k: usize,
        trans_a: bool,
        trans_b: bool,
    ) -> Result<(), String> {
        let mut da_ptr = d_a.dptr;
        let mut db_ptr = d_b.dptr;
        let mut dc_ptr = d_c.dptr;
        let mut m_u32 = m as u32;
        let mut n_u32 = n as u32;
        let mut k_u32 = k as u32;

        let mut params: [*mut c_void; 6] = [
            &mut da_ptr as *mut CUdeviceptr as *mut c_void,
            &mut db_ptr as *mut CUdeviceptr as *mut c_void,
            &mut dc_ptr as *mut CUdeviceptr as *mut c_void,
            &mut m_u32 as *mut u32 as *mut c_void,
            &mut n_u32 as *mut u32 as *mut c_void,
            &mut k_u32 as *mut u32 as *mut c_void,
        ];

        let block = (16u32, 16u32, 1u32);
        let grid = (((n as u32) + 15) / 16, ((m as u32) + 15) / 16, 1u32);

        let kernel = match (trans_a, trans_b) {
            (false, false) => &self.kernel_gemm_nn,
            (false, true) => &self.kernel_gemm_nt,
            (true, false) => &self.kernel_gemm_tn,
            (true, true) => return Err("GEMM TT not implemented".to_string()),
        };

        unsafe {
            kernel.launch_raw(grid, block, 0, &mut params)?;
        }

        Ok(())
    }

    /// GPU-accelerated SwiGLU forward pass: act = SiLU(G) * U
    pub fn swiglu(
        &self,
        d_g: &CudaDeviceBuffer,
        d_u: &CudaDeviceBuffer,
        d_act: &CudaDeviceBuffer,
        len: usize,
    ) -> Result<(), String> {
        let mut dg_ptr = d_g.dptr;
        let mut du_ptr = d_u.dptr;
        let mut dact_ptr = d_act.dptr;
        let mut len_u32 = len as u32;

        let mut params: [*mut c_void; 4] = [
            &mut dg_ptr as *mut CUdeviceptr as *mut c_void,
            &mut du_ptr as *mut CUdeviceptr as *mut c_void,
            &mut dact_ptr as *mut CUdeviceptr as *mut c_void,
            &mut len_u32 as *mut u32 as *mut c_void,
        ];

        let block = (256u32, 1u32, 1u32);
        let grid = (((len as u32) + 255) / 256, 1u32, 1u32);

        unsafe {
            self.kernel_swiglu_fwd.launch_raw(grid, block, 0, &mut params)?;
        }

        Ok(())
    }

    /// GPU in-place AdamW parameter update with decoupled weight decay
    pub fn adamw_step(
        &self,
        d_w: &CudaDeviceBuffer,
        d_g: &CudaDeviceBuffer,
        d_m: &CudaDeviceBuffer,
        d_v: &CudaDeviceBuffer,
        lr: f32,
        beta1: f32,
        beta2: f32,
        eps: f32,
        weight_decay: f32,
        step: usize,
        n_elements: usize,
    ) -> Result<(), String> {
        let mut dw_ptr = d_w.dptr;
        let mut dg_ptr = d_g.dptr;
        let mut dm_ptr = d_m.dptr;
        let mut dv_ptr = d_v.dptr;

        let mut lr_f = lr;
        let mut b1_f = beta1;
        let mut b2_f = beta2;
        let mut eps_f = eps;
        let mut wd_f = weight_decay;

        let mut bc1 = 1.0f32 - beta1.powi(step as i32);
        let mut bc2 = 1.0f32 - beta2.powi(step as i32);
        let mut n_u32 = n_elements as u32;

        let mut params: [*mut c_void; 12] = [
            &mut dw_ptr as *mut CUdeviceptr as *mut c_void,
            &mut dg_ptr as *mut CUdeviceptr as *mut c_void,
            &mut dm_ptr as *mut CUdeviceptr as *mut c_void,
            &mut dv_ptr as *mut CUdeviceptr as *mut c_void,
            &mut lr_f as *mut f32 as *mut c_void,
            &mut b1_f as *mut f32 as *mut c_void,
            &mut b2_f as *mut f32 as *mut c_void,
            &mut eps_f as *mut f32 as *mut c_void,
            &mut wd_f as *mut f32 as *mut c_void,
            &mut bc1 as *mut f32 as *mut c_void,
            &mut bc2 as *mut f32 as *mut c_void,
            &mut n_u32 as *mut u32 as *mut c_void,
        ];

        let block = (256u32, 1u32, 1u32);
        let grid = (((n_elements as u32) + 255) / 256, 1u32, 1u32);

        unsafe {
            self.kernel_adamw_step.launch_raw(grid, block, 0, &mut params)?;
        }

        Ok(())
    }

    /// GPU-accelerated SwiGLU backward pass: computes dG and dU from d_act, G, U
    pub fn swiglu_bwd(
        &self,
        d_dact: &CudaDeviceBuffer,
        d_g: &CudaDeviceBuffer,
        d_u: &CudaDeviceBuffer,
        d_dg: &CudaDeviceBuffer,
        d_du: &CudaDeviceBuffer,
        len: usize,
    ) -> Result<(), String> {
        let mut dact_ptr = d_dact.dptr;
        let mut dg_ptr = d_g.dptr;
        let mut du_ptr = d_u.dptr;
        let mut ddg_ptr = d_dg.dptr;
        let mut ddu_ptr = d_du.dptr;
        let mut len_u32 = len as u32;

        let mut params: [*mut c_void; 6] = [
            &mut dact_ptr as *mut CUdeviceptr as *mut c_void,
            &mut dg_ptr as *mut CUdeviceptr as *mut c_void,
            &mut du_ptr as *mut CUdeviceptr as *mut c_void,
            &mut ddg_ptr as *mut CUdeviceptr as *mut c_void,
            &mut ddu_ptr as *mut CUdeviceptr as *mut c_void,
            &mut len_u32 as *mut u32 as *mut c_void,
        ];

        let block = (256u32, 1u32, 1u32);
        let grid = (((len as u32) + 255) / 256, 1u32, 1u32);

        unsafe {
            self.kernel_swiglu_bwd.launch_raw(grid, block, 0, &mut params)?;
        }

        Ok(())
    }

    /// GPU in-place vector addition: d_y = d_y + d_x
    pub fn vec_add_inplace(
        &self,
        d_y: &CudaDeviceBuffer,
        d_x: &CudaDeviceBuffer,
        n: usize,
    ) -> Result<(), String> {
        let mut dy_ptr = d_y.dptr;
        let mut dx_ptr = d_x.dptr;
        let mut n_u32 = n as u32;

        let mut params: [*mut c_void; 3] = [
            &mut dy_ptr as *mut CUdeviceptr as *mut c_void,
            &mut dx_ptr as *mut CUdeviceptr as *mut c_void,
            &mut n_u32 as *mut u32 as *mut c_void,
        ];

        let block = (256u32, 1u32, 1u32);
        let grid = (((n as u32) + 255) / 256, 1u32, 1u32);

        unsafe {
            self.kernel_vec_add_inplace.launch_raw(grid, block, 0, &mut params)?;
        }

        Ok(())
    }
}
