#import bevy_pbr::forward_io::VertexOutput;
#import bevy_pbr::mesh_view_bindings::{globals, view};

#import "shaders/helpers.wgsl"::{PI, rand11, remap, easeOutCirc};

const cycle_duration = 3.0;
const active_duration = 2.0;

const particle_count = 20;
const particle_base_size = 0.1;
const particle_velocity = 4.0;
const particle_gravity = 0.5;

@group(#{MATERIAL_BIND_GROUP}) @binding(0) var<uniform> effect: vec4u;   // Ah web

@fragment
fn fragment(
    mesh: VertexOutput,
) -> @location(0) vec4<f32> {
    let coord = norm_coord(mesh.uv);

    var out = vec4(0.0);

    for (var i = 0; i < particle_count; i++) {
        let seed = rand11(f32(i)+cycle_count());
        let part = particle(coord, seed);

        if !part.hit {
            continue;
        }

        if effect.x == 0 {
            out = max(ring(part.uv), out);
        } else if effect.x == 1 {
            out = max(heart(part.uv, seed), out);
        }

    }

    return out;
}

fn ring(uv: vec2f) -> vec4f {
    // Smooth ring
    let half_width = 0.35;
    let radius = 1.0 - half_width;

    let coord = norm_coord(uv);
    let dist = abs(length(coord) - radius);

    var alpha = 0.0;
    if dist < half_width {
        let signed_dist = remap(dist, 0.0, half_width, 0.0, 1.0);

        alpha = clamp(
            1-signed_dist,
            0.0, 
            1.0,
        );
    }

    return vec4(0.1, 0.6, 0.9, alpha);
}

fn self_dot(input: vec2f) -> f32 {
    return dot(input, input);
}

fn heart(uv: vec2f, seed: f32) -> vec4f {
    let t = cycle();
    // Once again, thanks to the goat https://iquilezles.org/articles/distfunctions2d/
    let start_angle = remap(rand11(seed*69), 0.0, 1.0, -1.0, 1.0) * PI / 4.0;
    let angle_vel = remap(rand11(start_angle * 9), 0.0, 1.0, -1.0, 1.0) * PI;
    let angle = start_angle + t * angle_vel;

    let rot = mat2x2(
        vec2(cos(angle), sin(angle)),
        vec2(-sin(angle), cos(angle)),
    );
    
    var p = rot * norm_coord(uv);

    p.x = abs(p.x);
    p.y = 0.5-p.y;

    var heart_sdf = 0.0;
    if( p.y+p.x>1.0 ) {
        heart_sdf = sqrt(self_dot(p-vec2(0.25,0.75))) - sqrt(2.0)/4.0;
    } else {
        heart_sdf = sqrt(min(self_dot(p-vec2(0.00,1.00)), self_dot(p-0.5*max(p.x+p.y,0.0)))) * sign(p.x-p.y);
    }
    
    if heart_sdf < 0.0 {
        return vec4(1.0, 0.0, 0.2, min(20*t, 1.0));
    }

    return vec4(0.0);
}

struct ParticleHit {
    hit: bool,
    uv: vec2f,
}

fn particle(coord: vec2f, seed: f32) -> ParticleHit {
    let t = cycle();
    let goal_time = remap(rand11(seed), 0.0, 1.0, 0.6, 1.0);

    let start_ang = rand11(seed*999) * 2 * PI;
    let start_dist = remap(rand11(start_ang*987), 0.0, 1.0, 0.35, 0.5);

    let start_pos = vec2(cos(start_ang), sin(start_ang)) * start_dist;

    let velocity_bias = start_pos * 2;
    let v0x = rand11(start_ang);
    let v0y = rand11(v0x);
    let v0 = (vec2(v0x, v0y) - 0.5) * particle_velocity + velocity_bias;

    let gravity = -(start_pos + v0*goal_time)/pow(goal_time, 2.0);
    let pos = start_pos + v0 * t + gravity * t * t;

    var out: ParticleHit;

    let ease = easeOutCirc(max(0.0, goal_time-t));
    let particle_size = particle_base_size * ease;

    var left = pos.x - particle_size;
    var right = pos.x + particle_size;
    var top = pos.y - particle_size;
    var bottom = pos.y + particle_size;

    var uv = vec2(
        remap(coord.x, left, right, 0.0, 1.0),
        remap(coord.y, top, bottom, 0.0, 1.0),
    );

    let in_area = uv.x >= 0.0 && uv.y >= 0.0 && uv.x <= 1.0 && uv.y <= 1.0;

    if in_area {
        out.hit = true;
        out.uv = uv;
    }

    return out;
}

fn norm_coord(uv: vec2f) -> vec2f {
    return (uv-0.5)*vec2(2.0, -2.0);
}

fn cycle_count() -> f32 {
    let time = globals.time + 10000;
    return floor(time / cycle_duration);
}

fn cycle() -> f32 {
    let time = globals.time + 10000;
    return min(1.0, fract(time / cycle_duration) * cycle_duration / active_duration);
}


