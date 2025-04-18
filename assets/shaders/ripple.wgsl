#import bevy_pbr::forward_io::{Vertex};
#import bevy_pbr::mesh_view_bindings::{globals, view};
#import bevy_pbr::mesh_functions::{get_world_from_local, mesh_position_local_to_clip}

#import "shaders/helpers.wgsl"::{easeInQuint, easeOutQuint, easeInCirc, PI}

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) world_position: vec4<f32>,
    @location(1) world_normal: vec3<f32>,
    @location(2) uv: vec2<f32>,
    @location(3) uv_b: vec2<f32>,
    @location(4) world_tangent: vec4<f32>,
    @location(5) color: vec4<f32>,
    @location(6) @interpolate(flat) instance_index: u32,
    @location(7) height: f32,
}

const duration = 2.0;
const edge_color = vec4(1.0, 0.0, 0.0, 1.0);
const base_color = vec4(0.0, 1.0, 0.0, 1.0);

const max_height = 0.2;
const ring_thickness = 0.6;

@vertex
fn vertex(vertex: Vertex) -> VertexOutput {
    var out: VertexOutput;

    out.position = mesh_position_local_to_clip(
        get_world_from_local(vertex.instance_index),
        vec4<f32>(vertex.position, 1.0),
    );

    out.world_normal = vertex.normal;
    out.instance_index = vertex.instance_index;
    out.uv = vertex.uv;

    // Coordinate relative to middle
    let centered = 2 * (vertex.uv - 0.5);
    //let time = easeOutQuint((globals.time % duration) / duration);
    let time = (globals.time % duration) / duration;
    //let time = 1.0;

    let half_thickness = ring_thickness / 2.0;
    let norm_t = easeOutQuint(time) * (1.0 - half_thickness); // From 0 to 
    let target_dist = abs(length(centered) - norm_t);
    let dist_fade = 1.0-pow((PI / 2.0) * target_dist / half_thickness, 2.0);

    // TODO: This should but doesn't take the scale into account
    out.height = dist_fade * max_height - easeInCirc(time);

    let offset = vertex.normal * out.height;
    out.position.x += offset.x;
    out.position.y += offset.y;
    out.position.z += offset.z;

    return out;
}

@fragment
fn fragment(
    mesh: VertexOutput,
) -> @location(0) vec4<f32> {
    if mesh.height <= 0.01{
        return vec4(0.0);
    }
    return mix(edge_color, base_color, mesh.height / max_height);
}
