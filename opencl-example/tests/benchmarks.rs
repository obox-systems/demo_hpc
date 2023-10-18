use criterion::{criterion_group, criterion_main, Criterion};

use opencl_example::*;
use rand::Rng;

fn bench_add_vectors_with_opencl(c: &mut Criterion) {
  let mut rng = rand::thread_rng();
  let mut first = vec![0.0; 1000];
	let mut second = vec![0.0; 1000];

  for j in 0..1000 {
    first[j] = rng.gen_range(1.0..=100.0);
    second[j] = rng.gen_range(1.0..=100.0);
  }

  c.bench_function("add_vectors_opencl_one", |b| b.iter(|| add_vectors_with_opencl(&first, &second, 1)));
}

fn batch1000_add_vectors_with_opencl(c: &mut Criterion) {
  let mut rng = rand::thread_rng();
  let mut first = vec![0.0; 1000];
	let mut second = vec![0.0; 1000];

  for j in 0..1000 {
    first[j] = rng.gen_range(1.0..=100.0);
    second[j] = rng.gen_range(1.0..=100.0);
  }

  c.bench_function("batch1000_add_vectors_opencl", |b| b.iter(|| add_vectors_with_opencl(&first, &second, 1000)));
}

fn batch1000000_add_vectors_with_opencl(c: &mut Criterion) {
  let mut rng = rand::thread_rng();
  let mut first = vec![0.0; 1000];
	let mut second = vec![0.0; 1000];

  for j in 0..1000 {
    first[j] = rng.gen_range(1.0..=100.0);
    second[j] = rng.gen_range(1.0..=100.0);
  }

  c.bench_function("batch1000000_add_vectors_opencl", |b| b.iter(|| add_vectors_with_opencl(&first, &second, 1000000)));
}

fn dry_run_add_vectors_with_opencl(c: &mut Criterion) {
  let first = vec![1.0];
	let second = vec![1.0];

  c.bench_function("dry_run_add_vectors_opencl", |b| b.iter(|| add_vectors_with_opencl(&first, &second, 1)));
}

fn bench_sum_vectors_with_opencl(c: &mut Criterion) {
  let mut rng = rand::thread_rng();
	let mut first = vec![0.0; 1000];

  for j in 0..1000 {
    first[j] = rng.gen_range(1.0..=100.0);
  }

  c.bench_function("sum_vectors_opencl_one", |b| b.iter(|| sum_array_opencl(&first, 1)));
}

fn batch1000_sum_vectors_with_opencl(c: &mut Criterion) {
	let mut rng = rand::thread_rng();
	let mut first = vec![0.0; 1000];

  for j in 0..1000 {
    first[j] = rng.gen_range(1.0..=100.0);
  }

  c.bench_function("batch1000_sum_vectors_opencl", |b| b.iter(|| sum_array_opencl(&first, 1000)));
}

fn batch1000000_sum_vectors_with_opencl(c: &mut Criterion) {
	let mut rng = rand::thread_rng();
	let mut first = vec![0.0; 1000];

  for j in 0..1000 {
    first[j] = rng.gen_range(1.0..=100.0);
  }

  c.bench_function("batch1000000_sum_vectors_opencl", |b| b.iter(|| sum_array_opencl(&first, 100000)));
}

fn dry_run_sum_vectors_with_opencl(c: &mut Criterion) {
	let first = vec![1.0];

  c.bench_function("dry_run_sum_vectors_opencl", |b| b.iter(|| sum_array_opencl(&first, 1)));
}

fn bench_optimized_sum_vectors_with_opencl(c: &mut Criterion) {
  let mut rng = rand::thread_rng();
	let mut first = vec![0.0; 1000];

  for j in 0..1000 {
    first[j] = rng.gen_range(1.0..=100.0);
  }

  c.bench_function("optimized_sum_vectors_opencl_one", |b| b.iter(|| optimized_array_opencl(&first, 1)));
}

fn batch1000_optimized_sum_vectors_with_opencl(c: &mut Criterion) {
  let mut rng = rand::thread_rng();
	let mut array1 = vec![0.0; 1000];

  for j in 0..1000 {
    array1[j] = rng.gen_range(1.0..=100.0);
  }

  c.bench_function("batch1000_optimized_sum_vectors_opencl", |b| b.iter(|| optimized_array_opencl(&array1, 1000)));
}

fn batch1000000_optimized_sum_vectors_with_opencl(c: &mut Criterion) {
  let mut rng = rand::thread_rng();
	let mut array1 = vec![0.0; 1000];

  for j in 0..1000 {
    array1[j] = rng.gen_range(1.0..=100.0);
  }

  c.bench_function("batch1000000_optimized_sum_vectors_opencl", |b| b.iter(|| optimized_array_opencl(&array1, 100000)));
}

fn dry_run_optimized_sum_vectors_with_opencl(c: &mut Criterion) {
	let array1 = vec![1.0];

  c.bench_function("dry_run_optimized_sum_vectors_opencl", |b| b.iter(|| optimized_array_opencl(&array1, 1)));
}

criterion_group!{
  name = benches;
  config = Criterion::default().sample_size(10);
  targets = 
    bench_add_vectors_with_opencl, batch1000_add_vectors_with_opencl, batch1000000_add_vectors_with_opencl,
    bench_sum_vectors_with_opencl, batch1000_sum_vectors_with_opencl, batch1000000_sum_vectors_with_opencl,
    bench_optimized_sum_vectors_with_opencl, batch1000_optimized_sum_vectors_with_opencl, batch1000000_optimized_sum_vectors_with_opencl,
    dry_run_add_vectors_with_opencl, dry_run_sum_vectors_with_opencl, dry_run_optimized_sum_vectors_with_opencl
}

criterion_main!(
  benches,
);
