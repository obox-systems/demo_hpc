use rayon::prelude::*;

pub fn add_arrays_rayon(a: &[f64], b: &[f64]) -> Vec<f64> {
	a.par_iter().zip(b.par_iter()).map(|(&x, &y)| x + y).collect()
}

pub fn add_arrays_rayon_batch(a: &[f64], b: &[f64], batch: u32) {
  for _ in 0..batch  {
    add_arrays_rayon(a, b);
  }
}

// pub fn add_arrays_rayon_batch() {
//   let mut rng = rand::thread_rng();
//   let mut array1 = [0.0; 10];
//   let mut array2 = [0.0; 10];

//   for _ in 0..1000000 {
//     for j in 0..10 {
//       array1[j] = rng.gen_range(1.0..=100.0);
//       array2[j] = rng.gen_range(1.0..=100.0);
//     }
    
//     let _ = add_arrays_rayon(&array1, &array2);
//   }
// }

pub fn sum_array_rayon(arr: &[f64]) -> f64 {
  arr.par_iter().sum()
}

pub fn sum_array_rayon_batch(a: &[f64], batch: u32) {
  for _ in 0..batch  {
    sum_array_rayon(a);
  }
}

pub fn optimized_array_rayon(arr: &[f64], start: usize, end: usize) -> f64 {
  if end == start {
    return arr[end];
  }
  if end - start == 1 {
    return arr[start] + arr[end];
  }
  else {
    return optimized_array_rayon(arr, start, (end - start)/2 + start) 
    + optimized_array_rayon(arr, (end - start)/2 + start +1, end);
  }
}

pub fn optimized_sum_array_rayon_batch(arr: &[f64], start: usize, end: usize, batch: u32) {
  for _ in 0..batch  {
    optimized_array_rayon(arr, start, end);
  }
}
