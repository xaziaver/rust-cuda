mod ffi;

// use 'mock' implementation when feature flag enabled
#[cfg(feature = "mock")]
mod mock;
#[cfg(feature = "mock")]
pub use mock::MockCudaApi as CudaApi;

// use 'real' implementation otherwise (must have GPU and drivers)
#[cfg(not(feature = "mock"))]
mod wrapper;
#[cfg(not(feature = "mock"))]
pub use wrapper::RealCudaApi as CudaApi;

