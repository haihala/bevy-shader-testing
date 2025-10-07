#import bevy_pbr::forward_io::VertexOutput;
#import bevy_pbr::mesh_view_bindings::{globals, view};

#import "shaders/helpers.wgsl"::{PI, rand11, remap, easeInQuint, point_in_quad, project_to_line};

const cycle_duration = 3.0;
const active_duration = 2.0;

const particle_count = 25;
const particle_base_width = 0.15;
const particle_base_height = 0.07;
const particle_velocity = 5.0;
const particle_gravity = vec2(0.0, -7.0);

const dollar_green = vec4(133.0,187.0,101.0, 255.0) / 255.0;
const dark_green = vec4(50.0,77.0,34.0, 255.0) / 255.0;


// TODO: Not entirely satisfied with how the bill edges look (weird projection to plane)

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

        if length(out) < 0.01 {
            out = bill(part.uv);
        }

    }

    return vec4(out);
}

fn bill(uv: vec2f) -> vec4f {
    let out_of_bounds = uv.x < 0.0 || uv.y < 0.0 || uv.x > 1.0 || uv.y > 1.0;
    if (out_of_bounds) {
        return vec4(0.0);
    }

    let edge = 0.07;
    let on_edge = uv.x < edge || 1.0-uv.x < edge || uv.y < edge || 1.0-uv.y < edge;
    if on_edge {
        return dark_green;
    }

    let centered = (uv-0.5);
    let in_middle = length(centered) < 0.3;
    if in_middle {
        return dark_green;
    }

    return dollar_green;
}

struct ParticleHit {
    hit: bool,
    uv: vec2f,
}

fn particle(coord: vec2f, seed: f32) -> ParticleHit {
    let t = cycle();
    let start_ang = rand11(seed) * 2 * PI;
    let start_dist = rand11(start_ang);

    let start_pos = vec2(cos(start_ang), sin(start_ang)) * start_dist * 0.4;

    let v0x = rand11(seed);
    let v0y = rand11(v0x);
    let v0 = vec2(v0x-0.5, v0y) * particle_velocity;

    let pos = vec3((start_pos + v0 * t + particle_gravity * t * t).xy, 0.0);

    let base_tan = vec3(1.0, 0.0, 0.0);
    let base_bitan = vec3(0.0, 1.0, 0.0);

    let e1 = (rand11(seed)-0.5);
    let e2 = (rand11(e1)-0.5);
    let e3 = (rand11(e2)-0.5);

    let e1s = (rand11(e3)-0.5);
    let e2s = (rand11(e1s)-0.5);
    let e3s = (rand11(e2s)-0.5);

    let init_ang = PI*0.5;
    let rot_speed = 10.0;
    let rot_x = e1*init_ang + t * e1s*rot_speed;
    let rot_y = e2*init_ang + t * e2s*rot_speed;
    let rot_z = e3*init_ang + t * e3s*rot_speed;
    let tangent = euler_rot(base_tan, rot_x, rot_y, rot_z);
    let bitangent = euler_rot(base_bitan, rot_x, rot_y, rot_z);

    var out: ParticleHit;

    let ease = easeInQuint(1-t);
    let particle_height = particle_base_height * ease;
    let particle_width = particle_base_width * ease;
    let bl = project(pos + (-tangent * particle_width - bitangent * particle_height));
    let tl = project(pos + (-tangent * particle_width + bitangent * particle_height));
    let tr = project(pos + (tangent * particle_width + bitangent * particle_height));
    let br = project(pos + (tangent * particle_width - bitangent * particle_height));

    let right_side_include = point_in_quad(coord, bl, tl, tr, br);
    let inverse_include = point_in_quad(coord, br, tr, tl, bl);

    if right_side_include || inverse_include {
        out.hit = true;
        let d1 = clamp(project_to_line(coord, tl, tr), 0.0, 1.0);
        let d2 = clamp(project_to_line(coord, bl, br), 0.0, 1.0);

        out.uv.x = (d1 + d2) * 0.5;

        let d3 = project_to_line(coord, tl, bl);
        let d4 = project_to_line(coord, tr, br);

        out.uv.y = (d3 + d4) * 0.5;
    }

    return out;
}

fn euler_rot(v: vec3f, x: f32, y: f32, z: f32) -> vec3f {
    // Written from https://en.wikipedia.org/wiki/Rotation_matrix
    // Easier to transpose
    let Rz = transpose(mat3x3f(
        vec3(cos(z), -sin(z), 0.0),
        vec3(sin(z), cos(z), 0.0),
        vec3(0.0, 0.0, 1.0),
    ));
    let Ry = transpose(mat3x3f(
        vec3(cos(y), 0.0, sin(y)),
        vec3(0.0, 1.0, 0.0),
        vec3(-sin(y), 0.0, cos(y)),
    ));
    let Rx = transpose(mat3x3f(
        vec3(1.0, 0.0, 0.0),
        vec3(0.0, cos(x), -sin(x)),
        vec3(0.0, sin(x), cos(x)),
    ));

    let R = Rz*Ry*Rx;

    return R*v;
}

fn project(input: vec3f) -> vec2f {
    // Not sure if this is correct, probably isn't
    let xy = input.xy;
    let norm = normalize(xy);
    let scaled_norm = norm * input.z;
    return xy + scaled_norm;
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
    // let time = 1000.0;
    return min(1.0, fract(time / cycle_duration) * cycle_duration / active_duration);
}

