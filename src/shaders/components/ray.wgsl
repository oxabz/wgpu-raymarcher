fn send_ray(origin:vec3<f32>, direction:vec3<f32>, params: RayParams)->Hit{
    var res: Hit;
    var step_count = 0u;
    var ray_length = 0.0;
    var closest_shape = -1;
    var root_shape = -1;
    var closest_distance_g = 9999999999.0;
    //Params
    let threshold = params.threshold;
    let max_step = params.max_step;
    let max_length = params.max_length;
    let skip_shape = params.skip_shape;
    res.hit_shape = -1;
    res.root_shape = -1;
    var ray_pos = origin + direction * threshold * 10.0;
    loop {
        var closest_distance : f32 = 9999999999.0;
        closest_shape = -1;
        for(var i:u32 = 0u; i < shape_count.count && threshold < closest_distance; i=i+1u){
            if (i32(i) == skip_shape || shapes[i].visible == 0u){continue;}
            let shape_dist_r = shape_distance(ray_pos, i, skip_shape);
            if(closest_distance > shape_dist_r.distance){
                closest_shape = i32(shape_dist_r.index);
                root_shape = i32(i);
                closest_distance = shape_dist_r.distance;
            }
        }
        ray_pos += direction * closest_distance;
        ray_length += closest_distance;
        step_count += 1u;
        if (closest_distance < closest_distance_g){
            closest_distance_g = closest_distance;
        }

        if !( step_count < max_step
                && threshold < closest_distance
                && ray_length < max_length ){
            break;
        }
    }
    if(threshold > closest_distance){
        res.hit_shape = closest_shape;
        res.root_shape = root_shape;
    }
    res.ray_length = ray_length;
    res.step_count = step_count;
    res.hit_pos = ray_pos;
    res.min_distance = closest_distance_g;
    return res;
};