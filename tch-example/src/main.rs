use tch::Tensor;
use tch_example::*;

fn main() {
  let a = Tensor::of_slice(&[1.0, 2.0, 3.0]);
	let b = Tensor::of_slice(&[4.0, 5.0, 6.0]);
	
	let result = add_arrays_tch(&a, &b);
	println!("pytorch result {:?}", result);

	let c = Tensor::of_slice(&[1.0, 2.0, 3.0]);
	println!("Sum (tch-rs): {}", sum_array_tch(&c));

	let c = Tensor::of_slice(&[1.0, 2.0, 3.0]);
	println!("Optimized sum (tch-rs): {}", optimized_array_tch(&c, 0, (c.size1().unwrap() - 1) as usize));
}
