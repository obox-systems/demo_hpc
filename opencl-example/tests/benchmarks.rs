use criterion::{criterion_group, criterion_main, Criterion};

use opencl_example::*;

fn bench_add_vectors_with_opencl(c: &mut Criterion) {
	let first = vec![1.0, 2.0, 3.0];
	let second = vec![4.0, 5.0, 6.0];

  c.bench_function("add_vectors_with_opencl_one", |b| b.iter(|| add_vectors_with_opencl(&first, &second, 1)));
}

fn batch_add_vectors_with_opencl(c: &mut Criterion) {
	let first = vec![1.0, 2.0, 3.0];
	let second = vec![4.0, 5.0, 6.0];

  c.bench_function("batch_add_vectors_with_opencl", |b| b.iter(|| add_vectors_with_opencl(&first, &second, 1000000)));
}

fn bench_sum_vectors_with_opencl(c: &mut Criterion) {
	let first = vec![1.0, 2.0, 3.0];

  c.bench_function("add_vectors_with_opencl_one", |b| b.iter(|| sum_array_opencl(&first, 1)));
}

fn batch_sum_vectors_with_opencl(c: &mut Criterion) {
	let first = vec![1.0, 2.0, 3.0];

  c.bench_function("batch_add_vectors_with_opencl", |b| b.iter(|| sum_array_opencl(&first, 10000)));
}

criterion_group!{
  name = benches;
  config = Criterion::default().sample_size(10);
  targets = 
    bench_add_vectors_with_opencl, batch_add_vectors_with_opencl,
    bench_sum_vectors_with_opencl, batch_sum_vectors_with_opencl
}

criterion_main!(
  benches,
);
