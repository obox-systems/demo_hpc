use criterion::{criterion_group, criterion_main, Criterion};
use wgsl_example::*;
use rand::Rng;

fn bench_add_arrays_wgsl(c: &mut Criterion) {
  let mut rng = rand::thread_rng();
	let vec1 = vec![0; 1000];
  let mut vec2 = vec![0; 1000];
  let mut vec3 = vec![0; 1000];

  for j in 0..1000 {
    vec3[j] = rng.gen_range(1..=100);
    vec2[j] = rng.gen_range(1..=100);
  }

  let mut bindings: Bindings = Bindings::initialize_three(vec1, vec2, vec3);

  let gpu = pollster::block_on(GpuConsts::initialaze("src/vec_func.wgsl")).unwrap();
  let bc = BufCoder::initialize(&gpu, &mut bindings, "vectorAddition_call", 3);


  c.bench_function("add_arrays_wgsl_one", |b| b.iter(|| pollster::block_on(gpu.run(&bc))));
}

fn batch1000_add_arrays_wgsl(c: &mut Criterion) {
  let mut rng = rand::thread_rng();
	let vec1 = vec![0; 1000];
  let mut vec2 = vec![0; 1000];
  let mut vec3 = vec![0; 1000];

  for j in 0..1000 {
    vec3[j] = rng.gen_range(1..=100);
    vec2[j] = rng.gen_range(1..=100);
  }

  let mut bindings: Bindings = Bindings::initialize_three(vec1, vec2, vec3);

  let gpu = pollster::block_on(GpuConsts::initialaze("src/vec_func.wgsl")).unwrap();
  let bc = BufCoder::initialize(&gpu, &mut bindings, "batch1000_vectorAddition_call", 3);


  c.bench_function("batch1000_add_arrays_wgsl", |b| b.iter(|| pollster::block_on(gpu.run(&bc))));
}

fn batch100000_add_arrays_wgsl(c: &mut Criterion) {
  let mut rng = rand::thread_rng();
	let vec1 = vec![0; 1000];
  let mut vec2 = vec![0; 1000];
  let mut vec3 = vec![0; 1000];

  for j in 0..1000 {
    vec3[j] = rng.gen_range(1..=100);
    vec2[j] = rng.gen_range(1..=100);
  }

  let mut bindings: Bindings = Bindings::initialize_three(vec1, vec2, vec3);

  let gpu = pollster::block_on(GpuConsts::initialaze("src/vec_func.wgsl")).unwrap();
  let bc = BufCoder::initialize(&gpu, &mut bindings, "batch100000_vectorAddition_call", 3);


  c.bench_function("batch100000_add_arrays_wgsl", |b| b.iter(|| pollster::block_on(gpu.run(&bc))));
}

fn dry_run_add_arrays_wgsl(c: &mut Criterion) {
	let vec1 = vec![0; 1];
  let vec2 = vec![0; 1];
  let vec3 = vec![0; 1];

  let mut bindings: Bindings = Bindings::initialize_three(vec1, vec2, vec3);

  let gpu = pollster::block_on(GpuConsts::initialaze("src/vec_func.wgsl")).unwrap();
  let bc = BufCoder::initialize(&gpu, &mut bindings, "vectorAddition_call", 3);

  c.bench_function("dry_run_add_arrays_wgsl", |b| b.iter(|| pollster::block_on(gpu.run(&bc))));
}

fn bench_sum_arrays_wgsl(c: &mut Criterion) {
  let mut rng = rand::thread_rng();
	let vec1 = vec![0; 1];
  let mut vec2 = vec![0; 1000];

  for j in 0..1000 {
    vec2[j] = rng.gen_range(1..=100);
  }

  let mut bindings: Bindings = Bindings::initialize_two(vec1, vec2);

  let gpu = pollster::block_on(GpuConsts::initialaze("src/sum_func.wgsl")).unwrap();
  let bc = BufCoder::initialize(&gpu, &mut bindings, "vectorSum_call", 2);

  c.bench_function("sum_arrays_wgsl_one", |b| b.iter(|| pollster::block_on(gpu.run(&bc))));
}

fn batch1000_sum_arrays_wgsl(c: &mut Criterion) {
  let mut rng = rand::thread_rng();
	let vec1 = vec![0; 1];
  let mut vec2 = vec![0; 1000];

  for j in 0..1000 {
    vec2[j] = rng.gen_range(1..=100);
  }

  let mut bindings: Bindings = Bindings::initialize_two(vec1, vec2);

  let gpu = pollster::block_on(GpuConsts::initialaze("src/sum_func.wgsl")).unwrap();
  let bc = BufCoder::initialize(&gpu, &mut bindings, "batch1000_vectorSum_call", 2);

  c.bench_function("batch1000_sum_arrays_wgsl", |b| b.iter(|| pollster::block_on(gpu.run(&bc))));
}

