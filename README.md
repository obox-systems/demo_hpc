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

## Results of benchmark tests

### Sum of two vectors

| Function name | Single operation time (µs) | Batch operations(10000) time (ms) | Batch operations(1000000) time (s)
| :--- | :---: | :---: | :---: |
| ndarray    | 0.076484    | 3.2910    | 0.32625    |
| rayon    | 4.7141    | 102.83    | 9.7833    |
| tch    | 1.8299    | 45.556    | 4.5838    |
| opencl    | 132800    | 256.47    | 8.7774    |
| wgsl    | 3.5598    | 0.0047120    | 0.0011476    |
| cuda    | -    | -    | -    |
| Rust    | 0.060621    | 21.847    | 2.9436    |

### Sum of vec elements



