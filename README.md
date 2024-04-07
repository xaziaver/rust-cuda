# rust-cuda
The `rust-cuda` library aims to provide a Rustic interface for CUDA programming, enabling Rust developers to write and execute CUDA kernels directly from Rust.

This was created with the intention of being used specifically to create custom kernels to optimize a Mixture of Experts (MoE) layer in large language models (LLMs).

## How To Use
### Testing with mocking
Mocking framework `mockall` is used to define mock implementations for cuda and nvrtc in mock.rs files.
This can be used with the feature flag "mock" to run tests without needing a GPU or drivers:
 - `cargo test --features mock`
Without the feature flag, the 'real' implementations in wrapper.rs (for cuda) and compiler.rs (for nvrtc) will be used

### Custom Kernels to Optimize MoE layer in LLM
1. Initialization:
   - start by selecting a CUDA device and allocating necessary memory on the device using `device` and `memory` modules
2. Kernel Development:
   - instead of writing CUDA kernels in C/C++, write kernel logic directly in Rust
   - the library provides mechanisms to compile Rust code into CUDA-compatible kernels (through NVRTC)
3. Kernel Execution:
   - with kernels compiled, execute them by passing inputs (data, model parameters) from Rust to the GPU, perform the computation, and then retrieve the results back into Rust
4. Optimization Loop:
   - iteratively refine kernels and execution parameters based on performance measurements, all within Rust's ecosystem
5. Integration into Larger Models:
   - the kernels can be integrated

## Project Structure
The library is organized into several modules, each responsible for different aspects of CUDA programming:
- `src/lib.rs`: The main entry point of the library, tying together various modules and functionalities
- `src/memory.rs`: Handles GPU memory allocation, deallocation, and data transfers
- `src/device.rs`: Manages CUDA device selection and queries device properties
- `src/kernel.rs`: Focuses on compiling, managing, and executing CUDA kernels
- `src/cuda/mod.rs`:Contains low-level bindings and safe wrappers for CUDA API calls
  - `src/cuda/ffi.rs`: Raw FFI bindings to CUDA's C APIs
  - `src/cuda/wrapper.rs`: Safe, idiomatic Rust wrappers around the CUDA FFI bindings
  - `src/cuda/mock.rs`: Holds mock implementations
- `src/nvrtc/mod.rs`: Facilitates runtime compilation of CUDA kernels using NVRTC
  - `src/nvrtc/ffi.rs`: Raw FFI bindings to NVRTC's C APIs
  - `src/nvrtc/compiler.rs`: Higher-level functionality for compiling and managing CUDA code at runtime
  - `src/nvrtc/mock.rs`: Holds mock implementations

## Getting Started

(add instructions on how to build, run tests, and basic usage examples)
