use rand::Rng;
use rayon::prelude::*;

pub fn add_arrays_rayon(a: &[f64], b: &[f64]) -> Vec<f64> {
	a.par_iter().zip(b.par_iter()).map(|(&x, &y)| x + y).collect()
}

pub fn add_arrays_rayon_batch() {
  let mut rng = rand::thread_rng();
  let mut array1 = [0.0; 10];
  let mut array2 = [0.0; 10];

  for _ in 0..1000000 {
    for j in 0..10 {
      array1[j] = rng.gen_range(1.0..=100.0);
      array2[j] = rng.gen_range(1.0..=100.0);
    }
    
    let _ = add_arrays_rayon(&array1, &array2);
  }
}