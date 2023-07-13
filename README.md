# demo_cuda_vs_wgpu

### Vec function time

| Version(1 MB) |  Time(ms) |  
| :--- | :---: |
| Rust(cpu)   | 2.8956    |
| wgsl(gpu)    | 26,4761    |
| cuda(gpu)    | 5,841182    |

| Version(10 MB) |  Time(ms) |  
| :--- | :---: |
| Rust(cpu)   | 30.652    |
| wgsl(gpu)    | 58,7311    |
| cuda(gpu)    | 35,963802    |


### WGSL steps time


| Step(1 MB) |  Time(ms) |  
| :--- | :---: |
| initialaze_gpu   | 793.7504    |
| buffer    | 25.5632    |
| wgsl_execution  | 0.9129    |

| Step(10 MB) |  Time(ms) |  
| :--- | :---: |
| initialaze_gpu   | 802.9455    |
| buffer    | 56.3066    |
| wgsl_execution  | 2.4245    |


### Cuda steps time


| Step(1 MB) |  Time(ms) |  
| :--- | :---: |
| buffer    | 5.78963    |
| wgsl_execution  | 0.051552    |

| Step(10 MB) |  Time(ms) |  
| :--- | :---: |
| buffer    | 35,49113    |
| cuda_execution  | 0.472672    |