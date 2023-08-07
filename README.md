# demo_cuda_vs_wgpu

### Vec function time

| Version(1 MB) |  Time(ms) |  
| :--- | :---: |
| Rust(cpu)   | 2.8956    |
| wgsl(gpu)    | 3.5801    |
| cuda(gpu)    | 5,841182    |

| Version(10 MB) |  Time(ms) |  
| :--- | :---: |
| Rust(cpu)   | 30.652    |
| wgsl(gpu)    | 29.1722    |
| cuda(gpu)    | 35,963802    |


### WGSL steps time


| Step(1 MB) |  Time(ms) |  
| :--- | :---: |
| initialaze_gpu   | 575.7924    |
| buffer    | 11.3628    |
| wgsl_execution  | 0.034371    |

| Step(10 MB) |  Time(ms) |  
| :--- | :---: |
| initialaze_gpu   | 545.835    |
| buffer    | 49.9293    |
| wgsl_execution  | 0.281717    |


### Cuda steps time


| Step(1 MB) |  Time(ms) |  
| :--- | :---: |
| buffer    | 5.78963    |
| wgsl_execution  | 0.051552    |

| Step(10 MB) |  Time(ms) |  
| :--- | :---: |
| buffer    | 35,49113    |
| cuda_execution  | 0.472672    |



# HPC performance comparison

## Creates used in this project

1. [ndarray](https://crates.io/crates/ndarray)

It is a library for working with N-dimensional arrays that provides extensive array operations and optimized data access. It supports parallel operations and can be useful for HPC.

2. [rayon](https://crates.io/crates/rayon)

It allows you to conveniently perform parallel computations based on a declarative approach, which simplifies work with multi-core systems.

3. [tch](https://crates.io/crates/tch)

PyTorch is a library for working with neural networks and deep learning. Thanks to tch-rs, you can use PyTorch functionality together with the Rust language and work efficiently with large amounts of data.

4. [ocl](https://crates.io/crates/ocl)

Rust implementation of the OpenCL API. Provides: a simple and intuitive interface to OpenCL devices, the full functionality and power of the OpenCL API, an absolute minimum of boilerplate, zero or virtually zero performance overhead, thread-safe and automatic management of API pointers and resources.

## Results of benchmark tests

| Function name | Single operation time (µs) | Batch operations(10000) time (ms) | Batch operations(1000000) time (s)
| :--- | :---: | :---: | :---: |
| ndarray    | 0.076484    | 3.2910    | 0.32625    |
| rayon    | 4.7141    | 102.83    | 9.7833    |
| tch    | 1.8299    | 45.556    | 4.5838    |
| opencl    | 132800    | 256.47    | 8.7774    |