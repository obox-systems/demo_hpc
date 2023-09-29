use criterion::{criterion_group, criterion_main, Criterion};
use ndarray::Array1;

use ndarray_example::*;
use ndarray_rand::{rand, rand_distr::Uniform, RandomExt};

fn bench_add_arrays_ndarray(c: &mut Criterion) {
  let mut rng = rand::thread_rng();
  let uniform = Uniform::new(0.0, 100.0);

  let a = Array1::random_using(1000, uniform, &mut rng);
  let b1 = Array1::random_using(1000, uniform, &mut rng);

  c.bench_function("add_arrays_ndarray_one", |b| b.iter(|| add_arrays_ndarray(&a, &b1)));
}

fn batch1000_add_arrays_ndarray(c: &mut Criterion) {
  let mut rng = rand::thread_rng();
  let uniform = Uniform::new(0.0, 100.0);

  let a = Array1::random_using(1000, uniform, &mut rng);
  let b1 = Array1::random_using(1000, uniform, &mut rng);

  c.bench_function("add_arrays_ndarray_batch_1000", |b| b.iter(|| add_arrays_ndarray_batch(&a, &b1, 1000)));
}

fn batch1000000_add_arrays_ndarray(c: &mut Criterion) {
  let mut rng = rand::thread_rng();
  let uniform = Uniform::new(0.0, 100.0);

  let a = Array1::random_using(1000, uniform, &mut rng);
  let b1 = Array1::random_using(1000, uniform, &mut rng);

  c.bench_function("add_arrays_ndarray_batch_1000", |b| b.iter(|| add_arrays_ndarray_batch(&a, &b1, 1000000)));
}

fn dry_run_add_arrays_ndarray(c: &mut Criterion) {
  let a = Array1::from(vec![]);
  let b1 = Array1::from(vec![]);

  c.bench_function("dry_run_add_arrays_ndarray", |b| b.iter(|| add_arrays_ndarray(&a, &b1)));
}

fn bench_sum_arrays_ndarray(c: &mut Criterion) {
  let mut rng = rand::thread_rng();
  let uniform = Uniform::new(0.0, 100.0);

  let a = Array1::random_using(1000, uniform, &mut rng);

  c.bench_function("add_arrays_ndarray_one", |b| b.iter(|| sum_array_ndarray(&a)));
}

fn batch1000_sum_arrays_ndarray(c: &mut Criterion) {
  let mut rng = rand::thread_rng();
  let uniform = Uniform::new(0.0, 100.0);

  let a = Array1::random_using(1000, uniform, &mut rng);

  c.bench_function("add_arrays_ndarray_batch_1000", |b| b.iter(|| sum_array_ndarray_batch(&a, 1000)));
}

fn batch1000000_sum_arrays_ndarray(c: &mut Criterion) {
  let mut rng = rand::thread_rng();
  let uniform = Uniform::new(0.0, 100.0);

  let a = Array1::random_using(1000, uniform, &mut rng);

  c.bench_function("add_arrays_ndarray_batch_1000000", |b| b.iter(|| sum_array_ndarray_batch(&a, 1000000)));
}

fn dry_run_sum_arrays_ndarray(c: &mut Criterion) {
  let a = Array1::from(vec![]);

  c.bench_function("dry_run_sum_arrays_ndarray", |b| b.iter(|| sum_array_ndarray(&a)));
}

fn bench_optimized_sum_arrays_ndarray(c: &mut Criterion) {
  let mut rng = rand::thread_rng();
  let uniform = Uniform::new(0.0, 100.0);

  let a = Array1::random_using(1000, uniform, &mut rng);

  c.bench_function("optimized_add_arrays_ndarray_one", |b| b.iter(|| optimized_array_ndarray(&a, 0, a.len() - 1)));
}

fn batch1000_optimized_sum_arrays_ndarray(c: &mut Criterion) {
  let mut rng = rand::thread_rng();
  let uniform = Uniform::new(0.0, 100.0);

  let a = Array1::random_using(1000, uniform, &mut rng);

  c.bench_function("optimized_add_arrays_ndarray_batch_1000", |b| b.iter(|| optimized_array_ndarray_batch(&a, 1000)));
}

fn batch1000000_optimized_sum_arrays_ndarray(c: &mut Criterion) {
  let mut rng = rand::thread_rng();
  let uniform = Uniform::new(0.0, 100.0);

  let a = Array1::random_using(1000, uniform, &mut rng);

  c.bench_function("optimized_add_arrays_ndarray_batch_1000000", |b| b.iter(|| optimized_array_ndarray_batch(&a, 1000000)));
}

fn dry_run_optimized_sum_arrays_ndarray(c: &mut Criterion) {
  let a = Array1::from(vec![]);

  c.bench_function("dry_run_optimized_sum_arrays_ndarray", |b| b.iter(|| optimized_array_ndarray(&a, 0, 0)));
}


criterion_group!{
  name = benches;
  config = Criterion::default().sample_size(10);
  targets = 
    bench_add_arrays_ndarray, batch1000_add_arrays_ndarray, batch1000000_add_arrays_ndarray,
    bench_sum_arrays_ndarray, batch1000_sum_arrays_ndarray, batch1000000_sum_arrays_ndarray,
    bench_optimized_sum_arrays_ndarray, batch1000_optimized_sum_arrays_ndarray, batch1000000_optimized_sum_arrays_ndarray,
    dry_run_add_arrays_ndarray, dry_run_sum_arrays_ndarray, dry_run_optimized_sum_arrays_ndarray
}

criterion_main!(
  benches,
);
