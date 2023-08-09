use wgsl_example::*;

fn main() {
  let vec1 = vec![1; 1000000];
  let vec2 = vec![2; 1000000];
  let vec3 = vec![2; 1000000];

  let mut bindings: Bindings = Bindings::initialize_three(vec1, vec2, vec3);

  let t1 = std::time::Instant::now();
  let gpu = pollster::block_on(GpuConsts::initialaze()).unwrap();
  let macro_time = std::time::Instant::now() - t1;

  let t1 = std::time::Instant::now();
  let bc = BufCoder::initialize(&gpu, &mut bindings, "vectorAddition", 3);
  let buffer_time = std::time::Instant::now() - t1;

  let t1 = std::time::Instant::now();
  let _ = pollster::block_on(gpu.run(&bc)).unwrap();
  let wgsl_time = std::time::Instant::now() - t1;

  println!("macro_time {:?}", macro_time);
  println!("buffer_time {:?}", buffer_time);
  println!("wgsl_time {:?}", wgsl_time);

	let v = vec![1; 100000000];
  let v2 = vec![2; 100000000];

	let t1 = std::time::Instant::now();
	add_two_vec(&v, &v2, 100000000);
	let rust_time = std::time::Instant::now() - t1;
	println!("rust_time {:?}", rust_time);

  let vec1 = vec![1; 1000000];
  let vec2 = vec![2; 1000000];

  let mut bindings: Bindings = Bindings::initialize_two(vec1, vec2);
  let gpu = pollster::block_on(GpuConsts::initialaze()).unwrap();
  let bc = BufCoder::initialize(&gpu, &mut bindings, "vectorSum", 2);
  let _ = pollster::block_on(gpu.run(&bc)).unwrap();

  let v = vec![1; 100000000];
  sum_vec(&v, 100000000);
}
