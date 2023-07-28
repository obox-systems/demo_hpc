// @group(0)
// @binding(0)
// var<storage, read_write> v_indices: array<u32>; // this is used as both input and output for convenience

// fn add_two_vec(a: ptr<function, array<u32, 1000000>>, b: ptr<function,array<u32, 1000000>>) -> array<u32, 1000000> {
//   var res: array<u32, 1000000>;
//   var j = 0;
//   for(var j = 0; j<100; j++) {
//     for(var i = 0; i<10000; i++) {
//       res[i + j * 10000] = (*a)[i + j * 10000] + (*b)[i + j * 10000];
//     }
//   }
//   return res;
// }

// @compute
// @workgroup_size(1)
// fn add_two_vec_call() {
//   var a: array<u32, 1000000>;
//   var b: array<u32, 1000000>;
//   var j = 0;
//   for(var j = 0; j<100; j++) {
//     for(var i = 0; i<10000; i++) {
//       a[i + j * 10000] = v_indices[i + j * 10000];
//       b[i + j * 10000] = v_indices[i + j * 10000];
//     }  
//   }

//   var c = add_two_vec(&a, &b);
// }

@group(0) @binding(0) var<storage, read> input_a: array<f32>;
@group(0) @binding(1) var<storage, read> input_b: array<f32>;
@group(0) @binding(2) var<storage, read_write> output: array<f32>;

// Must use same workgroup size as CUDA for fair comparison!
@compute @workgroup_size(256)
fn vectorAddition(@builtin(global_invocation_id) global_id: vec3u) {
  let idx = global_id.x;
  if idx < arrayLength(&output) {
    output[idx] = input_a[idx] + input_b[idx];
  }
}