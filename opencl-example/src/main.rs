use opencl_example::*;

fn main() {
	let a = vec![1.0, 2.0, 3.0];
	let b = vec![4.0, 5.0, 6.0];

	let result = add_vectors_with_opencl(&a, &b, 1);
	println!("opencl result: {:?}", result);

	let arr = vec![1.0, 2.0, 3.0];
	let sum = sum_array_opencl(&arr, 1);
	println!("Sum (OpenCL): {}", sum);

	let arr = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0];
    let sum = optimized_array_opencl(&arr);
    println!("Optimized sum(OpenCL): {}", sum);
}
