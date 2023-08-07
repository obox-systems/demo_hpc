use ndarray::Array1;
use rand::{distributions::Uniform, Rng};
use ndarray_rand::RandomExt;
use ocl::{flags, Platform, Device, Context, Queue, Buffer};
use std::iter::repeat;

use crate::*;

pub fn add_arrays_ndarray_batch() {
  let mut rng = rand::thread_rng();
  let uniform = Uniform::new(0.0, 100.0);

  for _ in 0..1000000 {
    let a = Array1::random_using(10, uniform, &mut rng);
    let b = Array1::random_using(10, uniform, &mut rng);
    
    let _ = add_arrays_ndarray(&a, &b);
  }
}

pub fn add_arrays_rayon_batch() {
  let mut rng = rand::thread_rng();
  let mut array1 = [0.0; 10];
  let mut array2 = [0.0; 10];

  for _ in 0..1000000 {
    for j in 0..10 {
      array1[j] = rng.gen_range(1.0..=100.0);
      array2[j] = rng.gen_range(1.0..=100.0);
    }
    
    let _ = add_arrays_rayon(&array1, &array2);
  }
}

pub fn add_arrays_tch_batch() {
  let mut rng = rand::thread_rng();
  let mut array1 = [0.0; 10];
  let mut array2 = [0.0; 10];

  for _ in 0..1000000 {
    for j in 0..10 {
      array1[j] = rng.gen_range(1.0..=100.0);
      array2[j] = rng.gen_range(1.0..=100.0);
    }

    let a = Tensor::of_slice(&array1);
    let b = Tensor::of_slice(&array2);
    
    let _ = add_arrays_tch(&a, &b);
  }
}

pub fn add_vectors_with_opencl_batch(a: &[f32], b: &[f32]) -> Vec<f32> {
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

  for _ in 0..1000000 {
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
