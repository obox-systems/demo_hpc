use criterion::{criterion_group, criterion_main, Criterion};
use rayon_example::*;

fn bench_add_arrays_rayon(c: &mut Criterion) {
	let first = vec![1.0, 2.0, 3.0];
	let second = vec![4.0, 5.0, 6.0];

  c.bench_function("add_arrays_rayon_one", |b| b.iter(|| add_arrays_rayon(&first, &second)));
}

fn batch_add_arrays_rayon(c: &mut Criterion) {
  c.bench_function("add_arrays_rayon_batch", |b| b.iter(|| add_arrays_rayon_batch()));
}

criterion_group!{
  name = benches;
  config = Criterion::default().sample_size(10);
  targets = 
    bench_add_arrays_rayon, batch_add_arrays_rayon
}

criterion_main!(
  benches,
);