fn cube_normal(a:vec3<f32>, b:Cuboid)->vec3<f32>{
    let a_centered = a-b.pos;
    let a_rotated = a_centered*b.rotation;
    let half_size = b.scale/2.0;

    let a_scaled =  a_rotated/half_size;
    var magnet = vec3<f32>(0.0,0.0,0.0);
    var distmag = 9999999.0;

    let right = vec3<f32>(1.0,0.0,0.0);
    let left = vec3<f32>(-1.0,0.0,0.0);
    let forw = vec3<f32>(0.0,1.0,0.0);
    let back = vec3<f32>(0.0,-1.0,0.0);
    let up = vec3<f32>(0.0,0.0,1.0);
    let down = vec3<f32>(0.0,0.0,-1.0);
    var d = 0.0;
    d = distance(a_scaled,right);
    if(distmag > d){
        magnet = right;
        distmag = d;
    }
    d = distance(a_scaled,left);
    if(distmag > d){
        magnet = left;
        distmag = d;
    }
    d = distance(a_scaled,forw);
    if(distmag > d){
        magnet = forw;
        distmag = d;
    }
    d = distance(a_scaled,back);
    if(distmag > d){
        magnet = back;
        distmag = d;
    }
    d = distance(a_scaled,up);
    if(distmag > d){
        magnet = up;
        distmag = d;
    }
    d = distance(a_scaled,down);
    if(distmag > d){
        magnet = down;
        distmag = d;
    }
    return magnet;
};

fn sphere_normal(point: vec3<f32>, sphere:Sphere)->vec3<f32>{
    if distance(point, sphere.pos) >= sphere.radius{
        return normalize(point - sphere.pos);
    }else{
        return normalize(sphere.pos - point);
    }
};