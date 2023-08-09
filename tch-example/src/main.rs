use tch::Tensor;
use tch_example::add_arrays_tch;

fn main() {
  let a = Tensor::of_slice(&[1.0, 2.0, 3.0]);
	let b = Tensor::of_slice(&[4.0, 5.0, 6.0]);
	
	let result = add_arrays_tch(&a, &b);
	println!("pytorch result {:?}", result);
}
