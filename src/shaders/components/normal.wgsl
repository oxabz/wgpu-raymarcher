fn shape_normal(point: vec3<f32>, index:u32)-> vec3<f32>{
    let shape = shapes[index];
    var ret : vec3<f32>;
    switch(shape.shape_type){
        case 0u:{
            ret = sphere_normal(point, spheres[shape.index]);
        }
        case 1u:{
            ret = cube_normal(point, cuboids[shape.index]);
        }
        default:{
            ret = vec3<f32>(1.0, 0.0, 0.0);
        }
    }
    return ret;
};
