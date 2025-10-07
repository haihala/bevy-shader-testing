#import bevy_pbr::forward_io::VertexOutput;
#import bevy_pbr::mesh_view_bindings::{globals, view};

struct LFPack {
    speed: f32,
    angle: f32,
    line_thickness: f32,
    layer_count: i32,
}

@group(#{MATERIAL_BIND_GROUP}) @binding(0) var<uniform> base_color: vec4<f32>;
@group(#{MATERIAL_BIND_GROUP}) @binding(1) var<uniform> edge_color: vec4<f32>;
@group(#{MATERIAL_BIND_GROUP}) @binding(2) var<uniform> pack: LFPack;

#import "shaders/helpers.wgsl"::{PI};

@fragment
fn fragment(
    mesh: VertexOutput,
) -> @location(0) vec4<f32> {
    let speed = pack.speed;
    let angle = pack.angle;
    let layer_count = pack.layer_count;
        
    let time = globals.time * speed;
    let wave = cos(time);
    let centered = 2 * (mesh.uv - 0.5);

    var layers = 0.0;
    for (var i = 0; i < layer_count; i++) {
        let t = time * f32(i) + 1.0 / f32(i);
        layers += clamp(layer(t, angle, centered), 0.0, 1.0);
    }
    layers *= wave;

    let falloff = clamp(1.0 - length(centered), 0.0, 1.0);
    let color = mix(base_color, edge_color, clamp(layers, 0.0, 1.0));
    return vec4(color.xyz, layers * falloff);
}

fn layer(time: f32, ang: f32, coords: vec2<f32>) -> f32 {
    let line_thickness = pack.line_thickness;
    // Creates the lines that go along the wanted direction
    let angled = (coords.x * cos(ang) + coords.y * sin(ang));
    let stripes = step(abs(sin(angled / line_thickness)), 0.5);

    // Produces a rolling effect
    let coangled = coords.x * cos(ang - PI / 2) + coords.y * sin(ang - PI / 2);
    let roller = sin(coangled + 2 * PI * fract(time) - PI / 2);

    // Masks out some lines
    let lanes = step(abs(sin((5 + floor(time) % 3) * angled + 10 * floor(time))), line_thickness);
    return stripes * lanes * roller;
}

