use criterion::{criterion_group, criterion_main, Criterion};
use rand::Rng;
use rayon_example::*;

fn bench_add_arrays_rayon(c: &mut Criterion) {
	let first = vec![1.0, 2.0, 3.0];
	let second = vec![4.0, 5.0, 6.0];

  c.bench_function("add_arrays_rayon_one", |b| b.iter(|| add_arrays_rayon(&first, &second)));
}

fn batch_add_arrays_rayon(c: &mut Criterion) {
  let mut rng = rand::thread_rng();
  let mut array1 = vec![0.0; 1000000];
  let mut array2 = vec![0.0; 1000000];

  for j in 0..1000000 {
    array1[j] = rng.gen_range(1.0..=100.0);
    array2[j] = rng.gen_range(1.0..=100.0);
  }

  c.bench_function("add_arrays_rayon_batch", |b| b.iter(|| add_arrays_rayon(&array1, &array2)));
}

fn bench_sum_arrays_rayon(c: &mut Criterion) {
	let first = vec![1.0, 2.0, 3.0];

  c.bench_function("add_arrays_rayon_one", |b| b.iter(|| sum_array_rayon(&first)));
}

fn batch_sum_arrays_rayon(c: &mut Criterion) {
  let mut rng = rand::thread_rng();
  let mut array1 = vec![0.0; 10000];


  for j in 0..10000 {
    array1[j] = rng.gen_range(1.0..=100.0);
  }

  c.bench_function("add_arrays_rayon_batch", |b| b.iter(|| sum_array_rayon(&array1)));
}

fn bench_optimized_sum_arrays_rayon(c: &mut Criterion) {
	let first = vec![1.0, 2.0, 3.0];

  c.bench_function("optimized_add_arrays_rayon_one", |b| b.iter(|| optimized_array_rayon(&first, 0, first.len() - 1)));
}

fn batch_optimized_sum_arrays_rayon(c: &mut Criterion) {
  let mut rng = rand::thread_rng();
  let mut array1 = vec![0.0; 1000000];


  for j in 0..1000000 {
    array1[j] = rng.gen_range(1.0..=100.0);
  }

  c.bench_function("optimized_add_arrays_rayon_batch", |b| b.iter(|| optimized_array_rayon(&array1, 0, array1.len() - 1)));
}

criterion_group!{
  name = benches;
  config = Criterion::default().sample_size(10);
  targets = 
    bench_add_arrays_rayon, batch_add_arrays_rayon,
    bench_sum_arrays_rayon, batch_sum_arrays_rayon,
    bench_optimized_sum_arrays_rayon, batch_optimized_sum_arrays_rayon
}

criterion_main!(
  benches,
);