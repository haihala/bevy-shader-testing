#import bevy_pbr::forward_io::{VertexOutput, Vertex};
#import bevy_pbr::mesh_view_bindings::{globals, view};
#import bevy_pbr::mesh_functions::{get_world_from_local, mesh_position_local_to_clip}

#import "shaders/helpers.wgsl"::{PI}

const cycle_duration = 3.0;
const speed = 1.0;

@vertex
fn vertex(vertex: Vertex) -> VertexOutput {
    let mirror = i32(globals.time) % 2;
    var out: VertexOutput;

    out.position = mesh_position_local_to_clip(
        get_world_from_local(vertex.instance_index),
        vec4<f32>(vertex.position, 1.0),
    );

    out.world_normal = vertex.normal;

    out.position.x *= abs(sin(globals.time));

    out.uv = vertex.uv;
    out.instance_index = vertex.instance_index;
    return out;
}

@fragment
fn fragment(input: VertexOutput) -> @location(0) vec4<f32> {
    return vec4(input.uv, 0.0, 1.0);
}

