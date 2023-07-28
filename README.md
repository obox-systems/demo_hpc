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