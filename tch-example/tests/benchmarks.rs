use criterion::{criterion_group, criterion_main, Criterion};
use tch::Tensor;
use tch_example::*;

fn bench_add_arrays_tch(c: &mut Criterion) {
	let first = Tensor::of_slice(&[1.0, 2.0, 3.0]);
	let second = Tensor::of_slice(&[4.0, 5.0, 6.0]);

  c.bench_function("add_arrays_tch_one", |b| b.iter(|| add_arrays_tch(&first, &second)));
}

fn batch_add_arrays_tch(c: &mut Criterion) {
  c.bench_function("add_arrays_tch_batch", |b| b.iter(|| add_arrays_tch_batch()));
}

criterion_group!{
  name = benches;
  config = Criterion::default().sample_size(10);
  targets = 
  bench_add_arrays_tch, batch_add_arrays_tch
}

criterion_main!(
  benches,
);