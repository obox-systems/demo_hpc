@group(0) @binding(0) var<storage, read_write> output: array<u32>;
@group(0) @binding(1) var<storage, read> input_a: array<u32>;

fn optimized_vectorSum(global_id: vec3u) {
	let start: u32 = 0u;
	let end: u32 = arrayLength(&input_a) - 1u;

	var stack: array<vec2<u32>, 32>;

	var stackPtr: u32 = 0u;
	stack[stackPtr] = vec2<u32>(start, end);

	if (stackPtr >= global_id.x) {
		let currentRange: vec2<u32> = stack[stackPtr];
		stackPtr = stackPtr - 1u;

		let currentStart: u32 = currentRange.x;
		let currentEnd: u32 = currentRange.y;

		if (currentStart == currentEnd) {
			output[0] += input_a[currentStart];
		} else if (currentStart < currentEnd) {

			let mid: u32 = (currentStart + currentEnd) / 2u;

			stackPtr = stackPtr + 1u;
			stack[stackPtr] = vec2<u32>(currentStart, mid);

			stackPtr = stackPtr + 1u;
			stack[stackPtr] = vec2<u32>(mid + 1u, currentEnd);
		}
	}
}

@compute @workgroup_size(256)
fn optimized_vectorSum_call(@builtin(global_invocation_id) global_id: vec3u) {
  optimized_vectorSum(global_id);
}

@compute @workgroup_size(256)
fn batch1000_optimized_vectorSum_call(@builtin(global_invocation_id) global_id: vec3u) {
	for (var i: u32 = 0u; i < 1000u; i = i + 1u) {
    optimized_vectorSum(global_id);
  }
}

@compute @workgroup_size(256)
fn batch100000_optimized_vectorSum_call(@builtin(global_invocation_id) global_id: vec3u) {
	for (var i: u32 = 0u; i < 100000u; i = i + 1u) {
    optimized_vectorSum(global_id);
  }
}