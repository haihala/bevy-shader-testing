#import bevy_pbr::forward_io::VertexOutput
#import bevy_pbr::mesh_view_bindings::{globals, view};

#import "shaders/helpers.wgsl"::{PI, perlinNoise2, easeOutQuint, rand11};

const cycle_duration = 3.0;
const active_duration = 2.0;


@fragment
fn fragment(
    mesh: VertexOutput,
) -> @location(0) vec4<f32> {
    let coord = norm_coord(mesh.uv);

    // return particles(coord);
    return particles(coord) + powder(coord);
}


const particle_amount = 30;
const particle_seed = 69;
const particle_gravity = 4.0;
const particle_min_angle = -PI / 6;
const particle_max_angle = PI * 0.2;
const particle_min_velocity = 0.5;
const particle_max_velocity = 1.5;
const particle_size_vel_influence = 0.5;
const particle_deceleration_x = 0.15;
const particle_min_size = 0.015;
const particle_max_size = 0.02;
const particle_shrink_speed = 0.005;
const particle_min_start_x = -0.8;
const particle_max_start_x = 0.2;
const particle_min_start_y = -0.3;
const particle_max_start_y = 0.5;
const particle_inner_color = vec3(0.9, 0.85, 0.85);
const particle_border_color = vec3(0.7, 0.65, 0.65);

fn particles(coord: vec2f) -> vec4f {
    let t = cycle();

    var out = 0.0;

    for (var i = 0; i <  particle_amount; i++) {
        let key = f32(i+particle_seed);
        let size_rand = rand11(key);
        let size = max(0.0, size_rand * (particle_max_size - particle_min_size) + particle_min_size - particle_shrink_speed * t);
        let size_offset = vec2(0.0, size);
        let front_rock = point_in_arc(key, t + 0.01, size_rand) + size_offset;
        let back_rock = point_in_arc(key, t, size_rand) + size_offset;
        let norm = normalize(back_rock - front_rock);

        let front_dist = 0.5 * length(coord - front_rock);
        let back_dist = 0.5 * length(coord - (front_rock + 0.07 * norm * size_rand));
        let dist = front_dist + back_dist;

        out = max(step(dist, size), out);
    }

    if out <= 0.0 {
        return vec4(0.0);
    }

    let color = mix(particle_inner_color, particle_border_color, out);
    return vec4(color.xyz, 1.0);
}

fn point_in_arc(key: f32, t: f32, size_rand: f32) -> vec2f {
    let angle = rand11(key+1.0) * (particle_max_angle - particle_min_angle) + particle_min_angle;
    let size_influence = max(0.0, 1.0 - particle_size_vel_influence * pow(size_rand, 10.0));
    let velocity = (rand11(key+2.0) * (particle_max_velocity - particle_min_velocity) + particle_min_velocity) * size_influence;
    let start_x = rand11(key+3.0) * (particle_max_start_x - particle_min_start_x) + particle_min_start_x;
    let start_y = rand11(key+4.0) * (particle_max_start_y - particle_min_start_y) + particle_min_start_y;

    let launch = vec2(cos(angle), sin(angle)) * velocity;

    let rock_x = start_x + launch.x * t;
    let rock_y = launch.y * t - particle_gravity * pow(t, 2.0) / 2.0 + start_y;

    let rock_pos = vec2(rock_x, rock_y);
    return rock_pos;
}

const noise_layers = 3;

fn powder(coord: vec2f) -> vec4f {
    let t = cycle();
    let time_fade = easeOutQuint(1-t) * easeOutQuint(clamp(t*2.0, 0.0, 1.0));

    let inner_center = vec2(-1.3, -1.0);
    let inner_radius = 1.0;
    let inner_val = length(inner_center - coord) - inner_radius;

    let outer_center = vec2(-0.9, -1.5);
    let outer_radius = 2.3;
    let outer_val = length(outer_center - coord) - outer_radius;

    let dist_center = vec2(-0.9, 0.9);
    let dist_radius = 2.2;
    let dist_val = clamp(dist_radius - length(dist_center - coord), 0.0, 1.0);

    let left_dist = abs(-1 - coord.x);
    let left_mask = clamp(left_dist * 2.0, 0.0, 1.0);

    let boost_oval_p1 = vec2(-0.7, 0.2);
    let boost_oval_p2 = vec2(0.3, -0.5);
    let oval_dist = length(coord - boost_oval_p1) + length(coord - boost_oval_p2);
    let oval_boost = 0.2*clamp(1.0-0.5*oval_dist, 0.0, 1.0);

    let mask = max((-outer_val) * inner_val * dist_val * left_mask + oval_boost, 0.0);

    let shift = vec2(-pow(t, 0.7), pow(t, 2.0)) * 20.0;
    let noise_offsets = array<vec2f, 3>(
        vec2(12.0, 34.0) + shift,
        vec2(45.0, 67.0) + shift,
        vec2(78.0, 89.0) + shift,
    );
    let noise_scales = array<vec2f, 3>(
        vec2(25.0), 
        vec2(40.0),
        vec2(80.0), 
    );
    let noise_weights = array<f32, 3>(
        1.0, 3.0, 2.0
    );

    var noise = 0.0;
    var total_noise = 0.0;
    for (var i = 0; i < noise_layers; i++){
        let weight = noise_weights[i];
        let scale = noise_scales[i];
        let pos = coord * scale + noise_offsets[i];
        noise += perlinNoise2(pos) * weight;
        total_noise += weight;
    }
    noise = clamp(2*noise / total_noise, 0.3, 1.0);

    return vec4(noise * mask * time_fade);
}

fn norm_coord(uv: vec2f) -> vec2f {
    return (uv-0.5)*vec2(2.0, -2.0);
}

fn cycle() -> f32 {
    let time = globals.time + 10000;
    return min(1.0, fract(time / cycle_duration) * cycle_duration / active_duration);
}
