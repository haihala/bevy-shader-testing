#import bevy_pbr::forward_io::VertexOutput
#import bevy_pbr::mesh_view_bindings::{globals, view};

#import "shaders/helpers.wgsl"::{PI, remap, signed_distance_from_line};

const cycle_duration = 3.0;
const speed = 1.0;

@fragment
fn fragment(
    mesh: VertexOutput,
) -> @location(0) vec4<f32> {
    let time = globals.time + 10000;
    let cycle = min(1.0, speed * fract(time / cycle_duration));
    let coord = (mesh.uv - 0.5)*2;
    let angle = remap(
        atan2(coord.x, coord.y),
        -PI,
        PI,
        0.0,
        1.0,
    );
    
    let light_angle = cycle * 2 * PI;
    let light_pos = vec3(cos(light_angle), sin(light_angle), 1.0);

    let skew = 1.4; // Perspective thing. 1.0 is a perfect hexagon, less than that look weird
    let base_len = 0.5;

    let tris = 6;
    let ang_step = 2*PI/f32(tris);

    for (var tri = 0; tri < tris; tri++) {
        let ang1 = f32(tri) * ang_step;
        let ang2 = ang1 + ang_step;

        var len1: f32;
        var len2: f32;
        var norm_ang: f32;
        let first = (tri % 2) == 0;
        if first {
            len1 = skew*base_len;
            len2 = base_len;
            norm_ang = ang2;
        } else {
            len1 = base_len;
            len2 = skew*base_len;
            norm_ang = ang1;
        }

        let edge_point1 = vec2(cos(ang1), sin(ang1)) * len1;
        let edge_point2 = vec2(cos(ang2), sin(ang2)) * len2;


        let in_tri = point_in_triangle(coord, vec2(0.0), edge_point1, edge_point2);

        if !in_tri {
            continue;
        }

        let norm = normalize(vec3(cos(norm_ang), sin(norm_ang), 1.0/sqrt(2.0)));
        let direct_lighting = max(dot(norm, light_pos), 0.0);
        let ligthing = direct_lighting * 0.5 + 0.1;

        return vec4(ligthing);
    }

    return vec4(0.0);
}

fn point_in_triangle(point: vec2f, a: vec2f, b: vec2f, c: vec2f) -> bool {
    // Assume the points are wound clockwise
    if signed_distance_from_line(point, a, b) < 0 {
        return false;
    }
    if signed_distance_from_line(point, b, c) < 0 {
        return false;
    }
    if signed_distance_from_line(point, c, a) < 0 {
        return false;
    }

    return true;
}
