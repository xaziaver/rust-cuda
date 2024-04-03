use crate::CudaError;
use crate::memory::DeviceMemory;

// high-level abstraction for CUDA device
pub struct Device {
    // device specific properties
    id: i32,
}

impl Device {
    // methods to manage and use a CUDA device
    pub fn new(id: i32) -> Result<Self, CudaError> {
        // initialize & check for errors
        Ok(Self { id })
    }
    pub fn allocate_memory<T>(&self, size: usize) -> Result<DeviceMemory<T>, CudaError> {
        unimplemented!()
    }
    
    // more device-related methods
}
