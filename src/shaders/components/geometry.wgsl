fn vcos(a:vec3<f32>, b:vec3<f32>) -> f32{
    return dot(a,b) /(length(a)*length(b));
};

fn reflection(incoming:vec3<f32>, normal:vec3<f32>)->vec3<f32>{
    return -2.0*dot(incoming,normal)/dot(normal,normal)*normal+incoming;
};