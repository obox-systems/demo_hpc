use criterion::{criterion_group, criterion_main, Criterion};
use rand::Rng;
use tch::Tensor;
use tch_example::*;

fn bench_add_arrays_tch(c: &mut Criterion) {
	let first = Tensor::of_slice(&[1.0, 2.0, 3.0]);
	let second = Tensor::of_slice(&[4.0, 5.0, 6.0]);

  c.bench_function("add_arrays_tch_one", |b| b.iter(|| add_arrays_tch(&first, &second)));
}

fn batch_add_arrays_tch(c: &mut Criterion) {
  let mut rng = rand::thread_rng();
  let mut array1 = vec![0.0; 1000000];
  let mut array2 = vec![0.0; 1000000];

  for j in 0..1000000 {
    array1[j] = rng.gen_range(1.0..=100.0);
    array2[j] = rng.gen_range(1.0..=100.0);
  }

  let a = Tensor::of_slice(&array1);
  let b1 = Tensor::of_slice(&array2);

  c.bench_function("add_arrays_tch_batch", |b| b.iter(|| add_arrays_tch(&a, &b1)));
}

fn bench_sum_arrays_tch(c: &mut Criterion) {
	let first = Tensor::of_slice(&[1.0, 2.0, 3.0]);

  c.bench_function("add_arrays_tch_one", |b| b.iter(|| sum_array_tch(&first)));
}

fn batch_sum_arrays_tch(c: &mut Criterion) {
  let mut rng = rand::thread_rng();
  let mut array1 = vec![0.0; 10000];

  for j in 0..10000 {
    array1[j] = rng.gen_range(1.0..=100.0);
  }

  let a = Tensor::of_slice(&array1);

  c.bench_function("add_arrays_tch_batch", |b| b.iter(|| sum_array_tch(&a)));
}

criterion_group!{
  name = benches;
  config = Criterion::default().sample_size(10);
  targets = 
  bench_add_arrays_tch, batch_add_arrays_tch,
  bench_sum_arrays_tch, batch_sum_arrays_tch
}

criterion_main!(
  benches,
);