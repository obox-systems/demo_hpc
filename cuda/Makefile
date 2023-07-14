OS := $(shell uname)

check_cuda:
	@nvcc --version >/dev/null 2>&1 || { echo "CUDA is not installed."; exit 1; }

build:
    ifdef COMSPEC
        nvcc cuda_realization.cu -o cuda_realization
        .\cuda_realization.exe
    else ifeq ($(OS), Linux)
		nvcc cuda_realization.cu -o cuda_realization && ./cuda_realization
    else
        echo "Unsupported operating system: $(OS)"
        exit 1
    endif