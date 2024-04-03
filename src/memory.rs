
pub struct DeviceMemory<T> {
    ptr: *mut T,
    size: usize,
    // ownership and type safety around device memory
}

impl<T> DeviceMemory<T> {
    // methods for memory management: 
    // allocation, deallocation, read, write, ...
}
