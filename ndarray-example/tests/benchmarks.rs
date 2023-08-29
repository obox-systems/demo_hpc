use criterion::{criterion_group, criterion_main, Criterion};
use ndarray::Array1;

use ndarray_example::*;
use ndarray_rand::{rand, rand_distr::Uniform, RandomExt};

fn bench_add_arrays_ndarray(c: &mut Criterion) {
  let first = Array1::from(vec![1.0, 2.0, 3.0]);
	let second = Array1::from(vec![4.0, 5.0, 6.0]);

  c.bench_function("add_arrays_ndarray_one", |b| b.iter(|| add_arrays_ndarray(&first, &second)));
}

fn batch_add_arrays_ndarray(c: &mut Criterion) {
  let mut rng = rand::thread_rng();
  let uniform = Uniform::new(0.0, 100.0);

  let a = Array1::random_using(10000, uniform, &mut rng);
  let b1 = Array1::random_using(10000, uniform, &mut rng);

  c.bench_function("add_arrays_ndarray_batch", |b| b.iter(|| add_arrays_ndarray(&a, &b1)));
}

fn bench_sum_arrays_ndarray(c: &mut Criterion) {
  let first = Array1::from(vec![1.0, 2.0, 3.0]);

  c.bench_function("add_arrays_ndarray_one", |b| b.iter(|| sum_array_ndarray(&first)));
}

fn batch_sum_arrays_ndarray(c: &mut Criterion) {
  let mut rng = rand::thread_rng();
  let uniform = Uniform::new(0.0, 100.0);

  let a = Array1::random_using(1000000, uniform, &mut rng);

  c.bench_function("add_arrays_ndarray_batch", |b| b.iter(|| sum_array_ndarray(&a)));
}

fn bench_optimized_sum_arrays_ndarray(c: &mut Criterion) {
  let first = Array1::from(vec![1.0, 2.0, 3.0]);

  c.bench_function("optimized_add_arrays_ndarray_one", |b| b.iter(|| optimized_array_ndarray(&first, 0, first.len() - 1)));
}

fn batch_optimized_sum_arrays_ndarray(c: &mut Criterion) {
  let mut rng = rand::thread_rng();
  let uniform = Uniform::new(0.0, 100.0);

  let a = Array1::random_using(10000, uniform, &mut rng);

  c.bench_function("optimized_add_arrays_ndarray_batch", |b| b.iter(|| optimized_array_ndarray(&a, 0, a.len() - 1)));
}

criterion_group!{
  name = benches;
  config = Criterion::default().sample_size(10);
  targets = 
    bench_add_arrays_ndarray, batch_add_arrays_ndarray,
    bench_sum_arrays_ndarray, batch_sum_arrays_ndarray,
    bench_optimized_sum_arrays_ndarray, batch_optimized_sum_arrays_ndarray
}

criterion_main!(
  benches,
);
