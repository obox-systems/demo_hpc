use ndarray::Array1;
use ndarray_rand::{rand, rand_distr::Uniform, RandomExt};

pub fn add_arrays_ndarray(a: &Array1<f64>, b: &Array1<f64>) -> Array1<f64> {
  a + b
}

pub fn add_arrays_ndarray_batch() {
  let mut rng = rand::thread_rng();
  let uniform = Uniform::new(0.0, 100.0);

  for _ in 0..1000000 {
    let a = Array1::random_using(10, uniform, &mut rng);
    let b = Array1::random_using(10, uniform, &mut rng);
    
    let _ = add_arrays_ndarray(&a, &b);
  }
}

pub fn sum_array_ndarray(arr: &Array1<f64>) -> f64 {
  arr.sum()
}

pub fn optimized_array_ndarray(arr: &Array1<f64>, start: usize, end: usize) -> f64 {
  if end == start {
    return arr[end];
  }
  if end - start == 1 {
    return arr[start] + arr[end];
  }
  else {
    return optimized_array_ndarray(arr, start, (end - start)/2 + start) 
    + optimized_array_ndarray(arr, (end - start)/2 + start +1, end);
  }
}
