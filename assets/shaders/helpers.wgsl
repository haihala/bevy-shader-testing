const PI = 3.14159265359;
const TAU = PI*2.0;

fn inverse_lerp(floor: f32, ceil: f32, val: f32) -> f32 {
    return (val - floor) / (ceil - floor);
}

// val assumed to be in range [curr_a, curr_b], wanted to be in range [new_a, new_b]
fn remap(val: f32, curr_a: f32, curr_b: f32, new_a: f32, new_b: f32) -> f32 {
    let norm = inverse_lerp(curr_a, curr_b, val);
    return norm * (new_b - new_a) + new_a;
}

fn signed_distance_from_line(point: vec2f, start: vec2f, end: vec2f) -> f32 {
    let line = end - start;
    let offset = point - start;
    return (line.x * offset.y - line.y * offset.x) / length(line);
}

fn project_to_line(point: vec2f, start: vec2f, end: vec2f) -> f32 {
    let rel_point = point-start;
    let line = end-start;   // Line we are projecting on
    return dot(rel_point, line) / dot(line, line); // How far along the line we are
}

// Easing functions from https://easings.net
// Did type them out by hand, they may have problems

fn easeInSine(x: f32) -> f32 {
    return 1.0 - cos(x * PI / 2.0);
}
fn easeOutSine(x: f32) -> f32 {
    return sin(x * PI / 2.0);
}
fn easeInOutSine(x: f32) -> f32 {
    return -(cos(PI*x) - 1.0) / 2.0;
}

fn easeInCubic(x: f32) -> f32 {
    return pow(x, 3.0);
}
fn easeOutCubic(x: f32) -> f32 {
    return 1.0 - pow(1.0-x, 3.0);
}
fn easeInOutCubic(x: f32) -> f32 {
    if (x < 0.5) {
        return 4.0 * pow(x, 3.0);
    } else {
        return 1.0 - pow(-2.0 * x + 2.0, 3.0) / 2.0;
    }
}

fn easeInQuint(x: f32) -> f32{
    return pow(x, 5.0);
}
fn easeOutQuint(x: f32) -> f32 {
    return 1 - pow(1 - x, 5.0);
}
fn easeInOutQuint(x: f32) -> f32 {
    if (x < 0.5) {
        return 16 * pow(x, 5.0);
    } else {
        return 1.0 - pow(-2.0 * x + 2.0, 5.0) / 2.0;
    }
}

fn easeInCirc(x: f32) -> f32 {
    return 1.0 - sqrt(1.0-pow(x, 2.0));
}
fn easeOutCirc(x: f32) -> f32 {
    return sqrt(1.0-pow(x-1, 2.0));
}
fn easeInOutCirc(x: f32) -> f32 {
    if (x < 0.5) {
        return (1-sqrt(1.0 - pow(2.0 * x, 2.0))) / 2.0;
    } else {
        return (sqrt(1.0 - pow(-2.0 * x + 2, 2.0)) + 1.0) / 2.0;
    }
}

fn easeInElastic(x: f32) -> f32 {
    if (x <= 0.0) {
        return 0.0;
    } else if (x >= 1.0) {
        return 1.0;
    } else {
        let c4 = (2.0 * PI) / 3.0;
        return -pow(2.0, 10.0*x-10.0) * sin((x*10.0 - 10.75)*c4);
    }
}
fn easeOutElastic(x: f32) -> f32 {
    if (x <= 0.0) {
        return 0.0;
    } else if (x >= 1.0) {
        return 1.0;
    } else {
        let c4 = (2.0 * PI) / 3.0;
        return pow(2.0, -10.0*x) * sin((x*10.0 - 0.75)*c4) + 1.0;
    }
}
fn easeInOutElastic(x: f32) -> f32 {
    if (x <= 0.0) {
        return 0.0;
    } else if (x >= 1.0) {
        return 1.0;
    } else {
        let c5 = (2.0 * PI) / 4.5;
        if (x < 0.5) {
            return -pow(2.0, 20.0*x - 10.0) * sin((x*20.0 - 11.125)*c5) / 2.0;
        } else {
            return pow(2.0, -20.0*x + 10.0) * sin((x*20.0 - 11.125)*c5) / 2.0 + 1.0;
        }
    }
}

