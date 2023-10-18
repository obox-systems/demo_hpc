@group(0) @binding(0) var<storage, read_write> output: array<u32>;
@group(0) @binding(1) var<storage, read> input_a: array<u32>;
@group(0) @binding(2) var<storage, read> input_b: array<u32>;

fn vectorAddition(global_id: vec3u) {
  let idx = global_id.x;
  if idx < arrayLength(&output) {
    output[idx] = input_a[idx] + input_b[idx];
  }
}

@compute @workgroup_size(256)
fn vectorAddition_call(@builtin(global_invocation_id) global_id: vec3u) {
  vectorAddition(global_id);
}

@compute @workgroup_size(256)
fn batch1000_vectorAddition_call(@builtin(global_invocation_id) global_id: vec3u) {
  for (var i: u32 = 0u; i < 1000u; i = i + 1u) {
    vectorAddition(global_id);
  }
}

@compute @workgroup_size(256)
fn batch100000_vectorAddition_call(@builtin(global_invocation_id) global_id: vec3u) {
  for (var i: u32 = 0u; i < 10000u; i = i + 1u) {
    vectorAddition(global_id);
  }
}
