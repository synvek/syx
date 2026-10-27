struct VertexOutput {
    @builtin(position) pos: vec4<f32>,
    @location(0) uv: vec2<f32>,
}

@group(0) @binding(2) var<uniform> u: CompositeUniforms;

struct CompositeUniforms {
    overlay_size: vec2<f32>,
    surface_size: vec2<f32>,
    origin: vec2<f32>,
    bg: vec4<f32>,
}

@vertex
fn vs_main(@builtin(vertex_index) in_vertex_index: u32) -> VertexOutput {
    var pos = array<vec2<f32>, 3>(
        vec2<f32>(-1.0, -1.0),
        vec2<f32>(3.0, -1.0),
        vec2<f32>(-1.0, 3.0),
    );
    var out: VertexOutput;
    out.pos = vec4<f32>(pos[in_vertex_index], 0.0, 1.0);
    let surface = pos[in_vertex_index] * 0.5 + 0.5;
    out.uv = surface * u.surface_size;
    return out;
}

@group(0) @binding(0) var t: texture_2d<f32>;
@group(0) @binding(1) var s: sampler;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let local = in.uv - u.origin;
    if (local.x < 0.0 || local.y < 0.0 || local.x >= u.overlay_size.x || local.y >= u.overlay_size.y) {
        discard;
    }
    let uv = local / u.overlay_size;
    let c = textureSample(t, s, uv);
    if (c.a < 0.01) {
        discard;
    }
    return vec4<f32>(c.rgb, 1.0);
}
