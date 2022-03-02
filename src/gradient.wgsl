@group(0) @binding(0)
var target_texture: texture_storage_2d<rgba8snorm, write>;

@stage(compute) @workgroup_size(64)
fn render(@builtin(global_invocation_id) global_invocation_id: vec3<u32>){
    let target_size = textureDimensions(target_texture);
    let x = global_invocation_id.x;
    let y = global_invocation_id.y;

    let r = f32(x)/f32(target_size.x);
    let b = f32(y)/f32(target_size.y);

    textureStore(target_texture, vec2<i32>(i32(x),i32(y)), vec4<f32>(r,0.,b,1.));
}