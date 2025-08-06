#import bevy_pbr::forward_io::VertexOutput
#import bevy_pbr::mesh_view_bindings::{globals, view};

#import "shaders/helpers.wgsl"::{PI, point_in_tri}

// Note to reader:
// If you are planning on using this on a particle effect or similar, you probably shouldn't
// Raindrops are actually mostly round and rain effects are better done in other ways
// This is just a demonstration / study of making a shape geometrically

const cycle_duration = 3.0;
const active_duration = cycle_duration;

@fragment
fn fragment(
    mesh: VertexOutput,
) -> @location(0) vec4<f32> {
    let coord = norm_coord(mesh.uv);
    let t = cycle() * 2 * PI;

    let focal = vec2(0.0);
    let radius = 0.3 + 0.1 * sin(t);
    let tip_dist = radius + (1+cos(t)) * 0.2 + 0.1;
    let tip = focal + tip_dist * vec2(cos(3*t), sin(3*t));

    let in_shape = circle_drop(
        coord,
        tip,
        focal,
        radius,
    );
 
    return vec4(f32(i32(in_shape)));
}

fn circle_drop(coord: vec2f, tip: vec2f, focal: vec2f, radius: f32) -> bool {
    let in_sphere = length(coord - focal) < radius;

    let hypotenuse = length(focal - tip);
    let offset_ang = asin(radius / hypotenuse);
    let tan_len = hypotenuse * cos(offset_ang);

    let throughline = focal-tip;
    let base_ang = atan2(throughline.y, throughline.x);
    let ang1 = base_ang - offset_ang;
    let ang2 = base_ang + offset_ang;
    let tan1 = tan_len * vec2(cos(ang1), sin(ang1)) + tip;
    let tan2 = tan_len * vec2(cos(ang2), sin(ang2)) + tip;

    let in_tri = point_in_tri(coord, tan1, tan2, tip);
    return in_tri || in_sphere;
}

fn norm_coord(uv: vec2f) -> vec2f {
    return (uv-0.5)*vec2(2.0, -2.0);
}

fn cycle() -> f32 {
    let time = globals.time + 10000;
    return min(1.0, fract(time / cycle_duration) * cycle_duration / active_duration);
}

