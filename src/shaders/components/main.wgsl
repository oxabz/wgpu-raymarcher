@stage(compute) @workgroup_size(16,16)
fn render(@builtin(global_invocation_id) global_invocation_id: vec3<u32>){
    let target_size = textureDimensions(target_texture);
    let x = global_invocation_id.x;
    let y = global_invocation_id.y;
    let width = f32(target_size[0]);
    let height = f32(target_size[1]);

    let step_cap = 1000000u;
    let render_distance = 100.0;
    let shadow_blur = 5.0;
    let hit_threshold = 0.00001;
    let background_color = vec3<f32>(0.005, 0.0, 0.03);
    let light_direction = normalize(vec3<f32>(-1.0, -1.0, 0.4));
    let reflection_rays = 10u;
    let reflection_threshold = 0.000001;

    let shape_count = 5u;

    let depth = 2.0;
    var ray_direction = normalize(vec3<f32>((-f32(x) / width + 0.5) * camera.ratio, (-f32(y) / height + 0.5), camera.depth) * camera.ray_dir);

    var ray : RayParams;
    ray.max_length = render_distance;
    ray.max_step = step_cap;
    ray.threshold = hit_threshold;
    ray.skip_shape = -1;

    var color: vec3<f32> = vec3<f32>(0.0,0.0,0.0);
    var color_weight:f32 = 1.0;
    var latest_hit:Hit;
    latest_hit.hit_pos = camera.position;
    latest_hit.hit_shape = -1;
    var bounce_count = 0u;
    loop {
        if (bounce_count >= reflection_rays || color_weight<reflection_threshold){
            color = color * (1.0/(1.0-color_weight));
            break;
        }
        ray.skip_shape = -1;// latest_hit.hit_shape;
        latest_hit = send_ray(latest_hit.hit_pos, ray_direction, ray);
        if (latest_hit.hit_shape < 0){
            color += background_color * color_weight;
            break;
        }
        var s = shapes[latest_hit.hit_shape];
        var surface_info = shape_surface(latest_hit.hit_pos, u32(latest_hit.root_shape));
        var matcolor = surface_info.color;
        let reflectivity = surface_info.reflectivity;
        let matness = 1.0 - reflectivity;



        let normal = surface_info.normal;//shape_normal(latest_hit.hit_pos,u32(latest_hit.hit_shape));
        let diffuse = vcos(normal, -light_direction);
        matcolor = matcolor * diffuse ;
        // Applying mat lighting
        if (diffuse>0.00001){
            var light_ray : RayParams;
            light_ray.max_length = 2000.0;
            light_ray.max_step = 200u;
            light_ray.threshold = 0.0000001;
            light_ray.skip_shape = -1;
            let light_hit = send_ray(latest_hit.hit_pos, -light_direction, light_ray);
            matcolor = matcolor * max(0.0,-f32(light_hit.hit_shape));
        };

        //Specular lighting
        let light_reflection = reflection(light_direction, normal);
        let specular = reflectivity*pow(abs(vcos(light_reflection, ray_direction)),45.0)*max(0.0,diffuse);

        color+=vec3<f32>(specular,specular,specular);

        color += matcolor * color_weight * matness;
        color_weight = color_weight * reflectivity;
        ray_direction = reflection(ray_direction, normal);
        bounce_count += 1u;
    }
    textureStore(target_texture, vec2<i32>(i32(x),i32(y)), vec4<f32>(color,1.0));
};