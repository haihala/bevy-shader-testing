#import bevy_pbr::forward_io::VertexOutput
#import bevy_pbr::mesh_view_bindings::{globals, view};

@group(2) @binding(0) var<uniform> sharpness: f32;

const PI = 3.14159265359;
const offset = PI * 2 / 3;

@fragment
fn fragment(
    mesh: VertexOutput,
) -> @location(0) vec4<f32> {
    var ray = normalize(view.world_position.xyz - mesh.world_position.xyz);
    var norm = dot(mesh.world_normal, ray);
    var alpha = pow(1 - norm, sharpness) * 0.3;
    var color = vec3(0, offset, 2 * offset) + globals.time;
    return vec4(sin(color), alpha);
}
