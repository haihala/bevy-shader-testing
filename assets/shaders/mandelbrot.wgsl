#import bevy_pbr::forward_io::VertexOutput;
#import bevy_pbr::mesh_view_bindings::{globals, view};

#import "shaders/helpers.wgsl"::{PI};

const cycle_duration = 4.0;
const active_duration = cycle_duration;

const max_iters = 50.0;
const background_color = vec4(0.01, 0.02, 0.15, 0.1);
const edge_color = vec4(1.0, 1.0, 0.0, 1.0);

@fragment
fn fragment(
    mesh: VertexOutput,
) -> @location(0) vec4<f32> {
    let scale = 1.2;
    let offset = vec2(-0.55, 0.0);
    let coord = scale*(norm_coord(mesh.uv) + offset);
    let bound = sqrt(2.0)*scale;

    let phase = sin(PI * cycle());
    let loops = i32(phase * max_iters);
    // Mandelbrot iteration fc(z) = z^2 + c, where c is the original coordinate
    var z = vec2(0.0);
    var iters = -1;
    for (var i = 0; i<loops; i++){
        // Imaginary squaring
        z = vec2(
            z.x * z.x - z.y * z.y,
            2 * z.x * z.y,
        ) + coord;

        if length(z-offset) > bound {
            iters = i;
            break;
        }
    }


    var color1 = vec4(0.0);
    if iters == -1 {
        // In the shape
        return vec4(0.0, 0.0, 0.0, 1.0);
    }
    let t = pow(f32(iters) / f32(loops), 2.0);
    return mix(background_color, edge_color, t);
}

fn norm_coord(uv: vec2f) -> vec2f {
    return (uv-0.5)*vec2(2.0, -2.0);
}

fn cycle() -> f32 {
    let time = globals.time + 10000;
    return min(1.0, fract(time / cycle_duration) * cycle_duration / active_duration);
}
