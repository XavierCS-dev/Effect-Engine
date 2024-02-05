
struct EntityInput {
    @location(2) position: vec3<f32>,
    @location(3) tex_coords: vec2<f32>,
}

struct VertexInput {
    @location(0) position: vec3<f32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
}

@vertex
fn vrt_main(
    model: VertexInput,
    entity: EntityInput
) -> VertexOutput {
    var out: VertexOutput;
    out.tex_coords = entity.tex_coords;
    out.clip_position = vec4<f32>(model.position + entity.position, 1.0);
    return out;
}

@group(0) @binding(0)
var t_diffuse: texture_2d<f32>;
@group(0) @binding(1)
var s_diffuse: sampler;

@fragment
fn frg_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return textureSample(t_diffuse, s_diffuse, in.tex_coords);
}
