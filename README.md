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

| Function name | Single operation time (µs) | Batch operations(10000) time (µs) | Batch(single) operations(10000) time (µs) | Batch operations(1000000) time (µs)| Batch(single) operations(1000000) time (µs)
| :--- | :---: | :---: | :---: | :---: | :---: |
| ndarray    | 7.33e-02     | 2.33e+00     | 2.33e-04     | 2.69e+03     |  2.69e-03     |
| rayon    | 4.51e+00     | 5.76e+01     | 5.76e-03     | 3.20e+03     |  3.20e-03     
| tch    | 1.80e+00     | 2.26e+01     | 2.26e-03     | 2.59e+03     |   2.59e-03     |
| opencl    | 1.35e+05     | 2.19e+05     |  2.19e+01     | 8.79e+06     |  8.79e+00     |
| wgsl    | 3.84e+00     | 5.44e+00     |  5.44e-04     | 9.10e+02     |  9.10e-04     |
| cuda    | 4.29e+00     | 5.92e+00    | 5.92e-04      | 5.12e+01     |  5.12e-05     |
| Rust    | 6.26e-02    | 2.38e+01     | 2.38e-03     | 2.98e+03     | 2.98e-03     | 


### Sum of vec elements

| Function name | Single operation time (µs) | Batch operations(10000) time (µs) | Batch(single) operations(10000) time (µs) | Batch operations(1000000) time (µs) | Batch (single) operations(1000000) time (µs)
| :--- | :---: | :---: | :---: | :---: | :---: |
| ndarray    | 2.84e-03     | 1.01e+00    | 1.01e-04     | 2.07e+02     |  2.07e-04     |
| rayon    | 2.24e+00     | 4.38e+01  |  4.38e-03      | 1.68e+02     |  1.68e-04     | 
| tch    | 9.16e+00     | 2.30e+01   |  2.30e-03     | 1.17e+03     |  1.17e-03     |
| opencl    | 1.30e+05     | 2.28e+05    | 2.28e+01     | 8.22e+06     |  8.22e+00     |
| wgsl    | 3.39e+00     | 4.57e+00    | 4.57e-04     | 9.44e+02     | 9.44e-04     | 
| cuda    | 4.29e+00     | 1.84e+01    | 1.84e-03     | 6.62e+02    | 6.62e-04    | 
| Rust    | 1.87e-03     | 4.49e-01    | 4.49e-05      | 6.62e+01     | 6.62e-05     |

### Sum of vec elements(optimized)

| Function name | Single operation time (µs) | Batch operations(10000) time (µs) | Batch(single) operations(10000) time (µs) | Batch operations(1000000) time (µs) | Batch (single) operations(1000000) time (µs)
| :--- | :---: | :---: | :---: | :---: | :---: |
| ndarray    | 6.54e-03     | 2.80e+01    | 2.79e-03     | 2.52e+03     |  2.52e-03     |
| rayon    | 7.20e-03      | 2.94e+01  |  2.94e-03      | 2.62e+03     |  2.62e-03     | 
| tch    | 7.02e+00     | 2.38e+04   |  2.38e+00     | 2.58e+06     |  2.58e+00     |
| opencl    | 1.02e+02     | 1.06e+05    | 1.06e+01     | 2.92e+05      |  2.92e-01     |
| wgsl    | 7.24e-01     | 5.00e+03    | 5.00e-01     | 2.08e+05     | 2.08e-01     | 
| cuda    | 2.01e-01     | 2.46e+03    | 2.46e-01     | 2.21e+05    | 2.21e-01    | 
| Rust    | 4.59e-03     | 1.79e+01    | 1.79e-03      | 1.66e+03     | 1.66e-03     |