fn easeInQuad(x: f32) -> f32 {
    return pow(x, 2.0);
}
fn easeOutQuad(x: f32) -> f32 {
    return 1.0 - pow(1.0-x, 2.0);
}
fn easeInOutQuad(x: f32) -> f32 {
    if (x < 0.5) {
        return 2 * pow(x, 2.0);
    } else {
        return 1.0 - pow(-2.0 * x + 2.0, 2.0) / 2.0;
    }
}

fn easeInQuart(x: f32) -> f32 {
    return pow(x, 4.0);
}
fn easeOutQuart(x: f32) -> f32 {
    return 1.0 - pow(1.0-x, 4.0);
}
fn easeInOutQuart(x: f32) -> f32 {
    if (x < 0.5) {
        return 8 * pow(x, 4.0);
    } else {
        return 1.0 - pow(-2.0 * x + 2.0, 4.0) / 2.0;
    }
}

fn easeInExpo(x: f32) -> f32 {
    if (x <= 0.0) {
        return 0.0;
    }
    return pow(2.0, 10.0*x-10.0);
}
fn easeOutExpo(x: f32) -> f32 {
    if (x >= 1.0) {
        return 1.0;
    }
    return 1.0 - pow(2.0, -10.0*x);
}
fn easeInOutExpo(x: f32) -> f32 {
    if (x <= 0.0) {
        return 0.0;
    } else if (x >= 1.0) {
        return 1.0;
    } else {
        if (x < 0.5) {
            return pow(2.0, 20.0 * x - 10.0) / 2.0;
        } else {
            return 2.0 - pow(2.0, -20.0 * x + 10.0) / 2.0;
        }
    }
}

fn easeInBack(x: f32) -> f32 {
    let c1 = 1.70158;
    let c3 = c1 + 1.0;
    return c3 * pow(x, 3.0) - c1 * pow(x, 2.0);
}
fn easeOutBack(x: f32) -> f32 {
    let c1 = 1.70158;
    let c3 = c1 + 1.0;
    return 1+c3 * pow(x-1.0, 3.0) + c1 * pow(x-1, 2.0);
}
fn easeInOutBack(x: f32) -> f32 {
    let c1 = 1.70158;
    let c2 = c1 * 1.525;

    if (x < 0.5) {
        return (pow(2.0 * x, 2.0) * ((c2 + 1.0) * 2.0 * x - c2)) / 2.0;
    } else {
        return (pow(2.0 * x - 2.0, 2.0) * ((c2 + 1.0) * (x * 2.0 - 2.0) + c2) + 2.0) / 2.0;
    }
}

// Others are in, out, inout, but the in depends on the out here
// Not sure about this, as it used -= inline, and I'm assuming that evaluates
// To the value after the subtraction
fn easeOutBounce(x: f32) -> f32{
    let n1 = 7.5625;
    let d1 = 2.75;
    if (x < 1 / d1) {
        return n1 * pow(x, 2.0);
    } else if (x < 2 / d1) {
        let nx = x - 1.5;
        return n1 * (nx / d1) * nx + 0.75;
    } else if (x < 2.5 / d1) {
        let nx = x - 2.25;
        return n1 * (nx / d1) * nx + 0.9375;
    } else {
        let nx = x - 2.625;
        return n1 * (nx/ d1) * nx + 0.984375;
    }
}
fn easeInBounce(x: f32) -> f32{
    return 1.0 - easeOutBounce(1.0 - x);
}
fn easeInOutBounce(x: f32) -> f32{
    if (x < 0.5) {
        return (1.0 - easeOutBounce(1.0 - 2.0 * x)) / 2.0;
    } else {
        return (1.0 + easeOutBounce(2.0 * x - 1.0)) / 2.0;
    }
}

