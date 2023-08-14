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

| Function name | Single operation time (µs) | Batch operations(10000) time (µs) | Batch operations(1000000) time (µs)
| :--- | :---: | :---: | :---: |
| ndarray    | 0.073329     | 2.3268     | 2689.5     |  
| rayon    | 4.5121     | 57.551     | 3204.1     |  
| tch    | 1.8041     | 22.562     | 2594.3     |  
| opencl    | 134630     | 218780     | 8794800     |  
| wgsl    | 3.8407     | 5.4426     | 910.17     | 
| cuda    | 4.288     | 5.92     | 51.168     | 
| Rust    | 0.062583 ns    | 23.826     | 2984.0     | 

### Single operation time 

| Function name | Single operation time (µs) | Batch operations(10000) time (µs) | Batch operations(1000000) time (µs)
| :--- | :---: | :---: | :---: |
| ndarray    | 0.073329     | 0.00023268     | 0.0026895     |  
| rayon    | 4.5121     | 0.0057551     | 0.0032041     |  
| tch    | 1.8041     | 0.0022562     | 0.0025943     |  
| opencl    | 134630     | 21.8780     | 8.794800     |  
| wgsl    | 3.8407     | 0.00054426     | 0.00091017     | 
| cuda    | 4.288     | 0.000592     | 0.000051168     | 
| Rust    | 0.062583 ns    | 0.0023826     | 0.0029840     | 

### Sum of vec elements

| Function name | Single operation time (µs) | Batch operations(10000) time (µs) | Batch operations(1000000) time (µs)
| :--- | :---: | :---: | :---: |
| ndarray    | 0.0028401     | 1.0144     | 207.29     |  
| rayon    | 2.2440     | 43.808     | 167.97     |  
| tch    | 9.1645     | 22.990     | 1171.3     |  
| opencl    | 130170     | 228300     | 8222400     |  
| wgsl    | 3.3922     | 4.5717     | 943.88     | 
| cuda    | 4.288     | 18.432     | 662.016    | 
| Rust    | 0.0018699     | 0.44940     | 66.243     | 

### Single operation time 

| Function name | Single operation time (µs) | Batch operations(10000) time (µs) | Batch operations(1000000) time (µs)
| :--- | :---: | :---: | :---: |
| ndarray    | 0.0028401     | 0.00010144     | 0.00020729     |  
| rayon    | 2.2440     | 0.0043808     | 0.00016797     |  
| tch    | 9.1645     | 0.0022990     | 0.0011713     |  
| opencl    | 130170     | 22.8300     | 8.222400     |  
| wgsl    | 3.3922     | 0.00045717     | 0.00094388     | 
| cuda    | 4.288     | 0.0018432     | 0.000662016    | 
| Rust    | 0.0018699     | 0.000044940     | 0.000066243     | 