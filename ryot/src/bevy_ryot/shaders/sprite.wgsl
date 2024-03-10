#import bevy_sprite::mesh2d_view_bindings::globals
struct SpriteMaterial {
    index: u32,
    counts: vec2<f32>,
};
@group(2) @binding(0)
var<uniform> material: SpriteMaterial;
@group(2) @binding(1)
var texture: texture_2d<f32>;
@group(2) @binding(2)
var texture_sampler: sampler;

#import bevy_pbr::forward_io::VertexOutput
@fragment
fn fragment(
    in: VertexOutput,
) -> @location(0) vec4<f32> {
    let uv = in.uv;
    let ux = uv.x / material.counts.x;
    let uy = uv.y / material.counts.y;
    let cuv = vec2(ux, uy);

    let base = pixel(vec2(0.,0.), cuv);
    return base;
}

fn pixel(offset: vec2<f32>, uv: vec2<f32>) -> vec4<f32> {
    let uv_adjusted = uv_offset(offset, uv);
    return textureSample(texture, texture_sampler, uv_adjusted);
}

fn uv_offset(offset: vec2<f32>, uv: vec2<f32>) -> vec2<f32> {
    let uvx = uv.x + (f32(material.index % u32(material.counts.x)) + offset.x) / material.counts.x;
    let uvy = uv.y + (f32(material.index / u32(material.counts.y)) + offset.y) / material.counts.y;
    return vec2(uvx, uvy);
}
