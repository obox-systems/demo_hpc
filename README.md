# demo_hpc

## Creates used in this project

1. [ndarray](https://crates.io/crates/ndarray)

It is a library for working with N-dimensional arrays that provides extensive array operations and optimized data access. It supports parallel operations and can be useful for HPC.

2. [rayon](https://crates.io/crates/rayon)

It allows you to conveniently perform parallel computations based on a declarative approach, which simplifies work with multi-core systems.

3. [tch](https://crates.io/crates/tch)

PyTorch is a library for working with neural networks and deep learning. Thanks to tch-rs, you can use PyTorch functionality together with the Rust language and work efficiently with large amounts of data.

4. [ocl](https://crates.io/crates/ocl)

Rust implementation of the OpenCL API. Provides: a simple and intuitive interface to OpenCL devices, the full functionality and power of the OpenCL API, an absolute minimum of boilerplate, zero or virtually zero performance overhead, thread-safe and automatic management of API pointers and resources.

5. [wgsl](https://www.w3.org/TR/WGSL/)

WebGPU Shading Language (WGSL) is the shader language for WebGPU. That is, an application using the WebGPU API uses WGSL to express the programs, known as shaders, that run on the GPU.

## How to run

cargo run --bin cli .benchmarks.run crate:ndarray

## Results of benchmark tests

### Sum of two vectors

| Function name | Device | Dry Run time (µs) | Single operation time (µs) | Batch operations(1.00e+03) time (µs) | Batch operations(1.00e+06) time (µs)
| :--- | :---: | :---: | :---: | :---: | :---: |
| ndarray    | CPU | 3.32e-02 | 2.80e-01     | 3.00e+02     |  3.41e+05     |
| rayon    | CPU | - | 4.51e+00     | 5.76e-03     |  3.20e-03      |
| tch    | CPU | - | 1.80e+00     | 2.26e-03     |   2.59e-03     |
| opencl    | GPU | - | 1.35e+05     |  2.19e+01     |  8.79e+00     |
| wgsl    | GPU | 4.40e+00 | 4.46e+00     |  4.50e+00     |  4.53e+00     |
| cuda    | GPU | - | 4.29e+00     | 5.92e-04      |  5.12e-05     |
| Rust    | CPU | 1.15e-02 | 2.05e+00    | 2.04e+03     | 2.07e+05     | 


### Sum of vec elements

| Function name | Device | Dry Run time (µs) | Single operation time (µs) | Batch operations(1.00e+03) time (µs) | Batch operations(1.00e+06) time (µs)
| :--- | :---: | :---: | :---: | :---: | :---: |
| ndarray    | CPU | 2.68e-03 | 9.93e-02     | 9.88e+01     |  9.93e+04     |
| rayon    | CPU | - | 2.24e+00     |  4.38e-03      |  1.68e-04     | 
| tch    | CPU | - | 9.16e+00     |  2.30e-03     |  1.17e-03     |
| opencl    | GPU | - | 1.30e+05     | 2.28e+01     |  8.22e+00     |
| wgsl    | GPU | 4.39e+00 | 4.48e+00     | 4.51e+00     | 7.34e+00     | 
| cuda    | GPU | - | 4.29e+00     | 1.84e-03     | 6.62e-04    | 
| Rust    | CPU | 1.74e-03 | 5.95e-02     | 1.48e+00      | 1.79e+00     |

### Sum of vec elements(optimized)

| Function name | Device | Dry Run time (µs) | Single operation time (µs) | Batch operations(1.00e+03) time (µs) | Batch operations(1.00e+06) time (µs)
| :--- | :---: | :---: | :---: | :---: | :---: |
| ndarray    | CPU | 2.42e-03 | 2.44e+00     | 2.53e+03     |  2.46e+06     |
| rayon    | CPU | - | 7.20e-03      |  2.94e-03      |  2.62e-03     | 
| tch    | CPU | - | 7.02e+00     |  2.38e+00     |  2.58e+00     |
| opencl    | GPU | - | 1.02e+02     | 1.06e+01     |  2.92e-01     |
| wgsl    | GPU | 4.39e+00 | 4.40e+00     | 4.38e+00     | 4.50e+00     | 
| cuda    | GPU | - | 2.01e-01     | 2.46e-01     | 2.21e-01    | 
| Rust    | CPU | 2.31e-03 | 1.89e+00     | 1.73e+03      | 1.71e+05     |