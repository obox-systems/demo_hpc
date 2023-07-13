# How to run cuda code

1) Install [CUDA 11.7](https://developer.nvidia.com/cuda-downloads).
2) 
Run the code (Linux)
```bash
$ cd ./cuda && nvcc cuda_realization.cu -o cuda_realization && ./cuda_realization.exe
```
Run the code (Windows)
```bash
$ cd ./cuda ; nvcc cuda_realization.cu -o cuda_realization ; ./cuda_realization.exe
```