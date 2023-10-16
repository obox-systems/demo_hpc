use std::vec;

use wgsl_example::*;

fn main() {
  let vec1 = vec![1; 100];
  let vec2 = vec![2; 100];
  let vec3 = vec![4; 100];

  let mut bindings: Bindings = Bindings::initialize_three(vec1, vec2, vec3);


  let gpu = pollster::block_on(GpuConsts::initialaze("src/vec_func.wgsl")).unwrap();
  let bc = BufCoder::initialize(&gpu, &mut bindings, "batch1000_vectorAddition_call", 3);
  let res = pollster::block_on(gpu.run(&bc)).unwrap();

  println!("wgsl result {:?}", res);


	let v = vec![1; 100000000];
  let v2 = vec![2; 100000000];

	let t1 = std::time::Instant::now();
	add_two_vec(&v, &v2, 100000000);
	let rust_time = std::time::Instant::now() - t1;
	println!("rust_time {:?}", rust_time);

  let vec1 = vec![0; 1];
  let vec2 = vec![10; 100];

  let mut bindings: Bindings = Bindings::initialize_two(vec1, vec2);
  let gpu = pollster::block_on(GpuConsts::initialaze("src/sum_func.wgsl")).unwrap();
  let bc = BufCoder::initialize(&gpu, &mut bindings, "vectorSum_call", 2);
  let res = pollster::block_on(gpu.run(&bc)).unwrap();

  println!("Sum vec(wgsl) {:?}", res);

  let v = vec![1; 100000000];
  println!("Sum vec(Rust) {:?}", sum_vec(&v, 100000000));

  let v = vec![1, 2, 3];
  println!("Optimized sum vec(Rust) {:?}", optimized_sum_vec(&v, 0, v.len() - 1));

  let vec1 = vec![0; 1];
  let vec2 = vec![1, 2, 3];

  let mut bindings: Bindings = Bindings::initialize_two(vec1, vec2);
  let gpu = pollster::block_on(GpuConsts::initialaze("src/optimized_sum_func.wgsl")).unwrap();
  let bc = BufCoder::initialize(&gpu, &mut bindings, "optimized_vectorSum_call", 2);
  let res = pollster::block_on(gpu.run(&bc)).unwrap();

  println!("Optimized sum vec(WGSL) {:?}", res);
}
