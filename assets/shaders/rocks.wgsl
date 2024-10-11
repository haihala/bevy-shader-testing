#import bevy_pbr::forward_io::VertexOutput
#import bevy_pbr::mesh_view_bindings::{globals, view};

const PI = 3.14159265359;
const cycle_duration = 3.0;
const speed = 1.0;

const gravity = 0.05;
const friction = 0.8;
const land_speed_loss = 0.5; // Lose half

const min_angle = PI / 8;
const max_angle = PI * 0.35;

const min_velocity = 0.02;
const max_velocity = 0.03;

const min_size = 0.007;
const max_size = 0.02;

const min_start = 0.0;
const max_start = 0.1;

const border = 0.5;
const border_color = vec4(0.13);
const inner_color = vec4(0.33);

@fragment
fn fragment(
    mesh: VertexOutput,
) -> @location(0) vec4<f32> {
    let time = globals.time + 10000;
    let cycle = min(1.0, speed * fract(time / cycle_duration));
    let coords = (mesh.uv - vec2(0.0, 1.0)) * vec2(1.0, -1.0);

    var field = vec3(0.0);

    for (var i = 0; i < 30; i++) {
        let angle = rand(i, 1.234) * (max_angle - min_angle) + min_angle;
        let size = rand(i, 2.345) * (max_size - min_size) + min_size;
        let velocity = (rand(i, 3.456) * (max_velocity - min_velocity) + min_velocity) / size;
        let start_x = rand(i, 4.567) * (max_start - min_start) + min_start;
        let start_y = rand(i, 5.678) * (max_start - min_start) + min_start;

        let launch = vec2(cos(angle), sin(angle)) * velocity;

        let rock_y = max(launch.y * cycle - gravity * pow(cycle, 2.0) / size + start_y, size);
        // Does it land?
        var rock_x = start_x + launch.x * cycle;

        // Quadratic solving time
        // rock_y = -gravity * cycle^2 + launch.y * cycle + start_y;
        // rock_y = size when cycle = land_t
        // land_t = (-launch.y +- sqrt(launch.y^2 - 4*-gravity*start_y)) / 2*-gravity
        // land_t = (-launch.y +- sqrt(launch.y^2 + 4*gravity*start_y)) / 2*-gravity
        let discriminant = sqrt(pow(launch.y, 2.0) + 4 * gravity * start_y / size);
        let land_t = (-launch.y - discriminant) / (-2 * gravity / size);
        let has_landed = land_t < cycle;
        if has_landed {
            let landing_spot = start_x + launch.x * land_t;

            // Max rock_x when? This ensures rocks don't roll back
            // rock_x = landing_spot + (launch.x*land_land_speed_loss - friction*roll_time) * roll_time
            // rock_x = ls + (lx*lsl - fr*rt) * rt
            // d(rock_x)/d(rt) = lx*lsl - 2*fr*rt = 0
            // = lx*lsl = 2*fr*rt
            // = (lx*lsl)/(2*fr) = rt
            let stop_t = (launch.x * land_speed_loss) / (2 * friction);
            let roll_time = min(cycle - land_t, stop_t);

            let base_land_speed = launch.x * land_speed_loss;
            let land_speed = base_land_speed - friction * roll_time;
            rock_x = landing_spot + land_speed * roll_time;
        }
        let rock_pos = vec2(rock_x, rock_y);
        let dist = length(coords - rock_pos);
        if field.x == 0 {
            field.y += step(abs(dist - size), size * border);
        }
        field.x += step(dist, size);
    }

    if field.x <= 0.0 {
        return vec4(0.0);
    }

    let fade = 1 - pow(cycle, 5.0);
    if field.y > 0.0 {
        return vec4(border_color.xyz, fade);
    }

    return vec4(inner_color.xyz, fade);
}

fn rand(index: i32, mul: f32) -> f32 {
    let x = f32(index) - mul;
    let y = f32(index) + mul;
    return fract(sin(x * 12.9898 + y * 78.233) * 43758.5453);
}
