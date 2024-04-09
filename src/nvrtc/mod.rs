mod ffi;

/*
// use 'mock' implementation when feature flag enabled
#[cfg(feature = "mock")]
mod mock;
#[cfg(feature = "mock")]
pub use mock::MockvNvrtcApi as NvrtcApi;

// use 'real' implementation otherwise (must have GPU and drivers)
#[cfg(not(feature = "mock"))]
mod compiler;
#[cfg(not(feature = "mock"))]
pub use compiler::RealNvrtcApi as NvrtcApi;
*/
