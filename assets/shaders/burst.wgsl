#import bevy_pbr::forward_io::VertexOutput
#import bevy_pbr::mesh_view_bindings::{globals, view};

const PI = 3.14159265359;
const cycle_duration = 3.0;
const speed = 4.0;

@fragment
fn fragment(
    mesh: VertexOutput,
) -> @location(0) vec4<f32> {
    let time = globals.time + 10000;
    let cycle = min(1.0, speed * fract(time / cycle_duration));
    let centered = mesh.uv - 0.5;

    // Burst
    let time_mask = clamp(1 - 40 * pow(cycle, 5.0), 0.0, 1.0);
    let wave_mask = 1 - abs(length(centered) - 0.7 * sin(ease(cycle) * PI)) / 1.5;
    let angle = atan2(centered.x, -centered.y);
    let beams = pattern(3 * angle + PI / 3 - time) + pattern(5 * angle + time) + pattern(7 * angle);
    let burst_mask = clamp(
        (0.5 - length(centered)) * beams,
        0.0, 1.0
    );
    let center_mask = clamp((1 - abs(1.3 * angle)), 0.0, 1.0);
    let mask = clamp(time_mask * wave_mask * burst_mask * center_mask, 0.0, 1.0);

    let burst = lerp(
        vec3(1.0, 0.0, 0.0),
        vec3(1.0, 0.9, 0.2),
        mask
    );

    // Ring
    let ring_df = pow(1 - max(abs(length(centered) - (0.4 - 5 * pow(cycle, 2.0))), 0.0), 40.0);
    let ring_color = vec3(0.1, 0.3, 1.0);

    let total = lerp(burst, ring_color, ring_df);

    return vec4(total.xyz, ring_df + mask);
    //return vec4(burst, mask);
}

fn pattern(in: f32) -> f32 {
    return pow(sin(in), 8.0);
}

fn ease(in: f32) -> f32 {
    return 1 - pow(1 - in, 5.0);
}

fn lerp(a: vec3<f32>, b: vec3<f32>, t: f32) -> vec3<f32> {
    return a * (1 - t) + b * t;
}

// From https://gist.github.com/munrocket/236ed5ba7e409b8bdf1ff6eca5dcdc39
// MIT License. © Stefan Gustavson, Munrocket
//
fn permute4(x: vec4f) -> vec4f { return ((x * 34. + 1.) * x) % vec4f(289.); }
fn fade2(t: vec2f) -> vec2f { return t * t * t * (t * (t * 6. - 15.) + 10.); }

fn perlinNoise2(P: vec2f) -> f32 {
    var Pi: vec4f = floor(P.xyxy) + vec4f(0., 0., 1., 1.);
    let Pf = fract(P.xyxy) - vec4f(0., 0., 1., 1.);
    Pi = Pi % vec4f(289.); // To avoid truncation effects in permutation
    let ix = Pi.xzxz;
    let iy = Pi.yyww;
    let fx = Pf.xzxz;
    let fy = Pf.yyww;
    let i = permute4(permute4(ix) + iy);
    var gx: vec4f = 2. * fract(i * 0.0243902439) - 1.; // 1/41 = 0.024...
    let gy = abs(gx) - 0.5;
    let tx = floor(gx + 0.5);
    gx = gx - tx;
    var g00: vec2f = vec2f(gx.x, gy.x);
    var g10: vec2f = vec2f(gx.y, gy.y);
    var g01: vec2f = vec2f(gx.z, gy.z);
    var g11: vec2f = vec2f(gx.w, gy.w);
    let norm = 1.79284291400159 - 0.85373472095314 * vec4f(dot(g00, g00), dot(g01, g01), dot(g10, g10), dot(g11, g11));
    g00 = g00 * norm.x;
    g01 = g01 * norm.y;
    g10 = g10 * norm.z;
    g11 = g11 * norm.w;
    let n00 = dot(g00, vec2f(fx.x, fy.x));
    let n10 = dot(g10, vec2f(fx.y, fy.y));
    let n01 = dot(g01, vec2f(fx.z, fy.z));
    let n11 = dot(g11, vec2f(fx.w, fy.w));
    let fade_xy = fade2(Pf.xy);
    let n_x = mix(vec2f(n00, n01), vec2f(n10, n11), vec2f(fade_xy.x));
    let n_xy = mix(n_x.x, n_x.y, fade_xy.y);
    return 2.3 * n_xy;
}
