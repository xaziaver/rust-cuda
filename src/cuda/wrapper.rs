use crate::CudaError;
use crate::cuda::ffi::cuMemAlloc;
use crate::cuda::ffi::CUdeviceptr;
use crate::cuda::ffi::CUresult;

pub fn allocate_memory(size: usize) -> Result<CUdeviceptr, CUresult> {
    let mut device_ptr: CUdeviceptr = 0;
    let result = unsafe { cuMemAlloc(&mut device_ptr, size) };

    match result {
        CUresult::CUDA_SUCCESS => Ok(device_ptr),
        _ => Err(result),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memory_allocation() {
        let result = allocate_memory(1024);
        assert!(result.is_ok());
    }
}
