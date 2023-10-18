use criterion::{criterion_group, criterion_main, Criterion};
use rand::Rng;
use tch::Tensor;
use tch_example::*;

fn bench_add_arrays_tch(c: &mut Criterion) {
  let mut rng = rand::thread_rng();
  let mut first = vec![0.0; 1000];
	let mut second = vec![0.0; 1000];

  for j in 0..1000 {
    first[j] = rng.gen_range(1.0..=100.0);
    second[j] = rng.gen_range(1.0..=100.0);
  }

	let first1 = Tensor::of_slice(&first);
	let second1 = Tensor::of_slice(&second);

  c.bench_function("add_arrays_tch_one", |b| b.iter(|| add_arrays_tch(&first1, &second1)));
}

fn batch1000_add_arrays_tch(c: &mut Criterion) {
  let mut rng = rand::thread_rng();
  let mut first = vec![0.0; 1000];
	let mut second = vec![0.0; 1000];

  for j in 0..1000 {
    first[j] = rng.gen_range(1.0..=100.0);
    second[j] = rng.gen_range(1.0..=100.0);
  }

	let first1 = Tensor::of_slice(&first);
	let second1 = Tensor::of_slice(&second);

  c.bench_function("batch1000_add_arrays_tch", |b| b.iter(|| batch_add_arrays_tch(&first1, &second1, 1000)));
}

fn batch1000000_add_arrays_tch(c: &mut Criterion) {
  let mut rng = rand::thread_rng();
  let mut first = vec![0.0; 1000];
	let mut second = vec![0.0; 1000];

  for j in 0..1000 {
    first[j] = rng.gen_range(1.0..=100.0);
    second[j] = rng.gen_range(1.0..=100.0);
  }

	let first1 = Tensor::of_slice(&first);
	let second1 = Tensor::of_slice(&second);

  c.bench_function("batch1000000_add_arrays_tch", |b| b.iter(|| batch_add_arrays_tch(&first1, &second1, 1000000)));
}

fn dry_run_add_arrays_tch(c: &mut Criterion) {
  let first = vec![0.0];
	let second = vec![0.0];

	let first1 = Tensor::of_slice(&first);
	let second1 = Tensor::of_slice(&second);

  c.bench_function("dry_run_add_arrays_tch", |b| b.iter(|| batch_add_arrays_tch(&first1, &second1, 1)));
}

fn bench_sum_arrays_tch(c: &mut Criterion) {
	let mut rng = rand::thread_rng();
  let mut first = vec![0.0; 1000];

  for j in 0..1000 {
    first[j] = rng.gen_range(1.0..=100.0);
  }

	let first1 = Tensor::of_slice(&first);

  c.bench_function("sum_arrays_tch_one", |b| b.iter(|| sum_array_tch(&first1)));
}

fn batch1000_sum_arrays_tch(c: &mut Criterion) {
  let mut rng = rand::thread_rng();
  let mut first = vec![0.0; 1000];

  for j in 0..1000 {
    first[j] = rng.gen_range(1.0..=100.0);
  }

	let first1 = Tensor::of_slice(&first);

  c.bench_function("batch1000_sum_arrays_tch", |b| b.iter(|| batch_sum_array_tch(&first1, 1000)));
}

fn batch1000000_sum_arrays_tch(c: &mut Criterion) {
  let mut rng = rand::thread_rng();
  let mut first = vec![0.0; 1000];

  for j in 0..1000 {
    first[j] = rng.gen_range(1.0..=100.0);
  }

	let first1 = Tensor::of_slice(&first);

  c.bench_function("batch1000000_sum_arrays_tch", |b| b.iter(|| batch_sum_array_tch(&first1, 1000000)));
}

fn dry_run_sum_arrays_tch(c: &mut Criterion) {
  let first = vec![0.0; 1000];

	let first1 = Tensor::of_slice(&first);

  c.bench_function("dry_run_sum_arrays_tch", |b| b.iter(|| batch_sum_array_tch(&first1, 1)));
}

fn bench_optimized_sum_arrays_tch(c: &mut Criterion) {
	let mut rng = rand::thread_rng();
  let mut first = vec![0.0; 1000];

  for j in 0..1000 {
    first[j] = rng.gen_range(1.0..=100.0);
  }

	let first1 = Tensor::of_slice(&first);

  c.bench_function("optimized_sum_arrays_tch_one", |b| b.iter(|| optimized_array_tch(&first1, 0 , (first1.size1().unwrap() - 1) as usize)));
}

fn batch1000_optimized_sum_arrays_tch(c: &mut Criterion) {
  let mut rng = rand::thread_rng();
  let mut first = vec![0.0; 1000];

  for j in 0..1000 {
    first[j] = rng.gen_range(1.0..=100.0);
  }

	let first1 = Tensor::of_slice(&first);

  c.bench_function("batch1000_optimized_sum_arrays_tch", |b| b.iter(|| batch_optimized_sum_array_tch(&first1, 0, (first1.size1().unwrap() - 1) as usize, 1000)));
}

fn batch1000000_optimized_sum_arrays_tch(c: &mut Criterion) {
  let mut rng = rand::thread_rng();
  let mut first = vec![0.0; 1000];

  for j in 0..1000 {
    first[j] = rng.gen_range(1.0..=100.0);
  }

	let first1 = Tensor::of_slice(&first);

  c.bench_function("batch1000000_optimized_sum_arrays_tch", |b| b.iter(|| batch_optimized_sum_array_tch(&first1, 0, (first1.size1().unwrap() - 1) as usize, 10000)));
}

fn dry_run_optimized_sum_arrays_tch(c: &mut Criterion) {
  let first = vec![0.0];

	let first1 = Tensor::of_slice(&first);

  c.bench_function("dry_run_optimized_sum_arrays_tch", |b| b.iter(|| batch_optimized_sum_array_tch(&first1, 0, (first1.size1().unwrap() - 1) as usize, 1)));
}

criterion_group!{
  name = benches;
  config = Criterion::default().sample_size(10);
  targets = 
    bench_add_arrays_tch, batch1000_add_arrays_tch, batch1000000_add_arrays_tch,
    bench_sum_arrays_tch, batch1000_sum_arrays_tch, batch1000000_sum_arrays_tch,
    bench_optimized_sum_arrays_tch, batch1000_optimized_sum_arrays_tch, batch1000000_optimized_sum_arrays_tch,
    dry_run_add_arrays_tch, dry_run_sum_arrays_tch, dry_run_optimized_sum_arrays_tch
}

criterion_main!(
  benches,
);