fn batch100000_sum_arrays_wgsl(c: &mut Criterion) {
  let mut rng = rand::thread_rng();
	let vec1 = vec![0; 1];
  let mut vec2 = vec![0; 1000];

  for j in 0..1000 {
    vec2[j] = rng.gen_range(1..=100);
  }

  let mut bindings: Bindings = Bindings::initialize_two(vec1, vec2);

  let gpu = pollster::block_on(GpuConsts::initialaze("src/sum_func.wgsl")).unwrap();
  let bc = BufCoder::initialize(&gpu, &mut bindings, "batch100000_vectorSum_call", 2);

  c.bench_function("batch100000_sum_arrays_wgsl", |b| b.iter(|| pollster::block_on(gpu.run(&bc))));
}

fn dry_run_sum_arrays_wgsl(c: &mut Criterion) {
	let vec1 = vec![0; 1];
  let vec2 = vec![0; 1000];

  let mut bindings: Bindings = Bindings::initialize_two(vec1, vec2);

  let gpu = pollster::block_on(GpuConsts::initialaze("src/sum_func.wgsl")).unwrap();
  let bc = BufCoder::initialize(&gpu, &mut bindings, "vectorSum_call", 2);


  c.bench_function("dry_run_sum_arrays_wgsl", |b| b.iter(|| pollster::block_on(gpu.run(&bc))));
}

fn bench_optimized_sum_arrays_wgsl(c: &mut Criterion) {
	let mut rng = rand::thread_rng();
	let vec1 = vec![0; 1];
  let mut vec2 = vec![0; 1000];

  for j in 0..1000 {
    vec2[j] = rng.gen_range(1..=100);
  }

  let mut bindings: Bindings = Bindings::initialize_two(vec1, vec2);

  let gpu = pollster::block_on(GpuConsts::initialaze("src/optimized_sum_func.wgsl")).unwrap();
  let bc = BufCoder::initialize(&gpu, &mut bindings, "optimized_vectorSum_call", 2);

  c.bench_function("bench_optimized_sum_arrays_wgsl", |b| b.iter(|| pollster::block_on(gpu.run(&bc))));
}

fn batch1000_optimized_sum_arrays_wgsl(c: &mut Criterion) {
	let mut rng = rand::thread_rng();
	let vec1 = vec![0; 1];
  let mut vec2 = vec![0; 1000];

  for j in 0..1000 {
    vec2[j] = rng.gen_range(1..=100);
  }

  let mut bindings: Bindings = Bindings::initialize_two(vec1, vec2);

  let gpu = pollster::block_on(GpuConsts::initialaze("src/optimized_sum_func.wgsl")).unwrap();
  let bc = BufCoder::initialize(&gpu, &mut bindings, "batch1000_optimized_vectorSum_call", 2);

  c.bench_function("batch1000_optimized_sum_arrays_wgsl", |b| b.iter(|| pollster::block_on(gpu.run(&bc))));
}

fn batch100000_optimized_sum_arrays_wgsl(c: &mut Criterion) {
	let mut rng = rand::thread_rng();
	let vec1 = vec![0; 1];
  let mut vec2 = vec![0; 1000];

  for j in 0..1000 {
    vec2[j] = rng.gen_range(1..=100);
  }

  let mut bindings: Bindings = Bindings::initialize_two(vec1, vec2);

  let gpu = pollster::block_on(GpuConsts::initialaze("src/optimized_sum_func.wgsl")).unwrap();
  let bc = BufCoder::initialize(&gpu, &mut bindings, "batch100000_optimized_vectorSum_call", 2);

  c.bench_function("batch100000_optimized_sum_arrays_wgsl", |b| b.iter(|| pollster::block_on(gpu.run(&bc))));
}

fn dry_run_optimized_sum_arrays_wgsl(c: &mut Criterion) {
	let vec1 = vec![0; 1];
  let vec2 = vec![0; 1000];

  let mut bindings: Bindings = Bindings::initialize_two(vec1, vec2);

  let gpu = pollster::block_on(GpuConsts::initialaze("src/optimized_sum_func.wgsl")).unwrap();
  let bc = BufCoder::initialize(&gpu, &mut bindings, "optimized_vectorSum_call", 2);

  c.bench_function("dry_run_optimized_sum_arrays_wgsl", |b| b.iter(|| pollster::block_on(gpu.run(&bc))));
}

fn bench_add_arrays_rust(c: &mut Criterion) {
  let mut rng = rand::thread_rng();
	let mut vec1 = vec![0; 1000];
  let mut vec2 = vec![0; 1000];

  for j in 0..1000 {
    vec1[j] = rng.gen_range(1..=100);
    vec2[j] = rng.gen_range(1..=100);
  }

  c.bench_function("add_arrays_rust_one", |b| b.iter(|| add_two_vec(&vec1, &vec2, 1000)));
}

fn batch1000_add_arrays_rust(c: &mut Criterion) {
  let mut rng = rand::thread_rng();
  let mut v = vec![0; 1000];
  let mut v2 = vec![0; 1000];

  for j in 0..1000 {
    v[j] = rng.gen_range(1..=100);
    v2[j] = rng.gen_range(1..=100);
  }

  c.bench_function("batch1000_add_arrays_rust", |b| b.iter(|| batch_add_two_vec(&v, &v2, 1000, 1000)));
}

