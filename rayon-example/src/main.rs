use rayon_example::*;

fn main() {
	let a = vec![1.0, 2.0, 3.0];
	let b = vec![4.0, 5.0, 6.0];

	let result = add_arrays_rayon(&a, &b);
	println!("rayon result {:?}", result);

	let b = vec![1.0, 2.0, 3.0];
	println!("Sum (rayon): {}", sum_array_rayon(&b));
}
