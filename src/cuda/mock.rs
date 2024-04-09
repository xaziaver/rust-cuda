use mockall::automock;

#[automock]
mod cuda {
    pub trait CudaApi {
        fn allocate_memory(size: usize) -> Result<CUdeviceptr, CUresult>;
    }
}
