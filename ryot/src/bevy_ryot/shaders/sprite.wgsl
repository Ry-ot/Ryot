#import bevy_sprite::mesh2d_view_bindings::globals
struct SpriteMaterial {
    index: u32,
    counts: vec2<f32>,
    outline_thickness: f32,
    outline_color: vec4<f32>,
};
@group(2) @binding(0)
var<uniform> material: SpriteMaterial;
@group(2) @binding(1)
var texture: texture_2d<f32>;
@group(2) @binding(2)
var texture_sampler: sampler;


fn get_sample(
    probe: vec2<f32>
) -> vec4<f32> {
    return textureSample(texture, texture_sampler, probe);
}

#import bevy_pbr::forward_io::VertexOutput
@fragment
fn fragment(
    in: VertexOutput,
) -> @location(0) vec4<f32> {
    // This assumes the bottom center of the mesh.
    let centered_uv = (in.uv - vec2<f32>(0.25, 0.25));

    let uv = vec2<f32>(centered_uv.x * 2.0, centered_uv.y * 2.0);
    let ux = uv.x / material.counts.x;
    let uy = uv.y / material.counts.y;
    let cuv = vec2(ux, uy);
    let thickness = material.outline_thickness / 1000.;

    let base = pixel(vec2(0., 0.), cuv, vec2(0., 0.));

    var outline_alpha : f32 = 0.;
    outline_alpha += pixel(vec2(0., 0.), cuv, vec2(thickness, 0.)).a;
    outline_alpha += pixel(vec2(0., 0.), cuv, vec2(-thickness, 0.)).a;
    outline_alpha += pixel(vec2(0., 0.), cuv, vec2(0., thickness)).a;
    outline_alpha += pixel(vec2(0., 0.), cuv, vec2(0., -thickness)).a;
    outline_alpha += pixel(vec2(0., 0.), cuv, vec2(thickness, -thickness)).a;
    outline_alpha += pixel(vec2(0., 0.), cuv, vec2(-thickness, thickness)).a;
    outline_alpha += pixel(vec2(0., 0.), cuv, vec2(thickness, thickness)).a;
    outline_alpha += pixel(vec2(0., 0.), cuv, vec2(-thickness, -thickness)).a;
    outline_alpha = min(outline_alpha, 1.0);

    let outline_color = material.outline_color * vec4<f32>(1.0, 1.0, 1.0, outline_alpha);
    let lower = vec2<f32>(0.25, 0.25);
    let upper = vec2<f32>(0.75, 0.75);
    let lower_thickness = lower - vec2<f32>(thickness, thickness);
    let upper_thickness = upper + vec2<f32>(thickness, thickness);

    if (in.uv.x < lower_thickness.x || in.uv.x > upper_thickness.x) {
        discard;
    }

    if (in.uv.y < lower_thickness.y || in.uv.y > upper_thickness.y) {
        discard;
    }

    if (in.uv.x < lower.x || in.uv.x > upper.x) {
        return outline_color;
    }

    if (in.uv.y < lower.y || in.uv.y > upper.y) {
        return outline_color;
    }

    return mix(base, outline_color, outline_alpha - base.a);
}

fn pixel(offset: vec2<f32>, uv: vec2<f32>, adjustment: vec2<f32>) -> vec4<f32> {
    let base_uv = uv_offset(offset, vec2<f32>(0, 0));
    // Use a small inset to avoid sampling the borders
    let inset = 0.001;

    // Calculate inset boundaries for the sprite to avoid texture bleeding
    let sprite_size = vec2<f32>(1.0 / material.counts.x, 1.0 / material.counts.y);
    let min_uv = base_uv + vec2<f32>(inset) * sprite_size;
    let max_uv = base_uv + sprite_size - vec2<f32>(inset) * sprite_size;

    // Calculate adjusted UV coordinates
    let uv_adjusted = uv_offset(offset, uv) + adjustment;

    // Clamp the adjusted UV coordinates within the inset boundaries of the sprite
    let clamped_uv = clamp(uv_adjusted, min_uv, max_uv);

    // Sample the texture using the clamped UV coordinates
    return get_sample(clamped_uv);
}

fn uv_offset(offset: vec2<f32>, uv: vec2<f32>) -> vec2<f32> {
    let uvx = uv.x + (f32(material.index % u32(material.counts.x)) + offset.x) / material.counts.x;
    let uvy = uv.y + (f32(material.index / u32(material.counts.y)) + offset.y) / material.counts.y;
    return vec2(uvx, uvy);
}
