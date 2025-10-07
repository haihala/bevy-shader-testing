#import bevy_pbr::forward_io::VertexOutput;
#import bevy_pbr::mesh_view_bindings::globals;

// This started off with chatGPT. I wanted to see how it would do, and the
// overall result was a bit mixed. It generated a hash and noise which is to be
// expected, but I'm not sure which algorithms it used. It just seems like it
// did some number mushing to end up with something that kinda works.

// I had to spend quite a bit of time tweaking The main function for it to not
// look like a lava lamp. Ultimately I'm not sure it saved time, as the
// approach is pretty much what I would've gone with.


// Pseudo-random 2D noise function
fn hash(p: vec2f) -> f32 {
    let h = dot(p, vec2f(127.1, 311.7));
    return fract(sin(h) * 43758.5453123);
}

// 2D noise based on interpolation of hash values
fn noise(p: vec2f) -> f32 {
    let i = floor(p);
    let f = fract(p);

    let a = hash(i);
    let b = hash(i + vec2f(1.0, 0.0));
    let c = hash(i + vec2f(0.0, 1.0));
    let d = hash(i + vec2f(1.0, 1.0));

    let u = f * f * (3.0 - 2.0 * f);

    return mix(a, b, u.x) + (c - a) * u.y * (1.0 - u.x) + (d - b) * u.x * u.y;
}

// Fire color ramp
fn fireColor(t: f32) -> vec3f {
    let c1 = vec3f(0.1, 0.0, 0.0);   // Dark red
    let c2 = vec3f(1.0, 0.2, 0.0);   // Orange
    let c3 = vec3f(1.0, 1.0, 0.0);   // Yellow
    let c4 = vec3f(1.0, 1.0, 1.0);   // White

    if (t < 0.3) {
        return mix(c1, c2, t / 0.3);
    } else if (t < 0.6) {
        return mix(c2, c3, (t - 0.3) / 0.3);
    } else {
        return mix(c3, c4, (t - 0.6) / 0.4);
    }
}

@fragment
fn fragment(
    mesh: VertexOutput,
) -> @location(0) vec4f {
    let time = globals.time * 1.0;

    let base = mesh.uv * vec2f(8.0, 3.0) + vec2(0.0, time*0.5);

    var intensity = 0.0;
    for (var i = 1.0; i < 7.0; i += 1.0) {
        intensity += noise(base * pow(i,0.8) + vec2f(0.0, time * 3.0)) / i;
    }

    intensity = pow(intensity * 1.5, 2.0); // Increase contrast
    intensity *= 0.3*mesh.uv.y; // Fade out at the top

    // Make it taper out near the sides
    let middle = vec2(2.0, 1.0)*(mesh.uv - 0.5);
    intensity *= max(0.0, 1-(length(middle)));

    if intensity > 0.01 {
        let color = fireColor(intensity);
        return vec4f(color, 10*intensity);
    }
    return vec4f(0.0);
}

