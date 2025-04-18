#import bevy_pbr::forward_io::VertexOutput
#import bevy_pbr::mesh_view_bindings::{globals, view};

#import "shaders/helpers.wgsl"::{PI}

@fragment
fn fragment(
    mesh: VertexOutput,
) -> @location(0) vec4<f32> {
    let centered = 2 * (mesh.uv - 0.5);
    let angle = atan2(centered.x, centered.y);
    let dist = length(centered);

    let flower = 1 - step(sin(angle * 10 + globals.time), dist);
    let ring = smoothstep(0.0, 1.0, pow(1 - abs(0.6 - dist), 2.0));

    let field = smoothstep(0.7, 1.0, flower * ring);

    return vec4(field);
}

