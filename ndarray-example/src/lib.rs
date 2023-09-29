use ndarray::Array1;

pub fn add_arrays_ndarray(a: &Array1<f64>, b: &Array1<f64>) -> Array1<f64> {
  a + b
}

pub fn add_arrays_ndarray_batch(a: &Array1<f64>, b: &Array1<f64>, batch_number: u32) {
  for _ in 0..batch_number {   
    add_arrays_ndarray(&a, &b);
  }
}

pub fn sum_array_ndarray(arr: &Array1<f64>) -> f64 {
  arr.sum()
}

pub fn sum_array_ndarray_batch(arr: &Array1<f64>, batch_number: u32) {
  for _ in 0..batch_number { 
    sum_array_ndarray(arr);
  }
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

pub fn optimized_array_ndarray_batch(arr: &Array1<f64>, batch_number: u32) {
  for _ in 0..batch_number { 
    optimized_array_ndarray(arr, 0, arr.len() - 1);
  }
}