fn batch100000_add_arrays_rust(c: &mut Criterion) {
  let mut rng = rand::thread_rng();
  let mut v = vec![0; 1000];
  let mut v2 = vec![0; 1000];

  for j in 0..1000 {
    v[j] = rng.gen_range(1..=100);
    v2[j] = rng.gen_range(1..=100);
  }

  c.bench_function("batch100000_add_arrays_rust", |b| b.iter(|| batch_add_two_vec(&v, &v2, 1000, 100000)));
}

fn dry_run_add_arrays_rust(c: &mut Criterion) {
  let v = Vec::new();
  let v2 = Vec::new();

  c.bench_function("dry_run_add_arrays_rust", |b| b.iter(|| add_two_vec(&v, &v2, 0)));
}

fn bench_sum_arrays_rust(c: &mut Criterion) {
	let mut rng = rand::thread_rng();
  let mut v = vec![0; 1000];

  for j in 0..1000 {
    v[j] = rng.gen_range(1..=100);
  }

  c.bench_function("sum_arrays_rust_one", |b| b.iter(|| sum_vec(&v, 1000)));
}

fn batch1000_sum_arrays_rust(c: &mut Criterion) {
  let mut rng = rand::thread_rng();
  let mut v = vec![0; 1000];

  for j in 0..1000 {
    v[j] = rng.gen_range(1..=100);
  }

  c.bench_function("batch1000_sum_arrays_rust", |b| b.iter(|| batch_sum_vec(&v, 1000, 1000)));
}

fn batch100000_sum_arrays_rust(c: &mut Criterion) {
  let mut rng = rand::thread_rng();
  let mut v = vec![0; 1000];

  for j in 0..1000 {
    v[j] = rng.gen_range(1..=100);
  }

  c.bench_function("batch100000_sum_arrays_rust", |b| b.iter(|| batch_sum_vec(&v, 1000, 100000)));
}

fn dry_run_sum_arrays_rust(c: &mut Criterion) {
  let v = Vec::new();

  c.bench_function("dry_run_sum_arrays_rust", |b| b.iter(|| sum_vec(&v, 0)));
}

fn bench_optimized_sum_arrays_rust(c: &mut Criterion) {
	let mut rng = rand::thread_rng();
  let mut v = vec![0; 1000];

  for j in 0..1000 {
    v[j] = rng.gen_range(1..=100);
  }

  c.bench_function("optimized_sum_arrays_rust_one", |b| b.iter(|| optimized_sum_vec(&v, 0, v.len() - 1)));
}

fn batch1000_optimized_sum_arrays_rust(c: &mut Criterion) {
  let mut rng = rand::thread_rng();
  let mut v = vec![0; 1000];

  for j in 0..1000 {
    v[j] = rng.gen_range(1..=100);
  }

  c.bench_function("batch1000_optimized_sum_arrays_rust", |b| b.iter(|| batch_optimized_sum_vec(&v, 0, v.len() - 1, 1000)));
}

fn batch100000_optimized_sum_arrays_rust(c: &mut Criterion) {
  let mut rng = rand::thread_rng();
  let mut v = vec![0; 1000];

  for j in 0..1000 {
    v[j] = rng.gen_range(1..=100);
  }

  c.bench_function("batch100000_optimized_sum_arrays_rust", |b| b.iter(|| batch_optimized_sum_vec(&v, 0, v.len() - 1, 100000)));
}

fn dry_run_optimized_sum_arrays_rust(c: &mut Criterion) {
  let v = vec![0; 1];

  c.bench_function("dry_run_optimized_sum_arrays_rust", |b| b.iter(|| optimized_sum_vec(&v, 0, v.len() - 1)));
}

criterion_group!{
  name = wgsl;
  config = Criterion::default().sample_size(10);
  targets = 
    bench_add_arrays_wgsl, batch1000_add_arrays_wgsl, batch100000_add_arrays_wgsl, dry_run_add_arrays_wgsl,
    bench_sum_arrays_wgsl, batch1000_sum_arrays_wgsl, batch100000_sum_arrays_wgsl, dry_run_sum_arrays_wgsl,
    bench_optimized_sum_arrays_wgsl, batch1000_optimized_sum_arrays_wgsl, batch100000_optimized_sum_arrays_wgsl, dry_run_optimized_sum_arrays_wgsl
}

criterion_group!{
  name = rust;
  config = Criterion::default().sample_size(10);
  targets = 
    bench_add_arrays_rust, batch1000_add_arrays_rust, batch100000_add_arrays_rust, dry_run_add_arrays_rust,
    bench_sum_arrays_rust, batch1000_sum_arrays_rust, batch100000_sum_arrays_rust, dry_run_sum_arrays_rust,
    bench_optimized_sum_arrays_rust, batch1000_optimized_sum_arrays_rust, batch100000_optimized_sum_arrays_rust, dry_run_optimized_sum_arrays_rust
}

criterion_main!(
  wgsl,
  rust
);