mod memory;    // GPU memory allocation, deallocation, data transfer between host/device
mod device;    // manage device selection, properties, and context management
mod kernel;    // focuses on compiling, managing, and executing CUDA kernels
mod cuda;      // low-level CUDA bindings
mod nvrtc;     // runtime compilation of CUDA kernels


// re-export key types and functionalities for easier access by library users
pub use memory::DeviceMemory
pub use device::Device
pub use kernel::{Kernel, KernelArg};


// future error handling
#[derive(Debug)]
pub enum CudaError {
    MemoryAllocationFailed,
    KernelLaunchFailed,
    CompilationError(String),
    // more errors based on CUDA operations
}


// module declarations
pub mod utils {
    // utilities for common operations, like compiling Rust to PTX
}
