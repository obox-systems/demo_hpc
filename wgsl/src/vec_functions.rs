pub use ndarray::Array1;
use rayon::prelude::*;
pub use tch::Tensor;
use ocl::{flags, Platform, Device, Context, Queue, Buffer};
use std::iter::repeat;

// use coaster as co;
// use coaster_nn as nn;

// use co::prelude::*;
// use nn::*;

pub fn add_arrays_ndarray(a: &Array1<f64>, b: &Array1<f64>) -> Array1<f64> {
  a + b
}

pub fn add_arrays_rayon(a: &[f64], b: &[f64]) -> Vec<f64> {
  a.par_iter().zip(b.par_iter()).map(|(&x, &y)| x + y).collect()
}

pub fn add_arrays_tch(a: &Tensor, b: &Tensor) -> Tensor {
  a + b
}

pub fn add_vectors_with_opencl(a: &[f32], b: &[f32]) -> Vec<f32> {
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

  unsafe {
    kernel.cmd().enq().expect("Failed to enqueue OpenCL kernel command.");
  }

  let mut result = repeat(0.0f32).take(a.len()).collect::<Vec<_>>();
  buffer_result
  .read(&mut result)
  .enq()
  .expect("Failed to read OpenCL buffer to host memory.");

  result
}

// Currently (0.3.0), accel works only on Linux system. Windows support will come in future release (0.3.x or 0.4~).
// ### Vector Add
//
// ```
// use accel::*;
//
// #[kernel]
// unsafe fn add(a: *const f32, b: *const f32, c: *mut f32, n: usize) {
//     let i = accel_core::index();
//     if (i as usize) < n {
//         *c.offset(i) = *a.offset(i) + *b.offset(i);
//     }
// }
//
// fn main() -> error::Result<()> {
//     let device = Device::nth(0)?;
//     let ctx = device.create_context();
//
//     // Allocate memories on GPU
//     let n = 32;
//     let mut a = DeviceMemory::<f32>::zeros(&ctx, n);
//     let mut b = DeviceMemory::<f32>::zeros(&ctx, n);
//     let mut c = DeviceMemory::<f32>::zeros(&ctx, n);
//
//     // Accessible from CPU as usual Rust slice (though this will be slow)
//     for i in 0..n {
//         a[i] = i as f32;
//         b[i] = 2.0 * i as f32;
//     }
//     println!("a = {:?}", a.as_slice());
//     println!("b = {:?}", b.as_slice());
//
//     // Launch kernel synchronously
//     add(&ctx,
//         1 /* grid */,
//         n /* block */,
//         (a.as_ptr(), b.as_ptr(), c.as_mut_ptr(), n)
//     ).expect("Kernel call failed");
//
//     println!("c = {:?}", c.as_slice());
//     Ok(())
// }
