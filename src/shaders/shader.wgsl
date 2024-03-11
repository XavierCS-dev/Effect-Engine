
struct EntityInput {
    @location(2) model_a: vec4<f32>,
    @location(3) model_b: vec4<f32>,
    @location(4) model_c: vec4<f32>,
    @location(5) model_d: vec4<f32>,
    @location(6) index: vec2<f32>,
    @location(7) size: vec2<f32>,
}

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) tex_coords: vec2<f32>,
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
    var model_mat = mat4x4<f32>(
        entity.model_a,
        entity.model_b,
        entity.model_c,
        entity.model_d,  
    );
    out.tex_coords = vec2<f32>(model.tex_coords.x + (entity.index.x * entity.size.x), 
    model.tex_coords.y + (entity.index.y * entity.size.y));
    out.clip_position = model_mat * vec4<f32>(model.position, 1.0);
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
