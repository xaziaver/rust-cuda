pub mod ffi;
#[cfg(feature = "mock")]
mod mock;
#[cfg(not(feature = "mock"))]
pub mod wrapper;

use ffi::{CUdeviceptr, CUresult};
pub trait CudaApi {
    fn allocate_memory(size: usize) -> Result<CUdeviceptr, CUresult>; 
}

#[cfg(feature = "mock")]
pub use mock::MockCudaApi as CudaApiImpl;
#[cfg(not(feature = "mock"))]
pub use wrapper::RealCudaApi as CudaApiImpl;
