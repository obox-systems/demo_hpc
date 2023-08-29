@group(0) @binding(0) var<storage, read> arr: array<f32>;
@group(0) @binding(1) var<storage, read> start: u32;
@group(0) @binding(2) var<storage, read> end: u32;
@group(0) @binding(3) var<storage, read_write> output: f32;

fn optimized_vectorSum(arr: ptr<function, array<f32, 10000>>, start: u32, end: u32) -> f32 {
	var left = start;
	var right = end;
  var sum: f32 = 0.0;

  while end - start > 0u {
    if left == right {
			var left_copy = left;
			sum += (*arr)[left_copy];
		} else if right - left == 1u {
			sum += (*arr)[left] + (*arr)[right];
		} else {
			let mid = (right - left) / 2u + left;
			left = mid;
			right = mid + 1u;
		}
  }

  return sum;
}

@compute @workgroup_size(256)
fn optimized_vectorSum_call() {
	var arr_copy: array<f32, 10000>;
	for(var i = 0; i < 10000; i++) {
		arr_copy[i] = arr[i];
	}

  output = optimized_vectorSum(&arr_copy, start, end);
}
