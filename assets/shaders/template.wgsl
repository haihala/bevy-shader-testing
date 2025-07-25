#import bevy_pbr::forward_io::VertexOutput
#import bevy_pbr::mesh_view_bindings::{globals, view};

#import "shaders/helpers.wgsl"::{PI}

const cycle_duration = 5.0;
const active_duration = 1.0;

@fragment
fn fragment(
    mesh: VertexOutput,
) -> @location(0) vec4<f32> {
    let coord = norm_coord(mesh.uv);
    return vec4((cycle() * coord).xy, 0.0, 1.0);
}

fn norm_coord(uv: vec2f) -> vec2f {
    return (uv-0.5)*vec2(2.0, -2.0);
}

fn cycle() -> f32 {
    let time = globals.time + 10000;
    return min(1.0, fract(time / cycle_duration) * cycle_duration / active_duration);
}
