use rand::Rng;
use tch::Tensor;

pub fn add_arrays_tch(a: &Tensor, b: &Tensor) -> Tensor {
	a + b
}

pub fn add_arrays_tch_batch() {
  let mut rng = rand::thread_rng();
  let mut array1 = [0.0; 10];
  let mut array2 = [0.0; 10];

  for _ in 0..1000000 {
    for j in 0..10 {
      array1[j] = rng.gen_range(1.0..=100.0);
      array2[j] = rng.gen_range(1.0..=100.0);
    }

    let a = Tensor::of_slice(&array1);
    let b = Tensor::of_slice(&array2);
    
    let _ = add_arrays_tch(&a, &b);
  }
}

pub fn sum_array_tch(arr: &Tensor) -> f64 {
  arr.sum(tch::Kind::Float)
    .to_kind(tch::Kind::Float)
    .data()
    .into()
}

pub fn optimized_array_tch(arr: &Tensor, start: usize, end: usize) -> f64 {
  if end == start {
    return arr.get(end as i64).into();
  }
  if end - start == 1 {
    return (arr.get(start as i64) + arr.get(end as i64)).into();
  }
  else {
    return optimized_array_tch(arr, start, (end - start)/2 + start) 
    + optimized_array_tch(arr, (end - start)/2 + start +1, end);
  }
}
