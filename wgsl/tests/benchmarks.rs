use criterion::{criterion_group, criterion_main, Criterion};
use ndarray::Array1;

use wgsl_and_rust_realization::*;

fn bench_add_arrays_ndarray(c: &mut Criterion) {
  let first = Array1::from(vec![1.0, 2.0, 3.0]);
	let second = Array1::from(vec![4.0, 5.0, 6.0]);

  c.bench_function("add_arrays_ndarray_one", |b| b.iter(|| add_arrays_ndarray(&first, &second)));
}

fn bench_add_arrays_rayon(c: &mut Criterion) {
	let first = vec![1.0, 2.0, 3.0];
	let second = vec![4.0, 5.0, 6.0];

  c.bench_function("add_arrays_rayon_one", |b| b.iter(|| add_arrays_rayon(&first, &second)));
}

fn bench_add_arrays_tch(c: &mut Criterion) {
	let first = Tensor::of_slice(&[1.0, 2.0, 3.0]);
	let second = Tensor::of_slice(&[4.0, 5.0, 6.0]);

  c.bench_function("add_arrays_tch_one", |b| b.iter(|| add_arrays_tch(&first, &second)));
}

fn bench_add_vectors_with_opencl(c: &mut Criterion) {
	let first = vec![1.0, 2.0, 3.0];
	let second = vec![4.0, 5.0, 6.0];

  c.bench_function("add_vectors_with_opencl_one", |b| b.iter(|| add_vectors_with_opencl(&first, &second)));
}

criterion_group!{
  name = benches;
  config = Criterion::default().sample_size(10);
  targets = 
    bench_add_arrays_ndarray, bench_add_arrays_rayon, 
    bench_add_arrays_tch, bench_add_vectors_with_opencl
}

fn batch_add_arrays_ndarray(c: &mut Criterion) {
  c.bench_function("add_arrays_ndarray_batch", |b| b.iter(|| add_arrays_ndarray_batch()));
}

fn batch_add_arrays_rayon(c: &mut Criterion) {
  c.bench_function("add_arrays_rayon_batch", |b| b.iter(|| add_arrays_rayon_batch()));
}

fn batch_add_arrays_tch(c: &mut Criterion) {
  c.bench_function("add_arrays_tch_batch", |b| b.iter(|| add_arrays_tch_batch()));
}

fn batch_add_vectors_with_opencl(c: &mut Criterion) {
  let first = vec![1.0, 2.0, 3.0];
	let second = vec![4.0, 5.0, 6.0];

  c.bench_function("add_vectors_with_opencl_batch", |b| b.iter(|| add_vectors_with_opencl_batch(&first, &second)));
}

criterion_group!{
  name = batches;
  config = Criterion::default().sample_size(10);
  targets = 
    batch_add_arrays_ndarray, batch_add_arrays_rayon, 
    batch_add_arrays_tch, batch_add_vectors_with_opencl
}

criterion_main!(
  benches,
  batches
);