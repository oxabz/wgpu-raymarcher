// Vertex shader

struct VertexInput {
    @location(0) position: vec2<f32>;
};

struct VertexOutput {
    @builtin(position) position: vec4<f32>;
};

@stage(vertex)
fn vs_main(
    model: VertexInput,
) -> VertexOutput {
    var out: VertexOutput;
    out.position = vec4<f32>(model.position, 0.0, 1.0);
    return out;
}

// Fragment shader
@group(0) @binding(0)
var t_diffuse: texture_2d<f32>;
@group(0) @binding(1)
var s_diffuse: sampler;

@stage(fragment)
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let x = in.position[0];
    let y = in.position[1];
    return textureSample(t_diffuse, s_diffuse, vec2<f32>(x,y) );
}
