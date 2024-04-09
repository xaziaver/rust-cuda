use super::CudaApi;
use super::ffi::{cuMemAlloc, CUdeviceptr, CUresult};

pub struct RealCudaApi;

impl CudaApi for RealCudaApi {
    fn allocate_memory(size: usize) -> Result<CUdeviceptr, CUresult> {
        let mut device_ptr: CUdeviceptr = 0;
        let result = unsafe { cuMemAlloc(&mut device_ptr, size) };

        match result {
            CUresult::CUDA_SUCCESS => Ok(device_ptr),
            _ => Err(result),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::{CudaApi, CudaApiImpl};
    use std::ptr;

    #[test]
    fn test_memory_allocation() {
        let cuda_api = CudaApiImpl;
        let result = acuda_api.allocate_memory(1024);
        assert!(result.is_ok());
    }
}
