#import bevy_pbr::forward_io::VertexOutput
#import bevy_pbr::mesh_view_bindings::{globals, view};

@group(2) @binding(0) var<uniform> base_color: vec4<f32>;
@group(2) @binding(1) var<uniform> edge_color: vec4<f32>;
@group(2) @binding(2) var<uniform> pack: vec4f;


#import "shaders/helpers.wgsl"::{PI, easeOutQuint}

@fragment
fn fragment(
    mesh: VertexOutput,
) -> @location(0) vec4<f32> {
    let duration = pack.x;
    let ring_thickness = pack.y;
    // Coordinate relative to middle
    let centered = 2 * (mesh.uv - 0.5);
    let time = easeOutQuint((globals.time % duration) / duration);

    let normdist = length(centered);

    let half_ring = ring_thickness / 2.0;
    let ring_midpoint = normdist + half_ring;

    let t = abs(time - ring_midpoint);
    var alpha = 0.0;
    if t < half_ring {
        let distance_fade = 1 - time;
        // Technically, this should be 1.0, but the 1.2 makes for a less sharp
        // falloff and the mid color is more pronounced
        let sdf_fade = 1.2 - (t / half_ring);
        alpha = distance_fade * sdf_fade;
    } else {
        alpha = 0.0;
    }
    var color = mix(edge_color, base_color, t / half_ring);
    color.a *= alpha;
    return color;
}
