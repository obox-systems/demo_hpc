@group(0) @binding(0) var<storage, read> input_a: array<f32>;
@group(0) @binding(1) var<storage, read_write> output: f32;

@compute @workgroup_size(256)
fn vectorSum(@builtin(global_invocation_id) global_id: vec3u) {
  let idx = global_id.x;
  if idx < u32(output) {
    output += input_a[idx];
  }
}