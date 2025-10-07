#import bevy_pbr::forward_io::VertexOutput;
#import bevy_pbr::mesh_view_bindings::{globals, view};

#import "shaders/helpers.wgsl"::{PI, easeInQuint};

const cycle_duration = 1.0;
const cycle_cooldown = 1.0;
const total_duration = cycle_duration + cycle_cooldown;

// Z controls relative thickness
@group(#{MATERIAL_BIND_GROUP}) @binding(0) var<uniform> control_points: array<vec3f, 16>;
@group(#{MATERIAL_BIND_GROUP}) @binding(1) var<uniform> curve_count: vec4u;

const midline_color = vec4(0.165, 0.133, 0.988, 1.0);
const stripe_primary_color = vec4(0.892, 0.624, 1.0, 1.0);
const stripe_secondary_color = vec4(0.9, 0.624, 0.624, 1.0);

const sections_per_curve_per_unit: u32 = 40;

@fragment
fn fragment(
    mesh: VertexOutput,
) -> @location(0) vec4<f32> {
    let time_mode = 0;

    var time: f32 = 0;
    let real_time = globals.time + 10000;
    switch time_mode {
        case 1: {
            time = 10001.11;         // Tight bend right
        }
        default: {
            time = real_time;
        }
    }

    let real_cycle = min(1.0, fract(real_time / cycle_duration) * total_duration);
    let cycle = min(1.0, fract(time / cycle_duration) * total_duration);

    let coord = (mesh.uv - 0.5) * vec2(2.0, -2.0);
    let curve = calc_curve(
        coord, 
        cycle,
        0.3,
    );

    if !curve.on_curve {
        return vec4(0.0);
    }

    return swoosh(curve.uv, real_cycle);
}

fn swoosh(uv: vec2f, t: f32) -> vec4f {
    let coord = (uv-0.5)*2;

    let edge_mask = 1-easeInQuint(abs(coord.x));

    let corner_fade_x = 1-cos(coord.x * PI / 2.0);
    let corner_fade_y = abs(coord.y);
    let corner_mask = 1-(corner_fade_x*corner_fade_y);

    var stripes = 0.0;
    let layers = 3;
    let layer_offset = PI/f32(layers);
    let base_frequency = 0.2;
    let frequency_power_base = 3.0;

    for (var i = 1; i <= layers; i++) {
        let l = f32(i);

        let base_freq = layer_offset * l;
        let main_freq = abs(coord.x) * base_frequency * pow(frequency_power_base, l);
        let t_influence = -t*PI; // Different sign than main freq -> waves go mostly outwards
        let secondary = sin(t_influence) * coord.x * base_frequency * pow(frequency_power_base, l) / 3.0;
        let output = abs(sin(base_freq + main_freq + t_influence + secondary));
        stripes += output / f32(layers);
    }
    let wave_y = cos(abs(coord.y) * PI/2.0);
    let wave_mask = (1-stripes) * wave_y;

    let mask = edge_mask * corner_mask * wave_mask;

    let throughline_weight = easeInQuint(1-abs(coord.x));
    let base_color_mixer = 0.5 + 0.5*sin(4*coord.x*PI);
    let base_color = mix(stripe_primary_color, stripe_secondary_color, base_color_mixer);
    let color = mix(base_color, midline_color , throughline_weight);

    return vec4(mask * color);
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
