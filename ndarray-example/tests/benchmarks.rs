use criterion::{criterion_group, criterion_main, Criterion};
use ndarray::Array1;

use ndarray_example::*;

fn bench_add_arrays_ndarray(c: &mut Criterion) {
  let first = Array1::from(vec![1.0, 2.0, 3.0]);
	let second = Array1::from(vec![4.0, 5.0, 6.0]);

  c.bench_function("add_arrays_ndarray_one", |b| b.iter(|| add_arrays_ndarray(&first, &second)));
}

fn batch_add_arrays_ndarray(c: &mut Criterion) {
  c.bench_function("add_arrays_ndarray_batch", |b| b.iter(|| add_arrays_ndarray_batch()));
}

criterion_group!{
  name = benches;
  config = Criterion::default().sample_size(10);
  targets = 
    bench_add_arrays_ndarray, batch_add_arrays_ndarray
}

criterion_main!(
  benches,
);
