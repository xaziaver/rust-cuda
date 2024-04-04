use std::env;

fn main() {
    // directly look for env variable CUDA_LIB_PATH which specifies the CUDA library path
    let cuda_lib_path = match env::var("CUDA_LIB_PATH") {
        Ok(path) => path,
        Err(_) => {
            // CUDA_LIB_PATH not set, provide instructions
            println!("cargo:warning=CUDA_LIB_PATH env variable not set");
            println!("cargo:warning=Please set CUDA_LIB_PATH to your CUDA library install path");
            println!("cargo:warning=For example on Linux: export CUDA_LIB_PATH=/usr/local/cuda/lib64");
            std::process::exit(1);
        }
    };

    // if CUDA_LIB_PATH is found, proceed to instruct cargo to link against CUDA libraries
    println!("cargo:rustc-link-search=native={}", cuda_lib_path);
    println!("cargo:rustc-link-lib=cudart");        // CUDA runtime library for simple management
    println!("cargo:rustc-link-lib=nvrtc");         // NVRTC library for kernels
    // more libraries to link?
}
