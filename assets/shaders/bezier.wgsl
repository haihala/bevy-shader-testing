#import bevy_pbr::forward_io::VertexOutput
#import bevy_pbr::mesh_view_bindings::{globals};

#import "shaders/helpers.wgsl"::{PI};

const total_duration = 3.0;

// Z controls relative thickness
@group(2) @binding(0) var<uniform> control_points: array<vec3f, 16>;
@group(2) @binding(1) var<uniform> curve_count: vec4u;
@group(2) @binding(2) var imageTexture: texture_2d<f32>;
@group(2) @binding(3) var imageSampler: sampler;

const sections_per_curve_per_unit: u32 = 40;

@fragment
fn fragment(
    mesh: VertexOutput,
) -> @location(0) vec4<f32> {
    let time_mode = 0;  // Debugging purposes
    var time: f32 = 0;
    switch time_mode {
        case 1: {
            time = 10000.68;         // Tight bend right
        }
        case 2: {
            time = 9999.8;          // Tight bend left
        }
        case 3: {
            time = 10001.983;       // End
        }
        case 4: {
            time = 10001.7;         // Final straight
        }
        default: {
            time = globals.time + 10000;
        }
    }
    let cycle = min(1.0, fract(time / total_duration));

    let coord = (mesh.uv - 0.5) * vec2(2.0, -2.0);
    let curve = calc_curve(
        coord, 
        // 0.0,1.0,
        cycle,
        0.1,
    );

    if !curve.on_curve {
        return vec4(0.0);
    }

    let texture_mode = i32(floor(time / total_duration) % 6);
    switch texture_mode {
        case 0: {
            return chess(curve.uv);
        }
        case 1: {
            return ball(curve.uv);
        }
        case 2: {
            return hue_sway(curve.uv);
        }
        case 3: {
            return distance_vis(curve.dist);
        }
        case 4: {
            return uv_vis(curve.uv);
        }
        case 5: {
            return image(curve.uv);
        }

        default: {
            return vec4(1.0);
        }
    }
}

fn chess(uv: vec2f) -> vec4f {
    let grid = vec2i(floor(uv * vec2(4.0, 12.0)));
    let even = (grid.x + grid.y) % 2 == 0;
    if even {
        return vec4(1.0);
    } else {
        return vec4(0.0, 0.0, 0.0, 1.0);
    }
}

fn ball(uv: vec2f) -> vec4f {
    let dist = length(2*(uv - 0.5));
    if dist < 1.0{
        return vec4(1.0);
    } else {
        return vec4(0.0);
    }
}

fn hue_sway(uv: vec2f) -> vec4f {
    let left = vec4(1.0);
    let right = vec4(0.0);
    return mix(left, right, uv.x);
}

fn distance_vis(dist: f32) -> vec4f {
    return vec4(dist);
}

fn uv_vis(uv: vec2f) -> vec4f {
    return vec4(uv.xy, 0.0, 1.0);
}

fn image(uv: vec2f) -> vec4f {
    return textureSample(imageTexture, imageSampler, uv);
}

struct CurveHit {
    uv: vec2f,
    dist: f32,
    on_curve: bool,
}

fn calc_curve(coord: vec2f, uv_map_start: f32, uv_map_length: f32) -> CurveHit {
    let curve_length = min(uv_map_length, 1.0 - uv_map_start);
    let section_count = u32(
        max(
            2.0,    // In order to not get rounded ends, we need at least two segments
            f32(sections_per_curve_per_unit) * curve_length * f32(curve_count.x)
        )
    );
    let sections = f32(section_count);
    let section_len = curve_length/sections;

    var output: CurveHit;

    for (var int_i: u32 = 0; int_i < section_count; int_i++) {
        let i = f32(int_i);
        let t_start = uv_map_start + i * section_len;
        let t_end = t_start + section_len;
        let in_first_half = i < (sections/2.0);

        let start_bez = bezier(t_start);
        let end_bez = bezier(t_end);
        let start = start_bez.xy;
        let end = end_bez.xy;

        let projection = project(coord, start, end);
        let thickness = mix(start_bez.z, end_bez.z, projection) * 0.01;    // Multiply for nicer numbers

        // Aight so story time
        // Trying to deduce if a point is on the curve or not was a pain
        //
        // Approach 1: Project to line, if projection is not between start and end, cull it
        // Problem: When curving, there are gaps on the outer edge
        //
        // Approach 2: Distance field
        // Problem: Rounded edges, uvs outside of the [0, 1] range
        //
        // Approach 3 (active): Project + distance to segment end
        // On the first half, allow going over
        // On the last half, allow undershooting
        // This has hard ends and leaves no gaps

        var off_segment_proj_plus = false;
        if in_first_half {
            // Allow going over, but not under
            off_segment_proj_plus = projection < 0.0 || (projection > 1.0 && length(coord-end) > end_bez.z*0.01);
        } else {
            // Allow falling short, but not going over
            off_segment_proj_plus = projection > 1.0 || (projection < 0.0 && length(coord-start) > start_bez.z*0.01);
        }

        if off_segment_proj_plus {
            continue;
        }

        let clamp_proj = clamp(projection, 0.0, 1.0);
        let meeting_point = mix(start, end, clamp_proj);
        let dist = length(meeting_point - coord);

        if dist < thickness {
            let norm_dist = dist/thickness;
            let uvs = vec2(
                (calc_side(coord, start, end) / thickness + 1) / 2,
                (i + clamp_proj) / sections
            );

            if !output.on_curve || output.dist > norm_dist {
                output.uv = uvs;
                output.dist = norm_dist;
            }

            output.on_curve = true;
        }
    }

    return output;
}

fn bezier(full_t: f32) -> vec3f {
    let t_per_set = 1.0 / f32(curve_count.x);
    let set_index = floor(full_t / t_per_set);
    let offset = 3 * i32(set_index);
    let t = (full_t - (t_per_set * set_index)) / t_per_set;

    let a = control_points[offset];
    let b = control_points[offset+1];
    let c = control_points[offset+2];
    let d = control_points[offset+3];

    let a_part = a * (-pow(t, 3.0) + 3.0*pow(t, 2.0) - 3.0*t+1.0);
    let b_part = b * (3.0 * pow(t,3.0) - 6*pow(t, 2.0) + 3*t);
    let c_part = c * (-3.0*pow(t,3.0) + 3*pow(t,2.0));
    let d_part = d * pow(t, 3.0);
    return a_part + b_part + c_part + d_part;
}

// From Sebastian
fn calc_side(point: vec2f, start: vec2f, end: vec2f) -> f32 {
    let line = end - start;
    let offset = point - start;
    return (line.x * offset.y - line.y * offset.x) / length(line);
}

fn project(point: vec2f, start: vec2f, end: vec2f) -> f32 {
    let rel_point = point-start;
    let line = end-start;   // Line we are projecting on
    return dot(rel_point, line) / dot(line, line); // How far along the line we are
}
