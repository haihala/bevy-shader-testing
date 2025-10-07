#import bevy_pbr::forward_io::VertexOutput;
#import bevy_pbr::mesh_view_bindings::{globals, view};

#import "shaders/helpers.wgsl"::{PI};

@fragment
fn fragment(
    mesh: VertexOutput,
) -> @location(0) vec4<f32> {
    let time = globals.time;
    let centered = 2 * (mesh.uv - 0.5);
    let angle = atan2(centered.x, centered.y);
    let dist = length(centered);

    let flower = 1 - step(sin(angle * 12 + time), 0.98);
    let clock = pow(fract(time), 4.0);
    let focal_distance = 1 - (clock * 2);
    let ring = smoothstep(0.0, 1.0, pow(1 - abs(dist - focal_distance), 2.0));
    let core_mask = smoothstep(0.0, 0.1, dist);
    let outer_mask = 1 - step(1.0, dist);

    let field = smoothstep(0.1, 1.0, flower * ring * core_mask * outer_mask);

    return vec4(field);
}

