#include <iostream>
#include <cuda_runtime.h>

__global__ void vectorAddition(float* a, float* b, float* result, int size) {
    int index = threadIdx.x + blockIdx.x * blockDim.x;
    if (index < size) {
        result[index] = a[index] + b[index];
    }
}

__global__ void vectorSum(float* a, int size, float* sum) {
    int tid = blockIdx.x * blockDim.x + threadIdx.x;

    __shared__ float sharedSum;

    if (tid < size) {
        sharedSum = 0;

        atomicAdd(&sharedSum, a[tid]);

        __syncthreads();

        if (threadIdx.x == 0) {
            atomicAdd(sum, sharedSum);
        }
    }
}

__device__ double optimized_array_cuda(const double* arr, int start, int end) {
    if (end == start) {
        return arr[end];
    }
    if (end - start == 1) {
        return arr[start] + arr[end];
    }
    else {
        int mid = (end - start) / 2 + start;
        double sum1, sum2;
        sum1 = optimized_array_cuda(arr, start, mid);
        sum2 = optimized_array_cuda(arr, mid + 1, end);
        return sum1 + sum2;
    }
}

__global__ void kernel(double* result, const double* arr, int start, int end) {
    if (threadIdx.x == 0 && blockIdx.x == 0) {
        *result = optimized_array_cuda(arr, start, end);
    }
}

// int main() {
//     int size = 1000000;
//     int byteSize = size * sizeof(float);

//     float* hostVectorA = new float[size];
//     float* hostVectorB = new float[size];
//     float* hostResult = new float[size];

//     for (int i = 0; i < size; ++i) {
//         hostVectorA[i] = static_cast<float>(i);
//         hostVectorB[i] = static_cast<float>(i * 2);
//     }

//     float* deviceVectorA, * deviceVectorB, * deviceResult;
//     cudaMalloc((void**)&deviceVectorA, byteSize);
//     cudaMalloc((void**)&deviceVectorB, byteSize);
//     cudaMalloc((void**)&deviceResult, byteSize);

//     cudaMemcpy(deviceVectorA, hostVectorA, byteSize, cudaMemcpyHostToDevice);
//     cudaMemcpy(deviceVectorB, hostVectorB, byteSize, cudaMemcpyHostToDevice);

//     float* deviceSum;
//     cudaMalloc((void**)&deviceSum, sizeof(float));
//     cudaMemset(deviceSum, 0, sizeof(float));

//     int blockSize = 256;
//     int gridSize = (size + blockSize - 1) / blockSize;

//     cudaEvent_t start, stop;
//     cudaEventCreate(&start);
//     cudaEventCreate(&stop);

//     cudaEventRecord(start, 0);

//     // vectorAddition<<<gridSize, blockSize>>>(deviceVectorA, deviceVectorB, deviceResult, size);
//     vectorSum<<<gridSize, blockSize>>>(deviceVectorA, size, deviceSum);

//     cudaEventRecord(stop, 0);
//     cudaEventSynchronize(stop);

//     float elapsedTime;
//     cudaEventElapsedTime(&elapsedTime, start, stop);

//     cudaMemcpy(hostResult, deviceResult, byteSize, cudaMemcpyDeviceToHost);

//     // for (int i = 0; i < size; ++i) {
//     //     std::cout << hostVectorA[i] << " + " << hostVectorB[i] << " = " << hostResult[i] << std::endl;
//     // }

//     float hostSum;
//     cudaMemcpy(&hostSum, deviceSum, sizeof(float), cudaMemcpyDeviceToHost);

//     std::cout << "Sum of vector elements: " << hostSum << std::endl;
//     std::cout << "Elapsed Time: " << elapsedTime << " ms" << std::endl;

//     delete[] hostVectorA;
//     delete[] hostVectorB;
//     delete[] hostResult;
//     cudaFree(deviceVectorA);
//     cudaFree(deviceVectorB);
//     cudaFree(deviceResult);

//     return 0;
// }

int main() {
    const int N = 1000000;
    double h_arr[N];

    for (int i = 0; i < N; ++i) {
        h_arr[i] = static_cast<float>(i);
    }

    double h_result;

    double* d_arr;
    double* d_result;

    cudaMalloc((void**)&d_arr, N * sizeof(double));
    cudaMalloc((void**)&d_result, sizeof(double));

    cudaMemcpy(d_arr, h_arr, N * sizeof(double), cudaMemcpyHostToDevice);

    cudaEvent_t start, stop;
    cudaEventCreate(&start);
    cudaEventCreate(&stop);

    cudaEventRecord(start, 0);

    kernel<<<1, 1>>>(d_result, d_arr, 0, N - 1);

    cudaEventRecord(stop, 0);
    cudaEventSynchronize(stop);

    float elapsedTime;
    cudaEventElapsedTime(&elapsedTime, start, stop);

    cudaMemcpy(&h_result, d_result, sizeof(double), cudaMemcpyDeviceToHost);

    std::cout << "Sum: " << h_result << std::endl;
    std::cout << "Elapsed Time: " << elapsedTime << " ms" << std::endl;

    cudaFree(d_arr);
    cudaFree(d_result);

    return 0;
}