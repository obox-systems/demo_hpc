@group(0) @binding(0) var<storage, read_write> output: array<u32>;
@group(0) @binding(1) var<storage, read> input_a: array<u32>;

fn vectorSum(global_id: vec3u) {
  for (var i: u32 = global_id.x; i < arrayLength(&input_a); i = i + 1u) {
    output[0] += input_a[i];
  }
}

@compute @workgroup_size(256)
fn vectorSum_call(@builtin(global_invocation_id) global_id: vec3u) {
  vectorSum(global_id);
}

@compute @workgroup_size(256)
fn batch1000_vectorSum_call(@builtin(global_invocation_id) global_id: vec3u) {
  for (var i: u32 = 0u; i < 1000u; i = i + 1u) {
    vectorSum(global_id);
  }
}

@compute @workgroup_size(256)
fn batch100000_vectorSum_call(@builtin(global_invocation_id) global_id: vec3u) {
  for (var i: u32 = 0u; i < 10000u; i = i + 1u) {
    vectorSum(global_id);
  }
}