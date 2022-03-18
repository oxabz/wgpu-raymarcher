fn cube_distance(a:vec3<f32>, b:Cuboid)->f32{
    let a_centered = a-b.pos;
    let a_rotated = a_centered*b.rotation;
    let half_size = b.scale/2.0;
    let offset = abs(a_rotated)-half_size;
    var sign = 1.0;
    if offset[0]<0.0 && offset[1]<0.0 && offset[2]<0.0{
        return -length(offset);
    }else{
        return length(max(offset, vec3<f32>(0.0,0.0,0.0)));
    }
};

fn sphere_distance(a: vec3<f32>, b:Sphere)->f32{
    return distance(a,b.pos) - b.radius;
};
