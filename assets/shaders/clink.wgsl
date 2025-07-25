#import bevy_pbr::forward_io::VertexOutput
#import bevy_pbr::mesh_view_bindings::{globals, view};

@group(2) @binding(0) var<uniform> base_color: vec4<f32>;
@group(2) @binding(1) var<uniform> edge_color: vec4<f32>;

const speed = 1.2;

#import "shaders/helpers.wgsl"::{PI}

@fragment
fn fragment(
    mesh: VertexOutput,
) -> @location(0) vec4<f32> {
    // Coordinate relative to middle
    let centered = 2 * (mesh.uv - 0.5);
    let angle = atan2(centered.x, centered.y);
    let range = length(centered);

    let time = speed * globals.time;

    let trig = pow(
        ((1 - abs(
            sin(angle + time)
        )) + (1 - abs(
            cos(angle + time)
        ))) * 0.5,
        2.0
    );

    let dist = 1 / range;

    let st = pow(abs(sin(3 * time)), 15.0);
    let field = st * (6 * pow(trig, 2.0) + 0.08) * dist - 0.5;
    let col_t = clamp(field, 0.0, 1.0);
    let color = (1 - col_t) * edge_color.xyz + (col_t) * base_color.xyz;

    return vec4(color, step(0.0, field));
}


