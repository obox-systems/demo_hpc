use criterion::{criterion_group, criterion_main, Criterion};
use rand::Rng;
use rayon_example::*;

fn bench_add_arrays_rayon(c: &mut Criterion) {
    let mut rng = rand::thread_rng();
    let mut first = vec![0.0; 1000];
    let mut second = vec![0.0; 1000];

    for j in 0..1000 {
        first[j] = rng.gen_range(1.0..=100.0);
        second[j] = rng.gen_range(1.0..=100.0);
    }

    c.bench_function("add_arrays_rayon_one", |b| {
        b.iter(|| add_arrays_rayon(&first, &second))
    });
}

fn batch1000_add_arrays_rayon(c: &mut Criterion) {
    let mut rng = rand::thread_rng();
    let mut first = vec![0.0; 1000];
    let mut second = vec![0.0; 1000];

    for j in 0..1000 {
        first[j] = rng.gen_range(1.0..=100.0);
        second[j] = rng.gen_range(1.0..=100.0);
    }

    c.bench_function("batch1000_add_arrays_rayon", |b| {
        b.iter(|| add_arrays_rayon_batch(&first, &second, 1000))
    });
}

fn batch1000000_add_arrays_rayon(c: &mut Criterion) {
    let mut rng = rand::thread_rng();
    let mut first = vec![0.0; 1000];
    let mut second = vec![0.0; 1000];

    for j in 0..1000 {
        first[j] = rng.gen_range(1.0..=100.0);
        second[j] = rng.gen_range(1.0..=100.0);
    }

    c.bench_function("batch1000000_add_arrays_rayon", |b| {
        b.iter(|| add_arrays_rayon_batch(&first, &second, 1000000))
    });
}

fn dry_run_add_arrays_rayon(c: &mut Criterion) {
    let first = vec![0.0];
    let second = vec![0.0];

    c.bench_function("dry_run_add_arrays_rayon", |b| {
        b.iter(|| add_arrays_rayon_batch(&first, &second, 1))
    });
}

fn bench_sum_arrays_rayon(c: &mut Criterion) {
    let mut rng = rand::thread_rng();
    let mut first = vec![0.0; 1000];

    for j in 0..1000 {
        first[j] = rng.gen_range(1.0..=100.0);
    }

    c.bench_function("sum_arrays_rayon_one", |b| {
        b.iter(|| sum_array_rayon(&first))
    });
}

fn batch1000_sum_arrays_rayon(c: &mut Criterion) {
    let mut rng = rand::thread_rng();
    let mut first = vec![0.0; 1000];

    for j in 0..1000 {
        first[j] = rng.gen_range(1.0..=100.0);
    }

    c.bench_function("batch1000_sum_arrays_rayon", |b| {
        b.iter(|| sum_array_rayon_batch(&first, 1000))
    });
}

fn batch1000000_sum_arrays_rayon(c: &mut Criterion) {
    let mut rng = rand::thread_rng();
    let mut first = vec![0.0; 1000];

    for j in 0..1000 {
        first[j] = rng.gen_range(1.0..=100.0);
    }

    c.bench_function("batch1000000_sum_arrays_rayon", |b| {
        b.iter(|| sum_array_rayon_batch(&first, 1000000))
    });
}

fn dry_run_sum_arrays_rayon(c: &mut Criterion) {
    let first = vec![0.0];

    c.bench_function("dry_run_sum_arrays_rayon", |b| {
        b.iter(|| sum_array_rayon_batch(&first, 1))
    });
}

fn bench_optimized_sum_arrays_rayon(c: &mut Criterion) {
    let mut rng = rand::thread_rng();
    let mut first = vec![0.0; 1000];

    for j in 0..1000 {
        first[j] = rng.gen_range(1.0..=100.0);
    }

    c.bench_function("optimized_sum_arrays_rayon_one", |b| {
        b.iter(|| optimized_array_rayon(&first, 0, first.len() - 1))
    });
}

fn batch1000_optimized_sum_arrays_rayon(c: &mut Criterion) {
    let mut rng = rand::thread_rng();
    let mut first = vec![0.0; 1000];

    for j in 0..1000 {
        first[j] = rng.gen_range(1.0..=100.0);
    }

    c.bench_function("batch1000_optimized_sum_arrays_rayon", |b| {
        b.iter(|| optimized_sum_array_rayon_batch(&first, 0, first.len() - 1, 1000))
    });
}

fn batch1000000_optimized_sum_arrays_rayon(c: &mut Criterion) {
    let mut rng = rand::thread_rng();
    let mut first = vec![0.0; 1000];

    for j in 0..1000 {
        first[j] = rng.gen_range(1.0..=100.0);
    }

    c.bench_function("batch1000000_optimized_sum_arrays_rayon", |b| {
        b.iter(|| optimized_sum_array_rayon_batch(&first, 0, first.len() - 1, 1000000))
    });
}

fn dry_run_optimized_sum_arrays_rayon(c: &mut Criterion) {
    let first = vec![0.0];

    c.bench_function("dry_run_optimized_sum_arrays_rayon", |b| {
        b.iter(|| optimized_sum_array_rayon_batch(&first, 0, first.len() - 1, 1))
    });
}

criterion_group! {
  name = benches;
  config = Criterion::default().sample_size(10);
  targets =
    bench_add_arrays_rayon, batch1000_add_arrays_rayon, batch1000000_add_arrays_rayon,
    bench_sum_arrays_rayon, batch1000_sum_arrays_rayon, batch1000000_sum_arrays_rayon,
    bench_optimized_sum_arrays_rayon, batch1000_optimized_sum_arrays_rayon, batch1000000_optimized_sum_arrays_rayon,
    dry_run_add_arrays_rayon, dry_run_sum_arrays_rayon, dry_run_optimized_sum_arrays_rayon
}

criterion_main!(benches,);
