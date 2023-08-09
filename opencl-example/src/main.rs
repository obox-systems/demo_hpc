use opencl_example::add_vectors_with_opencl;

fn main() {
	let a = vec![1.0, 2.0, 3.0];
	let b = vec![4.0, 5.0, 6.0];

	let result = add_vectors_with_opencl(&a, &b, 1);
	println!("opencl result: {:?}", result);
}
