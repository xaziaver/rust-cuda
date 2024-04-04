// Use Rust's Foreign Function Interface (FFI) to call needed CUDA APIs


// during compilation Rust generates placeholders for calls into the foreign library
// after compilation, the Rust linker is invoked to resolve the external C function calls,
// using information from build.rs and environment variables
// linking can be dynamic (shared libraries at runtime) or static (include library directly in binary)
// 


// within this block, list names and signatures of external C functions used
////////////////////////////////////////////////////////////////////////////
extern "C" {        // extern says functions are impl using "X" language ABI
    pub fn cuMemAlloc(dptr: *mut CUdeviceptr, bytesize: usize) -> CUresult;
}


// manually define custom types used by the C functions in Rust
///////////////////////////////////////////////////////////////
// docs.nvidia.com/cuda/cuda-driver-api/group__CUDA__TYPES.html#group__CUDA__TYPES_1gc6c391505e117393cc2558fff6bfc2e9
#[repr(C)]
pub enum CUresult {
    CUDA_SUCCESS = 0,
    CUDA_ERROR_INVALID_VALUE = 1,
    CUDA_ERROR_OUT_OF_MEMORY = 2,
    CUDA_ERROR_NOT_INITIALIZED = 3,
    CUDA_ERROR_DEINITIALIZED = 4,
    // go to docs and add more errors as needed, this is a small portion
}

// should match the size of a pointer per CUDA docs
pub type CUdeviceptr = usize;
