#include <iostream>
#include <cuda_runtime.h>

__global__ void vectorAddition(float* a, float* b, float* result, int size) {
    int index = threadIdx.x + blockIdx.x * blockDim.x;
    if (index < size) {
        result[index] = a[index] + b[index];
    }
}

int main() {
    int size = 10000000;
    int byteSize = size * sizeof(float);

    float* hostVectorA = new float[size];
    float* hostVectorB = new float[size];
    float* hostResult = new float[size];

    for (int i = 0; i < size; ++i) {
        hostVectorA[i] = static_cast<float>(i);
        hostVectorB[i] = static_cast<float>(i * 2);
    }

    float* deviceVectorA, * deviceVectorB, * deviceResult;
    cudaMalloc((void**)&deviceVectorA, byteSize);
    cudaMalloc((void**)&deviceVectorB, byteSize);
    cudaMalloc((void**)&deviceResult, byteSize);

    cudaMemcpy(deviceVectorA, hostVectorA, byteSize, cudaMemcpyHostToDevice);
    cudaMemcpy(deviceVectorB, hostVectorB, byteSize, cudaMemcpyHostToDevice);

    int blockSize = 256;
    int gridSize = (size + blockSize - 1) / blockSize;

    cudaEvent_t start, stop;
    cudaEventCreate(&start);
    cudaEventCreate(&stop);

    cudaEventRecord(start, 0);

    vectorAddition<<<gridSize, blockSize>>>(deviceVectorA, deviceVectorB, deviceResult, size);

    cudaEventRecord(stop, 0);
    cudaEventSynchronize(stop);

    float elapsedTime;
    cudaEventElapsedTime(&elapsedTime, start, stop);

    cudaMemcpy(hostResult, deviceResult, byteSize, cudaMemcpyDeviceToHost);

    // for (int i = 0; i < size; ++i) {
    //     std::cout << hostVectorA[i] << " + " << hostVectorB[i] << " = " << hostResult[i] << std::endl;
    // }
    std::cout << "Elapsed Time: " << elapsedTime << " ms" << std::endl;

    delete[] hostVectorA;
    delete[] hostVectorB;
    delete[] hostResult;
    cudaFree(deviceVectorA);
    cudaFree(deviceVectorB);
    cudaFree(deviceResult);

    return 0;
}