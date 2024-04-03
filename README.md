# rust-cuda
The `rust-cuda` library aims to provide a Rustic interface for CUDA programming, enabling Rust developers to write and execute CUDA kernels directly from Rust.

This was created with the intention of being used specifically to create custom kernels to optimize a Mixture of Experts (MoE) layer in large language models (LLMs).

## How To Use
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
   - the kernels can be integrated (with a focus on MoE layers)

## Project Structure
The library is organized into several modules, each responsible for different aspects of CUDA programming:
- `src/lib.rs`: The main entry point of the library, tying together various modules and functionalities
- `src/memory.rs`: Handles GPU memory allocation, deallocation, and data transfers
- `src/device.rs`: Manages CUDA device selection and queries device properties
- `src/kernel.rs`: Focuses on compiling, managing, and executing CUDA kernels
- `src/cuda/mod.rs`:Contains low-level bindings and safe wrappers for CUDA API calls
  - `src/cuda/ffi.rs`: Raw FFI bindings to CUDA's C APIs
  - `src/cuda/wrapper.rs`: Safe, idiomatic Rust wrappers around the CUDA FFI bindings
- `src/nvrtc/mod.rs`: Facilitates runtime compilation of CUDA kernels using NVRTC
  - `src/nvrtc/ffi.rs`: Raw FFI bindings to NVRTC's C APIs
  - `src/nvrtc/compiler.rs`: Higher-level functionality for compiling and managing CUDA code at runtime

## Project Outline
To develop the Rust-CUDA interface, the following steps will be taken:

***DONE***
1. **Setup and Environment**:
   - Install Rust and the necessary development tools.
   - Set up the CUDA Toolkit and ensure it is properly configured.
   - Create a new Rust library project for the Rust-CUDA interface.

***IN PROGRESS***
2. **CUDA API Bindings**:
   - Identify the CUDA APIs required for the custom kernels.
   - Create Rust bindings for the necessary CUDA APIs using FFI (Foreign Function Interface).
   - Implement safe Rust wrappers around the CUDA API bindings to provide a more idiomatic Rust interface.

***IN PROGRESS***
3. **Memory Management**:
   - Implement Rust structs and methods for managing GPU memory allocation and deallocation.
   - Provide functions for transferring data between the CPU and GPU.
   - Ensure proper cleanup and resource management.

***IN PROGRESS***
4. **Kernel Launching**:
   - Define Rust functions for launching CUDA kernels.
   - Implement the necessary Rust code to configure and launch kernels with the appropriate parameters.
   - Handle kernel launch errors and provide meaningful error messages.

***IN PROGRESS***
5. **Synchronization and Error Handling**:
   - Implement synchronization primitives to ensure proper execution order and data consistency.
   - Add error handling mechanisms to catch and propagate CUDA errors.
   - Provide Rust-friendly error types and messages.

***IN PROGRESS***
6. **Integration and Testing**:
   - Write Rust code that utilizes the developed Rust-CUDA interface to perform basic operations on the GPU.
   - Compile and run the Rust code to verify the functionality of the interface.
   - Test the interface with different input sizes and configurations to ensure correctness and performance.

***IN PROGRESS***
7. **Incremental Development**:
   - Identify additional features and functionalities needed for the custom MoE kernels.
   - Incrementally extend the Rust-CUDA interface to support these features.
   - Repeat the integration and testing process for each new feature added.

***IN PROGRESS***
8. **Documentation and Examples**:
   - Provide clear and concise documentation for the Rust-CUDA interface, including usage guidelines and API references
   - Include example code snippets and tutorials demonstrating how to use the Rust-CUDA interface for common tasks.
   - Document any limitations, performance considerations, or best practices when using the interface.

## Getting Started

(add instructions on how to build, run tests, and basic usage examples)
