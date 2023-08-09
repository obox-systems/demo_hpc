use ocl::{flags, Platform, Device, Context, Queue, Buffer, Program, Kernel};
use std::iter::repeat;

pub fn add_vectors_with_opencl(a: &[f32], b: &[f32], batches: i32) -> Vec<f32> {
	let platform = Platform::default();
	let device = Device::first(platform).expect("No OpenCL devices found.");

	let context = Context::builder()
	.platform(platform)
	.devices(device.clone())
	.build()
	.expect("Failed to create OpenCL context.");
	let queue = Queue::new(&context, device, None).expect("Failed to create OpenCL queue.");

	let buffer_a = Buffer::<f32>::builder()
	.queue(queue.clone())
	.flags(flags::MEM_READ_ONLY)
	.len(a.len())
	.copy_host_slice(a)
	.build()
	.expect("Failed to create OpenCL buffer for input vector A.");
	let buffer_b = Buffer::<f32>::builder()
	.queue(queue.clone())
	.flags(flags::MEM_READ_ONLY)
	.len(b.len())
	.copy_host_slice(b)
	.build()
	.expect("Failed to create OpenCL buffer for input vector B.");
	let buffer_result = Buffer::<f32>::builder()
	.queue(queue.clone())
	.flags(flags::MEM_WRITE_ONLY)
	.len(a.len())
	.fill_val(0.0f32)
	.build()
	.expect("Failed to create OpenCL buffer for output vector.");

	let src = r#"
			__kernel void add(__global const float* a,
												__global const float* b,
												__global float* result) {
					int i = get_global_id(0);
					result[i] = a[i] + b[i];
			}
	"#;

	let program = ocl::Program::builder()
	.src(src)
	.devices(device)
	.build(&context)
	.expect("Failed to build OpenCL program.");

	let kernel = ocl::Kernel::builder()
	.program(&program)
	.name("add")
	.queue(queue.clone())
	.global_work_size(a.len())
	.arg(&buffer_a)
	.arg(&buffer_b)
	.arg(&buffer_result)
	.build()
	.expect("Failed to create OpenCL kernel.");

	for _ in 0..batches {
		unsafe {
			kernel.cmd().enq().expect("Failed to enqueue OpenCL kernel command.");
		}
	}

	let mut result = repeat(0.0f32).take(a.len()).collect::<Vec<_>>();
	buffer_result
	.read(&mut result)
	.enq()
	.expect("Failed to read OpenCL buffer to host memory.");

	result
}

pub fn sum_array_opencl(arr: &[f32], batches: i32) -> f32 {
	let platform = Platform::default();
	let device = Device::first(platform).expect("No OpenCL devices found.");
	let context = Context::builder().platform(platform).devices(device.clone()).build().unwrap();
	let queue = Queue::new(&context, device, None).expect("Couldn't create OpenCL queue.");

	let src = "
			__kernel void sum_array(__global const float *arr, __global float *result) {
					int gid = get_global_id(0);
					result[gid] = arr[gid];
			}
	";

	let program = Program::builder().devices(device).src(src).build(&context).unwrap();
	// program.build_info(&context).unwrap();

	let buffer_arr = Buffer::<f32>::builder().queue(queue.clone()).len(arr.len()).copy_host_slice(arr).build().unwrap();
	let buffer_result = Buffer::<f32>::builder().queue(queue.clone()).len(arr.len()).build().unwrap();

	let kernel = Kernel::builder().name("sum_array").program(&program).queue(queue.clone()).global_work_size(arr.len()).arg(&buffer_arr).arg(&buffer_result).build().unwrap();
	for _ in 0..batches {
		unsafe {
				kernel.enq().unwrap();
		}
	}

	let mut result = vec![0.0; arr.len()];
	buffer_result.read(&mut result).enq().unwrap();

	result.iter().sum()
}