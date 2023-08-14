use criterion::{criterion_group, criterion_main, Criterion};
use wgsl_example::*;
use rand::Rng;

fn bench_add_arrays_wgsl(c: &mut Criterion) {
	let vec1 = vec![1; 1];
  let vec2 = vec![2; 1];
  let vec3 = vec![2; 1];

  let mut bindings: Bindings = Bindings::initialize_three(vec1, vec2, vec3);

  let gpu = pollster::block_on(GpuConsts::initialaze()).unwrap();
  let bc = BufCoder::initialize(&gpu, &mut bindings, "vectorAddition", 3);


  c.bench_function("add_arrays_wgsl_one", |b| b.iter(|| pollster::block_on(gpu.run(&bc))));
}

fn batch_add_arrays_wgsl(c: &mut Criterion) {
  let mut rng = rand::thread_rng();
  let mut vec1 = vec![1; 1000000];
  let mut vec2 = vec![2; 1000000];
  let vec3 = vec![2; 1000000];

  for j in 0..1000000 {
    vec1[j] = rng.gen_range(1..=100);
    vec2[j] = rng.gen_range(1..=100);
  }

  let mut bindings: Bindings = Bindings::initialize_three(vec1, vec2, vec3);

  let gpu = pollster::block_on(GpuConsts::initialaze()).unwrap();
  let bc = BufCoder::initialize(&gpu, &mut bindings, "vectorAddition", 3);

  c.bench_function("add_arrays_wgsl_batch", |b| b.iter(|| pollster::block_on(gpu.run(&bc))));
}

fn bench_add_arrays_rust(c: &mut Criterion) {
	let v = vec![1; 1];
  let v2 = vec![2; 1];

  c.bench_function("add_arrays_rust_one", |b| b.iter(|| add_two_vec(&v, &v2, 1)));
}

fn batch_add_arrays_rust(c: &mut Criterion) {
  const SIZE: usize = 1000000;
  let mut rng = rand::thread_rng();
  let mut v = vec![0; SIZE];
  let mut v2 = vec![0; SIZE];

  for j in 0..SIZE {
    v[j] = rng.gen_range(1..=100);
    v2[j] = rng.gen_range(1..=100);
  }

  c.bench_function("add_arrays_rust_batch", |b| b.iter(|| add_two_vec(&v, &v2, SIZE)));
}

fn bench_sum_arrays_wgsl(c: &mut Criterion) {
	let vec1 = vec![1; 1];
  let vec2 = vec![2; 1];

  let mut bindings: Bindings = Bindings::initialize_two(vec1, vec2);

  let gpu = pollster::block_on(GpuConsts::initialaze()).unwrap();
  let bc = BufCoder::initialize(&gpu, &mut bindings, "vectorSum", 2);


  c.bench_function("sum_arrays_wgsl_one", |b| b.iter(|| pollster::block_on(gpu.run(&bc))));
}

fn batch_sum_arrays_wgsl(c: &mut Criterion) {
  let mut rng = rand::thread_rng();
  let mut vec1 = vec![1; 10000];
  let vec3 = vec![2; 10000];

  for j in 0..10000 {
    vec1[j] = rng.gen_range(1..=100);
  }

  let mut bindings: Bindings = Bindings::initialize_two(vec1, vec3);

  let gpu = pollster::block_on(GpuConsts::initialaze()).unwrap();
  let bc = BufCoder::initialize(&gpu, &mut bindings, "vectorSum", 2);

  c.bench_function("sum_arrays_wgsl_batch", |b| b.iter(|| pollster::block_on(gpu.run(&bc))));
}

fn bench_sum_arrays_rust(c: &mut Criterion) {
	let v = vec![1; 1];

  c.bench_function("sum_arrays_rust_one", |b| b.iter(|| sum_vec(&v, 1)));
}

fn batch_sum_arrays_rust(c: &mut Criterion) {
  const SIZE: usize = 10000;
  let mut rng = rand::thread_rng();
  let mut v = vec![0; SIZE];

  for j in 0..SIZE {
    v[j] = rng.gen_range(1..=100);
  }

  c.bench_function("sum_arrays_rust_batch", |b| b.iter(|| sum_vec(&v, SIZE)));
}

criterion_group!{
  name = benches;
  config = Criterion::default().sample_size(10);
  targets = 
    bench_add_arrays_wgsl, batch_add_arrays_wgsl,
    bench_add_arrays_rust, batch_add_arrays_rust,
    bench_sum_arrays_wgsl, batch_sum_arrays_wgsl,
    bench_sum_arrays_rust, batch_sum_arrays_rust

}

criterion_main!(
  benches,
);