// From https://gist.github.com/munrocket/236ed5ba7e409b8bdf1ff6eca5dcdc39
// On generating random numbers, with help of y= [(a+x)sin(bx)] mod 1", W.J.J. Rey, 22nd European Meeting of Statisticians 1998
fn rand11(n: f32) -> f32 { return fract(sin(n) * 43758.5453123); }
fn rand22(n: vec2f) -> f32 { return fract(sin(dot(n, vec2f(12.9898, 4.1414))) * 43758.5453); }

//  <https://www.shadertoy.com/view/Xd23Dh>
//  by Inigo Quilez
//
fn hash23(p: vec2f) -> vec3f {
    let q = vec3f(dot(p, vec2f(127.1, 311.7)),
        dot(p, vec2f(269.5, 183.3)),
        dot(p, vec2f(419.2, 371.9)));
    return fract(sin(q) * 43758.5453);
}

fn voroNoise2(x: vec2f, u: f32, v: f32) -> f32 {
    let p = floor(x);
    let f = fract(x);
    let k = 1. + 63. * pow(1. - v, 4.);
    var va: f32 = 0.;
    var wt: f32 = 0.;
    for(var j: i32 = -2; j <= 2; j = j + 1) {
        for(var i: i32 = -2; i <= 2; i = i + 1) {
            let g = vec2f(f32(i), f32(j));
            let o = hash23(p + g) * vec3f(u, u, 1.);
            let r = g - f + o.xy;
            let d = dot(r, r);
            let ww = pow(1. - smoothstep(0., 1.414, sqrt(d)), k);
            va = va + o.z * ww;
            wt = wt + ww;
        }
    }
    return va / wt;
}

// MIT License. © Stefan Gustavson, Munrocket
//
fn permute4(x: vec4<f32>) -> vec4<f32> { return ((x * 34. + 1.) * x) % vec4<f32>(289.); }
fn fade2(t: vec2<f32>) -> vec2<f32> { return t * t * t * (t * (t * 6. - 15.) + 10.); }

fn perlinNoise2(P: vec2<f32>) -> f32 {
  var Pi: vec4<f32> = floor(P.xyxy) + vec4<f32>(0., 0., 1., 1.);
  let Pf = fract(P.xyxy) - vec4<f32>(0., 0., 1., 1.);
  Pi = Pi % vec4<f32>(289.); // To avoid truncation effects in permutation
  let ix = Pi.xzxz;
  let iy = Pi.yyww;
  let fx = Pf.xzxz;
  let fy = Pf.yyww;
  let i = permute4(permute4(ix) + iy);
  var gx: vec4<f32> = 2. * fract(i * 0.0243902439) - 1.; // 1/41 = 0.024...
  let gy = abs(gx) - 0.5;
  let tx = floor(gx + 0.5);
  gx = gx - tx;
  var g00: vec2<f32> = vec2<f32>(gx.x, gy.x);
  var g10: vec2<f32> = vec2<f32>(gx.y, gy.y);
  var g01: vec2<f32> = vec2<f32>(gx.z, gy.z);
  var g11: vec2<f32> = vec2<f32>(gx.w, gy.w);
  let norm = 1.79284291400159 - 0.85373472095314 *
      vec4<f32>(dot(g00, g00), dot(g01, g01), dot(g10, g10), dot(g11, g11));
  g00 = g00 * norm.x;
  g01 = g01 * norm.y;
  g10 = g10 * norm.z;
  g11 = g11 * norm.w;
  let n00 = dot(g00, vec2<f32>(fx.x, fy.x));
  let n10 = dot(g10, vec2<f32>(fx.y, fy.y));
  let n01 = dot(g01, vec2<f32>(fx.z, fy.z));
  let n11 = dot(g11, vec2<f32>(fx.w, fy.w));
  let fade_xy = fade2(Pf.xy);
  let n_x = mix(vec2<f32>(n00, n01), vec2<f32>(n10, n11), vec2<f32>(fade_xy.x));
  let n_xy = mix(n_x.x, n_x.y, fade_xy.y);
  return 2.3 * n_xy;
}
